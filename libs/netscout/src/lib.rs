pub mod scanners;
pub mod models;
pub mod utils;
pub mod api;
pub mod cli;

use anyhow::Result;
// use jarvis::MLEnhancer;
use models:{ScanResult, ScanConfig};
use scanners::{ZapScanner, MetasploitScanner};
// use pyo3::Python;

pub struct NetScout {
    zap_scanner: ZapScanner,
    metasploit_scanner: MetasploitScanner,
    ml_enhancer: MLEnhancer,
}

impl NetScout {
    pub fn new() -> Result<Self> {
        Ok(Self {
            zap_scanner: ZapScanner::new(),
            metasploit_scanner: MetasploitScanner::new(),
            // ml_enhancer: MLEnhancer::new()?,
        })
    }

    pub async fn run_scan(
        &self,
        config: ScanConfig,
    ) -> Result<ScanResult> {
        let zap_result = self.zap_scanner.scan(&config).await?;
        let metasploit_result = self.metasploit_scanner.scan(&config).await?;

        let combined_result = self.combine_results(zap_result, metasploit_result);

        // Python::with_gil(|py| {
        //     let py_result = combined_result.to_object(py);
        //     let enhanced_result = self.ml_enhancer.enhance(py_result)?;
        //     // Convert enhanced_result back to Rust ScanResult
        //     // This step depends on how you define your data structures
        //     todo!("Convert Python object to Rust ScanResult")
        // })

        unimplemented!()
    }

    fn combine_results(
        &self,
        zap_result: ScanResult,
        metasploit_result: ScanResult,
    ) -> ScanResult {
        unimplemented!()
    }
}
