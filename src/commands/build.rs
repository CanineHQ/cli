use crate::client::{CanineClient};
use tabled::Table;

pub async fn handle_list(client: &CanineClient, project_id: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let builds = client.get_builds(project_id).await?.builds;
    println!("{}", Table::new(builds));
    Ok(())
}

pub async fn handle_kill(client: &CanineClient, build_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(client.kill_build(&build_id).await?)
}