use crate::NetScout;
use crate::models::{ScanConfig, ScanResult};

pub async fn run_scan(
    config: ScanConfig
) -> anyhow::Result<ScanResult> {
    let netscout = NetScout::new()?;
    
    netscout.run_scan(config).await?;
}
