use colored::Colorize;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

const DOCKER_COMPOSE_URL: &str = "https://raw.githubusercontent.com/CanineHQ/canine/refs/heads/main/docker-compose.yml";

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

pub async fn handle_start(port: u16) -> Result<(), Box<dyn std::error::Error>> {
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
        .env("PORT", port.to_string())
        .current_dir(local_dir())
        .status()?;

    if status.success() {
        println!("{} Local Canine environment started", "✓".green());
        println!("\n  Open {} in your browser", format!("http://localhost:{}", port).cyan());
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

    // Get structured JSON output from docker compose
    let output = Command::new("docker")
        .args(["compose", "ps", "--format", "json"])
        .current_dir(local_dir())
        .output()?;

    if !output.status.success() {
        println!("{} Failed to get container status", "✗".red());
        std::process::exit(1);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Parse each line as a JSON object (docker outputs one JSON object per line)
    let services: Vec<serde_json::Value> = stdout
        .lines()
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect();

    if services.is_empty() {
        println!("{} No services running", "✗".yellow());
        println!("  Run {} to start", "canine local start".cyan());
        return Ok(());
    }

    println!("\n{}", "Local Canine Services".bold());
    println!("{}", "─".repeat(50));

    for svc in &services {
        let name = svc["Service"].as_str().unwrap_or("unknown");
        let state = svc["State"].as_str().unwrap_or("unknown");
        let health = svc["Health"].as_str().unwrap_or("");
        let ports = svc["Publishers"].as_array();

        let status_icon = match state {
            "running" => "✓".green(),
            "exited" => "✗".red(),
            _ => "?".yellow(),
        };

        let health_str = if !health.is_empty() {
            format!(" ({})", health)
        } else {
            String::new()
        };

        // Extract published ports
        let port_str = ports
            .map(|p| {
                p.iter()
                    .filter_map(|pub_info| {
                        let published = pub_info["PublishedPort"].as_u64()?;
                        if published > 0 {
                            Some(format!(":{}", published))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default();

        println!(
            "{} {:<20} {:<10}{}  {}",
            status_icon,
            name,
            state,
            health_str,
            port_str.cyan()
        );
    }

    // Find the main web port from the "web" service
    let web_port = services.iter().find_map(|svc| {
        let name = svc["Service"].as_str().unwrap_or("");
        if name != "web" {
            return None;
        }
        svc["Publishers"].as_array().and_then(|publishers| {
            publishers.iter().find_map(|pub_info| {
                let published = pub_info["PublishedPort"].as_u64()?;
                if published > 0 {
                    Some(published)
                } else {
                    None
                }
            })
        })
    });

    if let Some(port) = web_port {
        println!("\n  Open {} in your browser", format!("http://localhost:{}", port).cyan());
    }

    println!();
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
    if !docker_compose_path().exists() {
        println!("{} Local Canine environment is not installed", "✗".red());
        println!("  Run {} to install", "canine local start".cyan());
        std::process::exit(1);
    }

    println!("{} Pulling latest images...", "→".cyan());

    let status = Command::new("docker")
        .args(["compose", "pull"])
        .current_dir(local_dir())
        .status()?;

    if status.success() {
        println!("{} Images updated successfully", "✓".green());
        println!("  Run {} to apply the upgrade", "canine local start".cyan());
    } else {
        println!("{} Failed to pull images", "✗".red());
        std::process::exit(1);
    }

    Ok(())
}
