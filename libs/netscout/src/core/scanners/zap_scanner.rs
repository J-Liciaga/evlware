use crate::integrations::zap::ZapClient;
use crate::models::ScanResult;
use std::error::Error;

pub struct ZapScanner {
    client: ZapClient,
}

impl ZapScanner {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let client = ZapClient::new("http://localhost:8080", "API_KEY_HERE")?;

        Ok(ZapScanner { client })
    }

    pub async fn scan(
        &self,
        target: &str
    ) -> Result<ScanResult, Box<dyn Error>> {
        let scan_id = self.client.start_scan(target).await?;

        // poll for scan completion

        loop {
            let status = self.client.get_scan_status(&scan_id).await?;
            
            if status == "100" {
                break;
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }

        let zap_results = self.client.get_alerts(target).await?;

        // convert ZAP results to ScanResult format
        let scan_result = ScanResult {
            vulnerabilitie: zap_results.alerts.into_iter().map(|alert| {
               // convert ZapAlert to Vulnerability model 
                Vulnerability {
                    name: alert.name,
                    severity: alert.rist,
                    description: alert.description,
                }
            }).collect(),
        };
        
        Ok(scan_result)
    }
}