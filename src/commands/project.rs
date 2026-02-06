use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

use colored::Colorize;
use tabled::Table;

use crate::cli::{DeployProjectParams, ProjectId, ProjectLogs, ProjectRun};
use crate::client::{CanineClient, CanineError, Pod, ProcessStatus};
use crate::config::CanineConfig;
use crate::kubeconfig::{ensure_kubectl, kubeconfig_to_yaml};

pub async fn handle_list(client: &CanineClient) -> Result<(), Box<dyn std::error::Error>> {
    let projects = client.get_projects().await?.projects;
    println!("{}", Table::new(projects));
    Ok(())
}

pub async fn handle_processes(
    client: &CanineClient,
    id: &ProjectId,
) -> Result<(), Box<dyn std::error::Error>> {
    let processes = client.get_processes(&id.project).await?;
    println!("{}", Table::new(processes.pods));
    Ok(())
}

pub async fn handle_run(
    config: &CanineConfig,
    client: &CanineClient,
    params: &ProjectRun,
) -> Result<(), Box<dyn std::error::Error>> {
    gate_kubectl();

    print!("Fetching project {}... ", params.project.cyan());
    io::stdout().flush().unwrap();
    let project = client.get_project(&params.project).await?;
    println!("{}", "done".green());

    print!("Downloading kubeconfig for cluster {}... ", project.cluster_name.cyan());
    io::stdout().flush().unwrap();
    // Save kubeconfig
    let kubeconfig = client
        .download_kubeconfig_file(&project.cluster_name.to_string())
        .await?;
    let yaml = kubeconfig_to_yaml(&kubeconfig.kubeconfig)?;
    config.save_kubeconfig(yaml)?;

    print!("Starting one-off container in {}... ", project.name.cyan());
    io::stdout().flush().unwrap();

    let pod = client.create_one_off_pod(&params.project).await?;
    println!("{}", "done".green());
    println!("  Pod: {}", pod.name.dimmed());

    wait_pod_ready(client, &params.project, &pod.name).await?;

    let mut args = vec![
        "exec".to_string(),
        "-it".to_string(),
        "-n".to_string(),
        pod.namespace,
        pod.name,
        "--".to_string(),
    ];
    args.extend(params.command.clone());

    Command::new("kubectl")
        .args(&args)
        .env(
            "KUBECONFIG",
            CanineConfig::credential_path().to_str().unwrap(),
        )
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    Ok(())
}

pub async fn handle_logs(
    config: &CanineConfig,
    client: &CanineClient,
    params: &ProjectLogs,
) -> Result<(), Box<dyn std::error::Error>> {
    gate_kubectl();

    print!("Fetching project {}... ", params.project.cyan());
    io::stdout().flush().unwrap();
    let project = client.get_project(&params.project).await?;
    println!("{}", "done".green());

    print!(
        "Downloading kubeconfig for cluster {}... ",
        project.cluster_name.cyan()
    );
    io::stdout().flush().unwrap();
    let kubeconfig = client
        .download_kubeconfig_file(&project.cluster_name.to_string())
        .await?;
    let yaml = kubeconfig_to_yaml(&kubeconfig.kubeconfig)?;
    config.save_kubeconfig(yaml)?;
    println!("{}", "done".green());

    let processes = client.get_processes(&params.project).await?;
    let pod = if let Some(ref process_name) = params.process {
        print!("Finding process {}... ", process_name.cyan());
        io::stdout().flush().unwrap();
        let found = processes
            .pods
            .iter()
            .find(|p| p.name.contains(process_name))
            .ok_or_else(|| format!("Process '{}' not found", process_name))?;
        println!("{}", "done".green());
        found
    } else {
        if processes.pods.is_empty() {
            return Err("No processes found for this project".into());
        }
        println!("\nSelect a process:");
        for (i, p) in processes.pods.iter().enumerate() {
            println!("  [{}] {} ({})", (i + 1).to_string().cyan(), p.name, format!("{:?}", p.status).dimmed());
        }
        print!("\n{} ", "Enter number:".bold());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input)?;
        let choice: usize = input.trim().parse().map_err(|_| "Invalid selection")?;
        if choice < 1 || choice > processes.pods.len() {
            return Err(format!("Selection out of range (1-{})", processes.pods.len()).into());
        }
        &processes.pods[choice - 1]
    };

    let mut args = vec!["logs", "-n", &pod.namespace, &pod.name];
    if params.tail {
        args.push("-f");
    }

    Command::new("kubectl")
        .args(&args)
        .env(
            "KUBECONFIG",
            CanineConfig::credential_path().to_str().unwrap(),
        )
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    Ok(())
}

pub async fn handle_deploy(
    client: &CanineClient,
    params: &DeployProjectParams,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = client
        .deploy_project(&params.name, params.skip_build)
        .await?;
    let url = format!(
        "{}/projects/{}/deployments/{}",
        client.base_url, params.name, result.build_id
    );
    println!("{} {}", "✓".green(), result.message);
    println!("  View deployment: {}", url.blue());
    Ok(())
}

fn gate_kubectl() {
    if ensure_kubectl().is_err() {
        println!(
            "{} kubectl not found. Install it: {}",
            "✗".red(),
            "https://kubernetes.io/docs/tasks/tools/".cyan()
        );
        std::process::exit(1);
    }
    println!("{} kubectl found", "✓".green());
}

async fn wait_pod_ready(
    client: &CanineClient,
    project_id: &str,
    pod_id: &str,
) -> Result<Pod, CanineError> {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    for i in 1..=30 {
        print!("\r{} Waiting for pod to be ready", frames[i % frames.len()].cyan());
        io::stdout().flush().unwrap();

        sleep(Duration::from_millis(400));
        let pod = client.get_pod(project_id, pod_id).await?;
        if pod.status == ProcessStatus::Running {
            println!("\r{} Pod ready                      ", "✓".green());
            return Ok(pod);
        }
    }
    println!("\r{} Pod failed to start            ", "✗".red());
    Err(CanineError::OneOffPodNeverReady)
}
