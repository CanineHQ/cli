use crate::cli::ClusterId;
use crate::client::CanineClient;
use crate::config::CanineConfig;
use crate::kubeconfig::kubeconfig_to_yaml;

pub async fn handle_download_kubeconfig(
    config: &CanineConfig,
    client: &CanineClient,
    id: &ClusterId,
) -> Result<(), Box<dyn std::error::Error>> {
    let kubeconfig = client.download_kubeconfig_file(&id.cluster).await?;
    let yaml = kubeconfig_to_yaml(&kubeconfig.kubeconfig)?;
    config.save_kubeconfig(yaml)?;
    Ok(())
}
