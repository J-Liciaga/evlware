pub mod scanner;
pub mod enumerator;
pub mod vulnerability_scanner;

use std::error::Error;

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
        todo!()
    }

    pub async fn enumerate(
        &self,
        target: &str,
    ) -> Result<EnumerationResults, Box<dyn Error>> {
        todo!()
    }

    pub asyn fn analyze_vulnerabilities(
        &self,
        target: &str,
    ) -> Result<VulnerabilitiesResults, Box<dyn Error>> {
        todo!()
    }
}

pub struct ScanResults {

}

pub struct EnumerationResults {

}

pub struct VulnerabilitiesResults {

}
