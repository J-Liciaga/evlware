use crate::modules::vulnerability::{VulnScanner, SqlInjectionScanner};
use crate::models::{Target, Vulnerability};
use crate::utils::http_client::HttpClient;

pub struct VulnerabilityScanner {
    scanner: VulnScanner,
}

impl VulnerabilityScanner {
    pub fn new() -> Self {
        let mut scanner = VulnScanner::new();

        scanner.add_check(Box::new(SqlInjectionScanner));

        VulnerabilityScanner{ scanner }
    }

    pub async fn scan(
        &self,
        target: &Target,
    ) -> Vec<Vulnerability> {
        let client = HttpClient::new();

        self.scanner.scan(
            target,
            &client,
        ).await
    }
}
