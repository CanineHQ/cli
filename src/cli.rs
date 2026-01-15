use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "k9", version, about = "Canine CLI - Manage your Canine projects, clusters, and local development environment")]
pub struct Cli {
    #[command(subcommand)]
    pub namespace: Namespace,
}

#[derive(Subcommand, Debug)]
pub enum Namespace {
    /// Manage authentication (login, logout, status)
    Auth(AuthCmd),

    /// Switch between Canine accounts
    Accounts(AccountCmd),

    /// Manage projects (list, deploy, run commands)
    Projects(ProjectCmd),

    /// Manage Kubernetes clusters (list, download kubeconfig, connect)
    Clusters(ClusterCmd),

    /// Manage project builds (list, kill)
    Builds(BuildCmd),

    /// Manage add-ons (list, restart)
    AddOns(AddOnCmd),

    /// Run Canine locally with Docker Compose
    Local(LocalCmd),
}

// Build commands

#[derive(Args, Debug)]
pub struct BuildCmd {
    #[command(subcommand)]
    pub action: BuildAction,
}

#[derive(Subcommand, Debug)]
pub enum BuildAction {
    /// List builds
    List(BuildList),

    /// Kill a specific build
    Kill(BuildId),
}

// Auth commands

#[derive(Args, Debug)]
pub struct AuthCmd {
    #[command(subcommand)]
    pub action: AuthAction,
}

#[derive(Subcommand, Debug)]
pub enum AuthAction {
    /// Login to Canine
    Login(AuthLogin),

    /// Show current logged in user, and available accounts
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

#[derive(Args, Debug)]
pub struct BuildList {
    pub project: Option<String>,
}

#[derive(Args, Debug)]
pub struct BuildId {
    pub build: String,
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
    List,

    /// Run a command in a project
    Run(ProjectRun),

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
pub struct ProjectRun {
    #[arg(long)]
    pub project: String,

    /// Command to run (e.g., "bundle exec rails c")
    #[arg(trailing_var_arg = true, required = true)]
    pub command: Vec<String>,
}

#[derive(Args, Debug)]
pub struct DeployProjectParams {
    #[arg(long)]
    pub name: String,

    #[arg(long, default_value_t = false)]
    pub skip_build: bool,
}

// Cluster commands
#[derive(Args, Debug)]
pub struct ClusterCmd {
    #[command(subcommand)]
    pub action: ClusterAction,
}

#[derive(Args, Debug)]
pub struct AddOnCmd {
    #[command(subcommand)]
    pub action: AddOnAction,
}

#[derive(Subcommand, Debug)]
pub enum ClusterAction {
    /// List clusters
    List,
    /// Download kubeconfig file
    DownloadKubeconfig(ClusterId),

    // Connect to cluster via telepresence
    Connect(ClusterId)
}

#[derive(Subcommand, Debug)]
pub enum AddOnAction {
    /// List add ons
    List,
    /// Download kubeconfig file
    Restart(AddOnId),
}

#[derive(Args, Debug)]
pub struct ClusterId {
    #[arg(long)]
    pub cluster: String,
}

#[derive(Args, Debug)]
pub struct AddOnId {
    #[arg(long)]
    pub add_on: String,
}

// Local commands
#[derive(Args, Debug)]
pub struct LocalCmd {
    #[command(subcommand)]
    pub action: LocalAction,
}

#[derive(Subcommand, Debug)]
pub enum LocalAction {
    /// Start local Canine environment
    Start {
        /// Port to run the local environment on
        #[arg(long, short, default_value = "3000")]
        port: u16,
    },

    /// Show status of local Canine environment
    Status,

    /// Stop local Canine environment
    Stop,

    /// Upgrade local Canine environment
    Upgrade,
}
