use tabled::Table;
use colored::Colorize;
use std::io::{self, BufRead, Write};
use std::process::Command;
use crate::cli::ClusterId;
use crate::client::CanineClient;
use crate::config::CanineConfig;
use crate::kubeconfig::kubeconfig_to_yaml;

pub enum TelepresenceError {
    NotFound,
    NotExecutable(io::Error),
    FailedToRun(String),
}

pub fn gate_telepresence() -> Result<(), TelepresenceError> {
    let output = Command::new("telepresence")
        .arg("version")
        .output()
        .map_err(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                TelepresenceError::NotFound
            } else {
                TelepresenceError::NotExecutable(e)
            }
        })?;

    if output.status.success() {
        Ok(())
    } else {
        Err(TelepresenceError::FailedToRun(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}


pub async fn handle_list(client: &CanineClient) -> Result<(), Box<dyn std::error::Error>> {
    let clusters = client.get_clusters().await?.clusters;
    println!("{}", Table::new(clusters));
    Ok(())
}

fn select_cluster_name(clusters: &[crate::client::Cluster]) -> Result<String, Box<dyn std::error::Error>> {
    if clusters.is_empty() {
        return Err("No clusters found".into());
    }
    println!("\nSelect a cluster:");
    for (i, c) in clusters.iter().enumerate() {
        println!("  [{}] {} ({})", (i + 1).to_string().cyan(), c.name, format!("{}", c.status).dimmed());
    }
    print!("\n{} ", "Enter number:".bold());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input)?;
    let choice: usize = input.trim().parse().map_err(|_| "Invalid selection")?;
    if choice < 1 || choice > clusters.len() {
        return Err(format!("Selection out of range (1-{})", clusters.len()).into());
    }
    Ok(clusters[choice - 1].name.clone())
}

async fn resolve_cluster_name(client: &CanineClient, id: &ClusterId) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(ref name) = id.cluster {
        Ok(name.clone())
    } else {
        let clusters = client.get_clusters().await?.clusters;
        select_cluster_name(&clusters)
    }
}

pub async fn handle_download_kubeconfig(
    config: &CanineConfig,
    client: &CanineClient,
    id: &ClusterId,
) -> Result<(), Box<dyn std::error::Error>> {
    let cluster_name = resolve_cluster_name(client, id).await?;
    let kubeconfig = client.download_kubeconfig_file(&cluster_name).await?;
    let yaml = kubeconfig_to_yaml(&kubeconfig.kubeconfig)?;
    config.save_kubeconfig(yaml)?;
    Ok(())
}

pub async fn handle_connect(
    config: &CanineConfig,
    client: &CanineClient,
    id: &ClusterId,
) -> Result<(), Box<dyn std::error::Error>> {
    if gate_telepresence().is_err() {
        println!(
            "{} telepresence not found. Install it here: {}",
            "✗".red(),
            "https://telepresence.io/docs/install/client".cyan()
        );
        std::process::exit(1);
    }
    println!("{} telepresence found", "✓".green());
    let cluster_name = resolve_cluster_name(client, id).await?;
    let kubeconfig = client.download_kubeconfig_file(&cluster_name).await?;
    let yaml = kubeconfig_to_yaml(&kubeconfig.kubeconfig)?;
    config.save_kubeconfig(yaml)?;

    Command::new("telepresence")
        .args(vec!["connect"])
        .env(
            "KUBECONFIG",
            CanineConfig::credential_path().to_str().unwrap(),
        )
        .status()?;

    Ok(())
}
