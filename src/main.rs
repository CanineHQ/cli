mod client;
mod kubeconfig;
use std::thread::sleep;
use std::io::{self, Write};

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;
use tabled::Table;
use client::CanineClient;
use std::fs;
use colored::*;
use serde_yaml;
use serde::{Serialize, Deserialize};
use clap::{Args, Parser, Subcommand};

use crate::{client::{Auth, CanineError}, kubeconfig::kubeconfig_to_yaml};

#[derive(Parser, Debug)]
#[command(name = "k9", version, about = "K9 CLI")]
struct Cli {
    #[command(subcommand)]
    namespace: Namespace,
}

#[derive(Subcommand, Debug)]
enum Namespace {
    /// Authentication commands
    Auth(AuthCmd),

    /// Project commands
    Project(ProjectCmd),

    /// Project commands
    Cluster(ClusterCmd),
}

#[derive(Subcommand, Debug)]
enum ClusterAction {
    /// Download kubeconfig file
    DownloadKubeconfig(ClusterId),
}

#[derive(Args, Debug)]
struct AuthCmd {
    #[command(subcommand)]
    action: AuthAction,
}

#[derive(Subcommand, Debug)]
enum AuthAction {
    /// Login to K9
    Login(AuthLogin),

    /// Show auth status
    Status,

    /// Logout
    Logout,
}

#[derive(Args, Debug)]
struct AuthLogin {
    /// Optional auth provider (e.g. github)
    #[arg(long)]
    token: String,

    #[arg(long)]
    host: Option<String>,
}

#[derive(Args, Debug)]
struct ProjectCmd {
    #[command(subcommand)]
    action: ProjectAction,
}

#[derive(Args, Debug)]
struct ClusterCmd {
    #[command(subcommand)]
    action: ClusterAction,
}


#[derive(Subcommand, Debug)]
enum ProjectAction {
    List(ProjectList),
    Shell(ProjectId),
    Deploy(DeployProjectParams),
    Processes(ProjectId),
}

#[derive(Args, Debug)]
struct ClusterId {
    #[arg(long)]
    name: String,
}

#[derive(Args, Debug)]
struct ProjectId {
    #[arg(long)]
    name: String,
}

#[derive(Args, Debug)]
struct DeployProjectParams {
    #[arg(long)]
    name: String,
    #[arg(long, default_value_t = false)]
    skip_build: bool,
}

#[derive(Args, Debug)]
struct ProjectList {
    /// Show all projects (including archived)
    #[arg(long)]
    all: bool,

    /// Output as JSON
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct CanineConfig {
    host: Option<String>,
    token: Option<String>
}

impl CanineConfig {
    pub const DEFAULT_HOST: &'static str = "https://canine.sh";
    pub fn credential_path() -> PathBuf {
        dirs::home_dir()
            .expect("Could not determine home directory")
            .join(".k9/kubeconfig.yaml")
    }

    pub fn config_path() -> PathBuf {
        dirs::home_dir()
            .expect("Could not determine home directory")
            .join(".k9/canine.yaml")
    }

    pub fn gate_directory(path: &Path) {
        let dir = path.parent().unwrap_or(Path::new("."));
        fs::create_dir_all(dir).expect("Failed to create parent directory");
    }

    pub fn clear() {
        CanineConfig::gate_directory(&CanineConfig::config_path());
        fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&CanineConfig::config_path()).unwrap();
    }

    pub fn save_kubeconfig(&self, yaml: String) -> Result<(), Box<dyn std::error::Error>> {
        CanineConfig::gate_directory(&CanineConfig::config_path());
        fs::write(&CanineConfig::credential_path(), yaml)?;
        println!("Saved kubeconfig to {}", CanineConfig::credential_path().to_str().unwrap());
        Ok(())
    }

    pub fn load() -> Self {
        CanineConfig::gate_directory(&CanineConfig::config_path());

        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(&CanineConfig::config_path())
            .expect(&format!("failed to open {}", CanineConfig::config_path().to_str().unwrap()));

        let contents = std::fs::read_to_string(&CanineConfig::config_path())
            .expect(&format!("failed to read {}", CanineConfig::config_path().to_str().unwrap()));

        let config: CanineConfig = serde_yaml::from_str(&contents)
            .expect(&format!("failed to parse {}", CanineConfig::config_path().to_str().unwrap()));

        return config
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(fs::write(&CanineConfig::config_path(), yaml)?)
    }
}

async fn handle_login(login: AuthLogin) -> Result<(), Box<dyn std::error::Error>> {
    // Store the token in ~/.canine/canine.yaml
    let mut config = CanineConfig::load();
    let host = match login.host {
        Some(h) => h,
        None => {
            println!("Defaulting host to {}", CanineConfig::DEFAULT_HOST);
            CanineConfig::DEFAULT_HOST.to_string()
        },
    };
    let client: CanineClient = CanineClient::new(
        &host,
        Auth::ApiKey(login.token.clone()),
    )?;
    match client.me().await {
        Ok(me) => {
            println!("Logged in as {}", me.email.green());
            config.token = Some(login.token);
            config.host = Some(host);
            config.save()?;
            println!("Saved credentials to {}", CanineConfig::config_path().to_str().unwrap().green());
        },
        Err(CanineError::Api(api_err)) => {
            println!("API Error: {}", api_err);
        },
        Err(e) => return Err(Box::new(e)),
    };
    Ok(())
}

async fn handle_logout() -> Result<(), Box<dyn std::error::Error>> {
    CanineConfig::clear();
    Ok(())
}

async fn status(config: CanineConfig) -> Result<(), Box<dyn std::error::Error>> {
    let token = config
        .token
        .ok_or_else(|| CanineError::NoToken)?;

    let host = config
        .host
        .unwrap_or_else(|| CanineConfig::DEFAULT_HOST.to_string());

    let client: CanineClient = CanineClient::new(
        &host,
        Auth::ApiKey(token),
    )?;

    let response = client.me().await?;
    println!("Currently logged in as: {}", response.email.green());
    Ok(())
}

async fn wait_pod_ready(client: &CanineClient, project_id: &str, pod_id: &str) -> Result<client::Pod, CanineError> {
    let frames = ['|', '/', '-', '\\'];
    for i in 1..=30 {
        print!("\r{}", frames[i % frames.len()]);
        io::stdout().flush().unwrap();

        sleep(Duration::from_secs(1));
        let pod = client.get_pod(&project_id, &pod_id).await?;
        if pod.status == client::ProcessStatus::Running {
            return Ok(pod);
        }
    }
    return Err(CanineError::OneOffPodNeverReady).unwrap();
}
fn gate_kubectl() {
    if let Err(_) = kubeconfig::ensure_kubectl() {
        println!("{}", "Please install kubectl: https://kubernetes.io/docs/tasks/tools/".red());
        std::process::exit(1);
    }
    println!("Checking `kubectl` is installed... done.");
}
fn build_default_client(config: &CanineConfig) -> CanineClient {
    CanineClient::new(
        &config.host.clone().unwrap_or_else(|| CanineConfig::DEFAULT_HOST.to_string()),
        Auth::ApiKey(config.token.clone().expect("Client is not authenticated")),
    ).unwrap()
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cli = Cli::parse();
    let config = CanineConfig::load();

    match cli.namespace {
        Namespace::Auth(cmd) => match cmd.action {
            AuthAction::Login(login) => {
                handle_login(login).await?;
            }
            AuthAction::Status => {
                status(config).await?;
            }
            AuthAction::Logout => {
                println!("auth logout");
                handle_logout().await?;
                // TODO: implement logout
            }
        },

        other => {
            let client = build_default_client(&config);
            println!("Using {} as backend", client.base_url);
            match other {
                Namespace::Project(cmd) => match cmd.action {
                    ProjectAction::List(list) => {
                        let projects = client.get_projects().await?.projects;
                        println!("{}", Table::new(projects));
                    }
                    ProjectAction::Processes(id) => {
                        let processes = client.get_processes(&id.name).await?;
                        println!("{}", Table::new(processes.pods));
                    }
                    ProjectAction::Shell(id) => {
                        gate_kubectl();
                        let project = client.get_project(&id.name).await?;

                        // Save kubeconfig
                        let kubeconfig = client.download_kubeconfig_file(&project.cluster_id.to_string()).await?;
                        let yaml = kubeconfig_to_yaml(&kubeconfig.kubeconfig)?;
                        config.save_kubeconfig(yaml)?;

                        println!("Starting a one off container in: {}...", project.name.green());
                        // TODO: implement shell
                        let mut pod = client.create_one_off_pod(&id.name).await?;
                        println!("Created one off pod: {}", pod.name.green());
                        print!("Waiting for pod to be ready...");

                        wait_pod_ready(&client, &id.name, &pod.name).await?;

                        println!("");
                        let status = Command::new("kubectl")
                            .args([
                                "exec", "-it",
                                "-n", &pod.namespace,
                                &pod.name,
                                "--",
                                "/bin/sh",
                            ])
                            .env("KUBECONFIG", CanineConfig::credential_path().to_str().unwrap())
                            .stdin(Stdio::inherit())
                            .stdout(Stdio::inherit())
                            .stderr(Stdio::inherit())
                            .status()?; 
                    }
                    ProjectAction::Deploy(params) => {
                        let result = client.deploy_project(&params.name, params.skip_build).await?;
                        println!("Message: {}\tBuild ID: {}", result.message, result.build_id);
                    }
                },
                Namespace::Cluster(cmd) => match cmd.action {
                    ClusterAction::DownloadKubeconfig(id) => {
                        let kubeconfig = client.download_kubeconfig_file(&id.name).await?;
                        let yaml = kubeconfig_to_yaml(&kubeconfig.kubeconfig)?;
                        config.save_kubeconfig(yaml)?;
                    }
                }
                Namespace::Auth(_) => unreachable!("This is impossible")
            }
        }
    }
    Ok(())
}