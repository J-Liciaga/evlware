use reqwest::Client;
use reqwest::redirect::Policy;
use std::time::Duration;
use crate::models::common::{Vulnerability, Severity};

pub struct WebAppScanner {
    client: Client,
}

pub struct WebAppScanResult {
    pub url: String,
    pub server_info: String,
    pub vulnerabilities: Vec<Vulnerability>,
}

impl WebAppScanner {
    pub async fn new(timeout: Duration) -> Self {
        WebAppScanner {
            client: Client::builder()
                .timeout(timeout)
                .redirect(Policy::limited(5))
                .build()
                .unwrap(),
        }
    }

    pub async fn scan(&self, url: &str) -> Result<WebAppScanResult, reqwest::Error> {
        println!("Scanning web application at {}, please wait...", url);
        
        let response = self.client.get(url).send().await?;
        
        let mut server_info = "Unknown server".to_string();
        let mut vulnerabilities = Vec::new();

        if let Some(server) = response.headers().get("server") {
            server_info = server.to_str().unwrap_or("Unknown").to_string();
        }

        let security_headers = [
            "Strict-Transport-Security",
            "Content-Security-Policy",
            "X-Frame-Options",
            "X-XSS-Protection",
            "X-Content-Type-Options",
        ];

        for header in &security_headers {
            if !response.headers().contains_key(*header) {
                vulnerabilities.push(Vulnerability::new(
                    format!("Missing {}", header),
                    Severity::Medium,
                    format!("The security header {} is missing", header)
                ));
            }
        }

        if url.starts_with("http://") {
            vulnerabilities.push(Vulnerability::new(
                "Insecure HTTP".to_string(),
                Severity::High,
                "The site is using HTTP instead of HTTPS".to_string()
            ));
        }

        Ok(WebAppScanResult {
            url: url.to_string(),
            server_info,
            vulnerabilities,
        })
    }
}
