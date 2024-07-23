use crate::models::{ScanResult, ScanConfig};
use crate::scanners::Scanner;
use async_trait::async_trait;
use anyhow::{Result, Context};
use reqwest::Client;
use serde_json::Value;
use url::Url;

pub struct ZapScanner {
    client: Client,
    api_key: String,
    zap_url: Url,
}

impl ZapScanner {
    pub fn new(zap_url: &str) -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            zap_url: Url::parse(zap_url).context("Invalid ZAP URL")?,
        })
    }

    async fn start_spider(
        &self,
        target: &str,
    ) -> Result<String> {
        let url = self.zap_url.join("JSON/spider/action/scan/")?;
        let params = [(
            "apiKey",
            &self.api_key,
        ), (
            "url",
            &target.to_string()
        )];

        let response: Value = self.client.get(url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

        response["scan"].as_str()
            .map(String::from)
            .context("Failed to start spider scan");
    }

    async fn spider_status(
        &self,
        scan_id: &str,
    ) -> Result<String> {
        let url = self.zap_url.join("JSON/spider/view/status/")?;
        let params = [(
            "apiKey",
            &self.api_key,
        ), (
            "scanId",
            &scan_id.to_string(),
        )];

        let response: Value = self.client.get(url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

        response["status"].as_str()
            .and_then(|s| s.parse().ok())
            .context("Failed to get spider status");
    }

    async fn start_active_scan(
        &self,
        target: &str,
    ) -> Result<String> {
        let url = self.zap_url.join("JSON/ascan/action/scan/")?;
        let params = [("apikey", &self.api_key), ("url", &target.to_string())];
        
        let response: Value = self.client.get(url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

        response["scan"].as_str()
            .map(String::from)
            .context("Failed to start active scan");
    }


    async fn active_scan_status(&self, scan_id: &str) -> Result<u8> {
        let url = self.zap_url.join("JSON/ascan/view/status/")?;
        let params = [("apikey", &self.api_key), ("scanId", &scan_id.to_string())];
        
        let response: Value = self.client.get(url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

        response["status"].as_str()
            .and_then(|s| s.parse().ok())
            .context("Failed to get active scan status");
    }

    async fn get_alerts(&self, target: &str) -> Result<Vec<Vulnerability>> {
        let url = self.zap_url.join("JSON/core/view/alerts/")?;
        let params = [("apikey", &self.api_key), ("baseurl", &target.to_string())];
        
        let response: Value = self.client.get(url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

        let alerts = response["alerts"].as_array()
            .context("Failed to get alerts")?;

        alerts.iter().map(|alert| {
            Ok(Vulnerability {
                name: alert["name"].as_str().context("Missing alert name")?.to_string(),
                severity: alert["risk"].as_str().context("Missing alert risk")?.to_string(),
                description: alert["description"].as_str().context("Missing alert description")?.to_string(),
            })
        }).collect();
    }
}

#[async_trait]
impl Scanner for ZapScanner {
    async fn scan(&self, config: &ScanConfig) -> Result<ScanResult> {
        // Start spider scan
        let spider_id = self.start_spider(&config.target).await?;
        // Wait for spider scan to complete
        while self.spider_status(&spider_id).await? < 100 {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
        // Start active scan
        let scan_id = self.start_active_scan(&config.target).await?;
        // Wait for active scan to complete
        while self.active_scan_status(&scan_id).await? < 100 {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
        // Get alerts (vulnerabilities)
        let vulnerabilities = self.get_alerts(&config.target).await?;

        Ok(ScanResult {
            target: config.target.clone(),
            vulnerabilities,
        });
    }
}
