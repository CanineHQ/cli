mod cli;
mod client;
mod commands;
mod config;
mod kubeconfig;

use clap::Parser;

use cli::{AccountAction, AddOnAction, AuthAction, BuildAction, Cli, ClusterAction, Namespace, ProjectAction};
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
            println!(
                "Using {} as backend, {}",
                client.base_url,
                config.account.as_deref().unwrap_or("Default")
            );

            match other {
                Namespace::Accounts(cmd) => match cmd.action {
                    AccountAction::ChangeAccount(account_id) => {
                        commands::account::handle_change_account(&config, &client, &account_id)
                            .await?;
                    }
                },
                Namespace::Projects(cmd) => match cmd.action {
                    ProjectAction::List => {
                        commands::project::handle_list(&client).await?;
                    }
                    ProjectAction::Processes(id) => {
                        commands::project::handle_processes(&client, &id).await?;
                    }
                    ProjectAction::Run(params) => {
                        commands::project::handle_run(&config, &client, &params).await?;
                    }
                    ProjectAction::Deploy(params) => {
                        commands::project::handle_deploy(&client, &params).await?;
                    }
                },
                Namespace::Builds(cmd) => match cmd.action {
                    BuildAction::List(list) => {
                        commands::build::handle_list(&client, &list.project).await?;
                    }
                    BuildAction::Kill(id) => {
                        commands::build::handle_kill(&client, &id.build).await?;
                    }
                }
                Namespace::Clusters(cmd) => match cmd.action {
                    ClusterAction::List => {
                        commands::cluster::handle_list(&client).await?;
                    }
                    ClusterAction::DownloadKubeconfig(id) => {
                        commands::cluster::handle_download_kubeconfig(&config, &client, &id)
                            .await?;
                    }
                },
                Namespace::AddOns(cmd) => match cmd.action {
                    AddOnAction::List => {
                        commands::add_on::handle_list(&client).await?;
                    }
                    AddOnAction::Restart(id) => {
                        commands::add_on::handle_restart(&client, &id).await?;
                    }
                }
                Namespace::Auth(_) => unreachable!(),
            }
        }
    }

    Ok(())
}
