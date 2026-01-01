use colored::Colorize;

use crate::cli::AccountId;
use crate::client::{CanineClient, CanineError};
use crate::config::CanineConfig;

pub async fn handle_change_account(
    config: &CanineConfig,
    client: &CanineClient,
    account_id: &AccountId,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.me().await?;

    if response
        .accounts
        .iter()
        .any(|item| item.slug.contains(&account_id.account))
    {
        println!("Changing account to {}", account_id.account.green());
        Ok(config.change_account(&account_id.account)?)
    } else {
        Err(Box::new(CanineError::NoAccount(format!(
            "Account {} not found",
            account_id.account
        ))))
    }
}
