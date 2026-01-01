mod cli;
mod client;
mod commands;
mod config;
mod kubeconfig;

use clap::Parser;

use cli::{AccountAction, AuthAction, Cli, ClusterAction, Namespace, ProjectAction};
use client::{Auth, CanineClient};
use config::CanineConfig;

fn build_default_client(config: &CanineConfig) -> CanineClient {
    CanineClient::new(
        &config
            .host
            .clone()
            .unwrap_or_else(|| CanineConfig::DEFAULT_HOST.to_string()),
        Auth::ApiKey(config.token.clone().expect("Client is not authenticated")),
        config.account.clone(),
    )
    .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = CanineConfig::load();

    match cli.namespace {
        Namespace::Auth(cmd) => match cmd.action {
            AuthAction::Login(login) => {
                commands::auth::handle_login(login).await?;
            }
            AuthAction::Status => {
                commands::auth::handle_status(&config).await?;
            }
            AuthAction::Logout => {
                commands::auth::handle_logout().await?;
            }
        },

        other => {
            let client = build_default_client(&config);
            println!("Using {} as backend", client.base_url);

            match other {
                Namespace::Account(cmd) => match cmd.action {
                    AccountAction::ChangeAccount(account_id) => {
                        commands::account::handle_change_account(&config, &client, &account_id)
                            .await?;
                    }
                },
                Namespace::Project(cmd) => match cmd.action {
                    ProjectAction::List(list) => {
                        commands::project::handle_list(&client, &list).await?;
                    }
                    ProjectAction::Processes(id) => {
                        commands::project::handle_processes(&client, &id).await?;
                    }
                    ProjectAction::Shell(id) => {
                        commands::project::handle_shell(&config, &client, &id).await?;
                    }
                    ProjectAction::Deploy(params) => {
                        commands::project::handle_deploy(&client, &params).await?;
                    }
                },
                Namespace::Cluster(cmd) => match cmd.action {
                    ClusterAction::DownloadKubeconfig(id) => {
                        commands::cluster::handle_download_kubeconfig(&config, &client, &id)
                            .await?;
                    }
                },
                Namespace::Auth(_) => unreachable!(),
            }
        }
    }

    Ok(())
}
