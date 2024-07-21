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
    // ml_enhancer: MLEnhancer,
}

impl NetScout {
    pub fn new() -> Result<Self> {
        let zap_url = std::env::var("ZAP_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
        let msf_url = std::env::var("MSF_URL").unwrap_or_else(|_| "localhost:55553".to_string());

        Ok(Self {
            zap_scanner: ZapScanner::new(&zap_url)?,
            metasploit_scanner: MetasploitScanner::new(&msf_url).await?,
            // ml_enhancer: MLEnhancer::new()?,
        });
    };

    pub async fn run_scan(
        &self,
        config: ScanConfig,
    ) -> Result<ScanResult> {
        let zap_result = self.zap_scanner.scan(&config).await?;
        let metasploit_result = self.metasploit_scanner.scan(&config).await?;  
        let combined_result = self.combine_results(zap_result, metasploit_result);
        // let enhanced_result = self.ml_enhancer.enhance(combined_result)?;
        
        Ok(enhanced_result)

    }

    fn combine_results(
        &self,
        zap_result: ScanResult,
        metasploit_result: ScanResult,
    ) -> ScanResult {
        // placeholder implementation
        ScanResult {
            target: zap_result.target,
            vulnerabilities: [zap_result.vulnerabilities, metasploit_result.vulnerabilities].concat(),
            exploit_suggestions: metasploit_result.exploit_suggestions,
        }
    }
}
