use async_trait::async_trait;
use crate::models::{ScanResult, ScanConfig};
use crate::scanners::Scanner;
use anyhow::Result;

pub struct ZapScanner;

impl ZapScanner {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Scanner for ZapScanner {
    async fn scan(&self, config: &ScanConfig) -> Result<ScanResult> {
        todo!("Implement ZAP scanning")
        unimplemented!()
    }
}
