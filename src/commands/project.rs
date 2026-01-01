use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

use colored::Colorize;
use tabled::Table;

use crate::cli::{DeployProjectParams, ProjectId, ProjectList};
use crate::client::{CanineClient, CanineError, Pod, ProcessStatus};
use crate::config::CanineConfig;
use crate::kubeconfig::{ensure_kubectl, kubeconfig_to_yaml};

pub async fn handle_list(
    client: &CanineClient,
    _list: &ProjectList,
) -> Result<(), Box<dyn std::error::Error>> {
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

pub async fn handle_shell(
    config: &CanineConfig,
    client: &CanineClient,
    id: &ProjectId,
) -> Result<(), Box<dyn std::error::Error>> {
    gate_kubectl();

    let project = client.get_project(&id.project).await?;

    // Save kubeconfig
    let kubeconfig = client
        .download_kubeconfig_file(&project.cluster_id.to_string())
        .await?;
    let yaml = kubeconfig_to_yaml(&kubeconfig.kubeconfig)?;
    config.save_kubeconfig(yaml)?;

    println!(
        "Starting a one off container in: {}...",
        project.name.green()
    );

    let pod = client.create_one_off_pod(&id.project).await?;
    println!("Created one off pod: {}", pod.name.green());
    print!("Waiting for pod to be ready...");

    wait_pod_ready(client, &id.project, &pod.name).await?;

    println!();
    Command::new("kubectl")
        .args([
            "exec",
            "-it",
            "-n",
            &pod.namespace,
            &pod.name,
            "--",
            "/bin/sh",
        ])
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

pub async fn handle_deploy(
    client: &CanineClient,
    params: &DeployProjectParams,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = client
        .deploy_project(&params.name, params.skip_build)
        .await?;
    println!("Message: {}\tBuild ID: {}", result.message, result.build_id);
    Ok(())
}

fn gate_kubectl() {
    if ensure_kubectl().is_err() {
        println!(
            "{}",
            "Please install kubectl: https://kubernetes.io/docs/tasks/tools/".red()
        );
        std::process::exit(1);
    }
    println!("Checking `kubectl` is installed... done.");
}

async fn wait_pod_ready(
    client: &CanineClient,
    project_id: &str,
    pod_id: &str,
) -> Result<Pod, CanineError> {
    let frames = ['|', '/', '-', '\\'];
    for i in 1..=30 {
        print!("\r{}", frames[i % frames.len()]);
        io::stdout().flush().unwrap();

        sleep(Duration::from_secs(1));
        let pod = client.get_pod(project_id, pod_id).await?;
        if pod.status == ProcessStatus::Running {
            return Ok(pod);
        }
    }
    Err(CanineError::OneOffPodNeverReady)
}
