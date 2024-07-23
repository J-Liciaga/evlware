// pub mod scanner;
// pub mod enumerator;
// pub mod vulnerability_scanner;
pub mod models;

use std::error::Error;
use models::{ScanResults, EnumerationResults, VulnerabilityResults};

pub struct NetScout {

}

impl NetScout {
    pub fn new() -> Self {
        NetScout {

        }
    }

    pub async fn scan(
        &self,
        target: &str,
    ) -> Result<ScanResults, Box<dyn Error>> {
        println!("scanning on target: {}", target);
        todo!()
    }

    pub async fn enumerate(
        &self,
        target: &str,
    ) -> Result<EnumerationResults, Box<dyn Error>> {
        println!("enumerating on target: {}", target);
        todo!()
    }

    pub async fn analyze_vulnerabilities(
        &self,
        target: &str,
    ) -> Result<VulnerabilityResults, Box<dyn Error>> {
        println!("analyzing vulnerabilities on target: {}", target);
        todo!()
    }
}
