use colored::Colorize;
use tabled::Table;

use crate::cli::AuthLogin;
use crate::client::{Auth, CanineClient, CanineError};
use crate::config::CanineConfig;

pub async fn handle_login(login: AuthLogin) -> Result<(), Box<dyn std::error::Error>> {
    let host = match login.host {
        Some(h) => h,
        None => {
            println!("No host specified, using {}", CanineConfig::DEFAULT_HOST.cyan());
            CanineConfig::DEFAULT_HOST.to_string()
        }
    };

    let client = CanineClient::new(&host, Auth::ApiKey(login.token.clone()), login.account)?;

    match client.me().await {
        Ok(me) => {
            CanineConfig {
                host: Some(host),
                token: Some(login.token),
                account: Some(me.current_account.slug),
            }
            .save()?;
            println!("{} Authenticated as {}", "✓".green(), me.email.green());
            println!(
                "  Credentials saved to {}",
                CanineConfig::config_path().to_str().unwrap().dimmed()
            );
        }
        Err(CanineError::Api(api_err)) => {
            println!("{} Authentication failed: {}", "✗".red(), api_err);
        }
        Err(e) => return Err(Box::new(e)),
    };

    Ok(())
}

pub async fn handle_logout() -> Result<(), Box<dyn std::error::Error>> {
    CanineConfig::clear();
    println!("{} Logged out successfully", "✓".green());
    Ok(())
}

pub async fn handle_status(config: &CanineConfig) -> Result<(), Box<dyn std::error::Error>> {
    let token = config.token.clone().ok_or_else(|| CanineError::NoToken)?;

    let host = config
        .host
        .clone()
        .unwrap_or_else(|| CanineConfig::DEFAULT_HOST.to_string());

    let client = CanineClient::new(&host, Auth::ApiKey(token), config.account.clone())?;

    let response = client.me().await?;
    println!("Logged in as {}  (account: {})", response.email.green(), response.current_account.slug.cyan());
    println!();
    println!("Available accounts:");
    println!("{}", Table::new(response.accounts));

    Ok(())
}
