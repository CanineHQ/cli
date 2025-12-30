use std::path::Path;

mod client;
use client::CanineClient;
use std::fs;
use colored::*;
use serde_yaml;
use serde::{Serialize, Deserialize};
use clap::{Args, Parser, Subcommand};

use crate::client::{Auth, CanineError};

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

#[derive(Subcommand, Debug)]
enum ProjectAction {
    /// Open a shell for a project
    Shell(ProjectShell),

    /// List projects
    List(ProjectList),
}

#[derive(Args, Debug)]
struct ProjectShell {
    /// Project name (required)
    #[arg(long)]
    name: String,

    /// Optional container name
    #[arg(long)]
    container: Option<String>,
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
    pub const PATH: &'static str = "~/.canine/canine.yaml";

    pub fn gate_directory() {
        let path = Path::new(CanineConfig::PATH);
        let dir = match path.parent() {
            Some(p) => p,
            None => Path::new(".")
        };
        fs::create_dir_all(&dir).expect("Failed to create parent directory");
    }

    pub fn clear() {
        CanineConfig::gate_directory();
        fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(CanineConfig::PATH).unwrap();
    }

    pub fn load() -> Self {
        CanineConfig::gate_directory();

        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(CanineConfig::PATH).expect(&format!("failed to open {}", CanineConfig::PATH));

        let contents = std::fs::read_to_string(CanineConfig::PATH)
            .expect(&format!("failed to read {}", CanineConfig::PATH));

        let config: CanineConfig = serde_yaml::from_str(&contents)
            .expect(&format!("failed to parse {}", CanineConfig::PATH));

        return config
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(fs::write(CanineConfig::PATH, yaml)?)
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
            println!("Saved credentials to {}", CanineConfig::PATH.green());
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
    println!("email: {}", response.email.green());
    Ok(())
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

        Namespace::Project(cmd) => match cmd.action {
            ProjectAction::Shell(shell) => {
                println!("project shell (name={}, container={:?})", shell.name, shell.container);
                // TODO: implement shell
            }
            ProjectAction::List(list) => {
                println!("project list (all={}, json={})", list.all, list.json);
                // TODO: implement list
            }
        },
    }
    Ok(())
}