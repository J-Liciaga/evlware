pub mod scanners;
pub mod models;
// pub mod utils;
pub mod api;
// pub mod cli;
use anyhow::{Result, Error};
// use jarvis::MLEnhancer;
use models::{ScanResult, ScanConfig};
use scanners::{ZapScanner, MetasploitScanner, Scanner};
// use pyo3::Python;

pub struct NetScout {
    zap_scanner: ZapScanner,
    metasploit_scanner: MetasploitScanner,
    // ml_enhancer: MLEnhancer,
}

impl NetScout {
    pub async fn new() -> Result<Self, Error> {
        let zap_url = std::env::var("ZAP_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
        let zap_api_key = std::env::var("ZAP_API_KEY").context("ZAP_API_KEY not set")?;
        let msf_url = std::env::var("MSF_URL").unwrap_or_else(|_| "localhost:55553".to_string());

        Ok(Self {
            zap_scanner: ZapScanner::new(&zap_url, &zap_api_key)?,
            metasploit_scanner: MetasploitScanner::new(&msf_url).await?,
            // ml_enhancer: MLEnhancer::new()?,
        })
    }

    pub async fn run_scan(&self, config: ScanConfig) -> Result<ScanResult> {
        let zap_result = self.zap_scanner.scan(&config).await?;
        let metasploit_result = self.metasploit_scanner.scan(&config).await?;
        
        let combined_result = ScanResult {
            target: zap_result.target,
            vulnerabilities: zap_result.vulnerabilities.into_iter().chain(metasploit_result.vulnerabilities).collect(),
            exploit_suggestions: metasploit_result.exploit_suggestions,
        };
        
        let enhanced_result = self.ml_enhancer.enhance(combined_result)?;
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
