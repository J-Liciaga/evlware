use crate::models::common::{Vulnerability, Severity};
use reqwest::{Client, header::HeaderValue};
use reqwest::redirect::Policy;
use std::time::Duration;
use regex::Regex;
use tokio::sync::Semaphore;
use std::sync::Arc;
use futures::stream::{self, StreamExt};

#[derive(Clone)]
pub struct WebAppScanConfig {
    pub timeout: Duration,
    pub max_concurrent_scans: usize,
}

pub struct WebAppScanResult {
    pub url: String,
    pub server_info: String,
    pub detected_services: Vec<String>,
    pub vulnerabilities: Vec<Vulnerability>,
}

pub async fn scan_web_app(
    config: &WebAppScanConfig,
    urls: Vec<String>,
) -> Vec<Result<WebAppScanResult, reqwest::Error>> {
    let semaphore = Arc::new(
        Semaphore::new(
            config.max_concurrent_scans
        )
    );

    let client = Arc::new(
        Client::builder()
            .timeout(config.timeout)
            .redirect(Policy::limited(5))
            .build()
            .unwrap()
    );

    let results = stream::iter(urls)
        .map(|url: String| {
            let sem_clone = semaphore.clone();
            let client_clone = client.clone();
            let config_clone = config.clone();

            async move {
                let _permit = sem_clone.acquire().await.unwrap();

                scan_single_web_app(
                    config_clone,
                    &url,
                    &client_clone,
                ).await
            }
        })
        .buffer_unordered(config.max_concurrent_scans)
        .collect::<Vec<_>>()
        .await;

    results
}

async fn scan_single_web_app(
    _config: WebAppScanConfig,
    url: &str,
    client: &Client
) -> Result<WebAppScanResult, reqwest::Error> {
    // let client = Client::builder()
    //     .timeout(config.timeout)
    //     .redirect(Policy::limited(5))
    //     .build()?;

    let response = client
        .get(url)
        .send()
        .await?;
    let _status = response
        .status();
    let headers = response
        .headers()
        .clone();

    let mut server_info = "Unknown server".to_string();
    let mut detected_services = Vec::new();
    let mut vulnerabilities = Vec::new();

    if let Some(server) = response
        .headers()
        .get("server") {
            server_info = header_value_to_string(server);

            detected_services
                .push(
                    format!(
                        "Web Server: {}",
                        server_info,
                    ),
                );
        }
    
    check_headers(
        &headers,
        &mut detected_services,
    );

    let body = response.text().await?;

    check_body(
        &body,
        &mut detected_services,
    );

    check_security_headers(
        &headers,
        &mut vulnerabilities,
    );

    if url.starts_with("http://") {
        vulnerabilities.push(
            Vulnerability::new(
                "Insecure HTTP".to_string(),
                Severity::High,
                "The site is using HTTP instead of HTTPS".to_string(),
            ),
        );
    }

    Ok( WebAppScanResult {
        url: url.to_string(),
        server_info,
        detected_services,
        vulnerabilities,
    })
}

fn header_value_to_string(
    value: &HeaderValue
) -> String {
    value.to_str().unwrap_or("Unknown").to_string()
}

fn check_headers(headers: &reqwest::header::HeaderMap, detected_services: &mut Vec<String>) {
    let headers_to_check = [
        ("X-Powered-By", "Technology"),
        ("X-AspNet-Version", "ASP.NET"),
        ("X-Rails-Version", "Ruby on Rails"),
    ];

    for (header, service) in &headers_to_check {
        if let Some(value) = headers.get(*header) {
            detected_services.push(format!("{}: {}", service, header_value_to_string(value)));
        }
    }
}

fn check_security_headers(headers: &reqwest::header::HeaderMap, vulnerabilities: &mut Vec<Vulnerability>) {
    let security_headers = [
        "Strict-Transport-Security",
        "Content-Security-Policy",
        "X-Frame-Options",
        "X-XSS-Protection",
        "X-Content-Type-Options",
    ];

    for header in &security_headers {
        if !headers.contains_key(*header) {
            vulnerabilities.push(Vulnerability::new(
                format!("Missing {}", header),
                Severity::Medium,
                format!("The security header {} is missing", header)
            ));
        }
    }
}

fn check_body(
    body: &str, 
    detected_services: &mut Vec<String>,
) {
    let patterns = [
        (r#"<meta name="generator" content="WordPress"#, "WordPress"),
        (r#"Drupal.settings"#, "Drupal"),
        (r#"<script src="[^"]*jquery"#, "jQuery"),
        (r#"react.js"#, "React"),
        (r#"angular.js"#, "Angular"),
    ];

    for (pattern, service) in &patterns {
        if Regex::new(pattern)
            .unwrap()
            .is_match(body) {
                detected_services.push(
                    service.to_string()
                );
            }
    }
}
