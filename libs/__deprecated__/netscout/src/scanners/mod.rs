mod zap;
mod metasploit;

pub use zap::ZapScanner;
pub use metasploit::MetasploitScanner;

use async_trait::async_trait;
use crate::models::{ScanResult, ScanConfig};
use anyhow::Result;

#[async_trait]
pub trait Scanner {
    async fn scan(
        &self,
        config: &ScanConfig,
    ) -> Result<ScanResult>;
}
