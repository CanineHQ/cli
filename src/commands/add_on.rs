use tabled::Table;

use crate::client::{CanineClient};
use crate::cli::{AddOnId};

pub async fn handle_list(client: &CanineClient) -> Result<(), Box<dyn std::error::Error>> {
    let result = client.get_add_ons().await?;
    println!("{}", Table::new(result.add_ons));
    Ok(())
}

pub async fn handle_restart(client: &CanineClient, add_on_id: &AddOnId) -> Result<(), Box<dyn std::error::Error>> {
    Ok(client.restart_add_on(&add_on_id.add_on).await?)
}
