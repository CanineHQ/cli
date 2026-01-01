use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "k9", version, about = "K9 CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub namespace: Namespace,
}

#[derive(Subcommand, Debug)]
pub enum Namespace {
    /// Authentication commands
    Auth(AuthCmd),

    /// Account commands
    Account(AccountCmd),

    /// Project commands
    Project(ProjectCmd),

    /// Cluster commands
    Cluster(ClusterCmd),
}

// Auth commands

#[derive(Args, Debug)]
pub struct AuthCmd {
    #[command(subcommand)]
    pub action: AuthAction,
}

#[derive(Subcommand, Debug)]
pub enum AuthAction {
    /// Login to K9
    Login(AuthLogin),

    /// Show auth status
    Status,

    /// Logout
    Logout,
}

#[derive(Args, Debug)]
pub struct AuthLogin {
    /// API token for authentication
    #[arg(long)]
    pub token: String,

    /// Custom API host
    #[arg(long)]
    pub host: Option<String>,

    /// Account slug to use
    #[arg(long)]
    pub account: Option<String>,
}

// Account commands

#[derive(Args, Debug)]
pub struct AccountCmd {
    #[command(subcommand)]
    pub action: AccountAction,
}

#[derive(Subcommand, Debug)]
pub enum AccountAction {
    /// Change account
    ChangeAccount(AccountId),
}

#[derive(Args, Debug)]
pub struct AccountId {
    pub account: String,
}

// Project commands

#[derive(Args, Debug)]
pub struct ProjectCmd {
    #[command(subcommand)]
    pub action: ProjectAction,
}

#[derive(Subcommand, Debug)]
pub enum ProjectAction {
    /// List projects
    List(ProjectList),

    /// Open a shell into a project
    Shell(ProjectId),

    /// Deploy a project
    Deploy(DeployProjectParams),

    /// List processes for a project
    Processes(ProjectId),
}

#[derive(Args, Debug)]
pub struct ProjectId {
    #[arg(long)]
    pub project: String,
}

#[derive(Args, Debug)]
pub struct DeployProjectParams {
    #[arg(long)]
    pub name: String,

    #[arg(long, default_value_t = false)]
    pub skip_build: bool,
}

#[derive(Args, Debug)]
pub struct ProjectList {
    /// Show all projects (including archived)
    #[arg(long)]
    pub all: bool,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

// Cluster commands

#[derive(Args, Debug)]
pub struct ClusterCmd {
    #[command(subcommand)]
    pub action: ClusterAction,
}

#[derive(Subcommand, Debug)]
pub enum ClusterAction {
    /// Download kubeconfig file
    DownloadKubeconfig(ClusterId),
}

#[derive(Args, Debug)]
pub struct ClusterId {
    #[arg(long)]
    pub name: String,
}
