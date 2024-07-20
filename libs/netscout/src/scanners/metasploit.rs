use async_trait::async_trait;
use crate::models::{ScanResult, ScanConfig};
use crate::scanners::Scanner;
use anyhow::Result;

pub struct MetasploitScanner;

impl MetasploitScanner {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Scanner for MetasploitScanner {
    async fn scan(&self, config: &ScanConfig) -> Result<ScanResult> {
        todo!("Implement Metasploit scanning")
        unimplemented!()
    }
}
