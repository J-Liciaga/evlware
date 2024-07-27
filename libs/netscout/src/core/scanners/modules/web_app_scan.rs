use reqwest::Client;
use reqwest::redirect::Policy;
use regex::Regex;
use std::time::Duration;
use crate::models::common::{Vulnerability, Severity};

pub struct WebAppScanner {
    client: Client,
}

pub struct WebAppScanResult {
    pub url: String,
    pub server_info: String,
    pub vulnerabilities: Vec<Vulnerability>,
    pub detected_services: Vec<String>,
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
        let mut detected_services = Vec::new();

        // check server header
        if let Some(server) = response.headers().get("server") {
            server_info = server.to_str().unwrap_or("Unknown").to_string();
            detected_services.push(format!("Web Server: {}", server_info));
        }

        // check for common frameworks/technologies based on headers
        let headers_to_check = [
            ("X-Powered-By", "Technology"),
            ("X-AspNet-Version", "ASP.NET"),
            ("X-Rails-Version", "Ruby on Rails"),
        ];

        for (header, service) in &headers_to_check {
            if let Some(value) = response.headers().get(*header) {
                detected_services.push(format!("{}:{}", service, value.to_str().unwrap_or("Unknown")));
            }
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

        // check response body for common patters
        let body = response.text().await?;

        // check body for common patterns
        let patterns = [
            (r#"<meta name=\"generator\" content=\"WordPress"#, "WordPress"),
            (r#"Drupal.settings"#, "Drupal"),
            (r#"react.js"#, "React"),
            (r#"next.js"#, "Next"),
            (r#"nuxt.js"#, "Nuxt"),
            (r#"angular.js"#, "Angular"),
            (r#"vue.js"#, "Vue"),
            (r#"remix.js"#, "Remix"),
            (r#"solid.js"#, "Solid"),
        ];

        for (pattern, service) in &patterns {
            if Regex::new(pattern).unwrap().is_match(&body) {
                detected_services.push(service.to_string());
            }
        }

        Ok(WebAppScanResult {
            url: url.to_string(),
            server_info,
            vulnerabilities,
            detected_services,
        })
    }
}
