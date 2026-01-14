use colored::Colorize;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

const DOCKER_COMPOSE_URL: &str = "https://raw.githubusercontent.com/CanineHQ/canine/refs/heads/main/install/portainer/docker-compose.yml";

pub enum DockerComposeError {
    NotFound,
    NotExecutable(io::Error),
    FailedToRun(String),
}

pub fn local_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Could not determine home directory")
        .join(".k9/local")
}

pub fn docker_compose_path() -> PathBuf {
    local_dir().join("docker-compose.yml")
}

pub fn check_docker_compose() -> Result<(), DockerComposeError> {
    let output = Command::new("docker")
        .args(["compose", "version"])
        .output()
        .map_err(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                DockerComposeError::NotFound
            } else {
                DockerComposeError::NotExecutable(e)
            }
        })?;

    if output.status.success() {
        Ok(())
    } else {
        Err(DockerComposeError::FailedToRun(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}

async fn download_docker_compose() -> Result<(), Box<dyn std::error::Error>> {
    println!("{} Downloading docker-compose.yml...", "→".cyan());

    let response = reqwest::get(DOCKER_COMPOSE_URL).await?;

    if !response.status().is_success() {
        return Err(format!("Failed to download docker-compose.yml: {}", response.status()).into());
    }

    let content = response.text().await?;

    // Ensure directory exists
    fs::create_dir_all(local_dir())?;

    // Save the file
    fs::write(docker_compose_path(), content)?;

    println!(
        "{} Saved docker-compose.yml to {}",
        "✓".green(),
        docker_compose_path().to_str().unwrap().cyan()
    );

    Ok(())
}

pub async fn handle_start() -> Result<(), Box<dyn std::error::Error>> {
    if check_docker_compose().is_err() {
        println!(
            "{} Docker Compose not found. Install Docker Desktop: {}",
            "✗".red(),
            "https://docs.docker.com/compose/install/".cyan()
        );
        std::process::exit(1);
    }
    println!("{} Docker Compose found", "✓".green());

    download_docker_compose().await?;

    println!("{} Starting local Canine environment...", "→".cyan());

    let status = Command::new("docker")
        .args(["compose", "up", "-d"])
        .current_dir(local_dir())
        .status()?;

    if status.success() {
        println!("{} Local Canine environment started", "✓".green());
    } else {
        println!("{} Failed to start local Canine environment", "✗".red());
        std::process::exit(1);
    }

    Ok(())
}

pub async fn handle_status() -> Result<(), Box<dyn std::error::Error>> {
    if !docker_compose_path().exists() {
        println!("{} Local Canine environment is not installed", "✗".red());
        println!("  Run {} to install", "canine local start".cyan());
        std::process::exit(1);
    }

    Command::new("docker")
        .args(["compose", "ps"])
        .current_dir(local_dir())
        .status()?;

    Ok(())
}

pub async fn handle_stop() -> Result<(), Box<dyn std::error::Error>> {
    if !docker_compose_path().exists() {
        println!("{} Local Canine environment is not installed", "✗".red());
        std::process::exit(1);
    }

    println!("{} Stopping local Canine environment...", "→".cyan());

    let status = Command::new("docker")
        .args(["compose", "down"])
        .current_dir(local_dir())
        .status()?;

    if status.success() {
        println!("{} Local Canine environment stopped", "✓".green());
    } else {
        println!("{} Failed to stop local Canine environment", "✗".red());
        std::process::exit(1);
    }

    Ok(())
}

pub async fn handle_upgrade() -> Result<(), Box<dyn std::error::Error>> {
    println!("{} Upgrading local Canine environment...", "→".cyan());
    Ok(())
}
