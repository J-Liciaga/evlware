use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct ZapClient {
    client: Client,
    base_url: Url,
    api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZapScanResult {
    alerts: Vec<ZapAlert>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZapAlert {
    risk: String,
    name: String,
    description: String,
}

impl ZapClient {
    pub fn new(
        base_url: &str,
        api_key: &str,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(ZapClient {
            client: Client::new(),
            base_url: Url::parse(base_url)?,
            api_key: api_key.to_string(),
        })
    }

    pub async fn start_scan(
        &self,
        target: &str
    ) -> Result<String, Box<dyn Error>> {
        let url = self.base_url.join("JSON/spider/action/scan/")?;
        let params = [("apiKey", &self.api_key), ("url", target)];

        let response = self.client.get(url).query(&params).send().await?;
        let scan_id: String = response.json().await?;

        Ok(scan_id)
    }

    pub async fn get_scan_results(
        &self,
        scan_id: &str
    ) -> Result<String, Box<dyn Error>> {
        let url = self.base_url.join("JSON/spider/view/status/")?;
        let params = [("apiKey", &self.api_key), ("scanId", scan_id)];

        let response = self.client.get(url).query(&params).send().await?;
        let status: String = response.json().await?;

        Ok(status)
    }

    pub async fn get_alerts(
        &self, 
        target: &str
    ) -> Result<ZapScanResult, Box<dyn Error>> {
        let url = self.base_url.join("JSON/alert/view/alerts/")?;
        let params = [("apikey", &self.api_key), ("baseurl", target)];
        
        let response = self.client.get(url).query(&params).send().await?;
        let alerts: ZapScanResult = response.json().await?;
        Ok(alerts)
    }
}
