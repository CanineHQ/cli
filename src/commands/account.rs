use std::io::{self, BufRead, Write};

use colored::Colorize;

use crate::cli::AccountId;
use crate::client::{Account, CanineClient, CanineError};
use crate::config::CanineConfig;

fn select_account(accounts: &[Account]) -> Result<String, Box<dyn std::error::Error>> {
    if accounts.is_empty() {
        return Err("No accounts found".into());
    }
    println!("\nSelect an account:");
    for (i, a) in accounts.iter().enumerate() {
        println!("  [{}] {}", (i + 1).to_string().cyan(), a.slug);
    }
    print!("\n{} ", "Enter number:".bold());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input)?;
    let choice: usize = input.trim().parse().map_err(|_| "Invalid selection")?;
    if choice < 1 || choice > accounts.len() {
        return Err(format!("Selection out of range (1-{})", accounts.len()).into());
    }
    Ok(accounts[choice - 1].slug.clone())
}

pub async fn handle_change_account(
    config: &CanineConfig,
    client: &CanineClient,
    account_id: &AccountId,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.me().await?;

    let slug = if let Some(ref account) = account_id.account {
        if response.accounts.iter().any(|item| item.slug.contains(account)) {
            account.clone()
        } else {
            return Err(Box::new(CanineError::NoAccount(format!(
                "Account {} not found",
                account
            ))));
        }
    } else {
        select_account(&response.accounts)?
    };

    config.change_account(&slug)?;
    println!("{} Switched to account {}", "✓".green(), slug.cyan());
    Ok(())
}
