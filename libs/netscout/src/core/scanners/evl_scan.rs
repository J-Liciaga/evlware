use crate::models::results::ScanResults;
use super::modules::firewall_scan::FirewallDetector;
use super::modules::port_scan::{PortScanConfig, scan_ports, PortState};
use super::modules::web_app_scan::{WebAppScanConfig, scan_web_app};
use std::time::Duration;

pub struct EVLScannerConfig {
    pub target: String,
    pub port_range: (u16, u16),
    pub port_timeout: Duration,
    pub web_timeout: Duration,
    pub max_concurrent_port_scans: usize,
    pub max_concurrent_web_scans: usize,
    pub mode: String,
    pub firewall_detection: bool,
    pub firewall_timeout: Duration,
}

pub async fn scan(
    config: &EVLScannerConfig
) -> Result<ScanResults, Box<dyn std::error::Error>> {
    let mut firewall_profile = None;

    if config.firewall_detection {
       match FirewallDetector::new(
            config.target.clone(),
            config.firewall_timeout.as_millis() as u64,
        ) {
            Ok(firewall_detector) => {
                firewall_profile = Some(firewall_detector.detect().await);
            },
            Err(e) => {
                println!("Error creating Firewall Detector: {}", e);
            }
        }
    }

     // Port scanning
     let port_config = PortScanConfig {
        target: config.target.clone(),
        timeout: config.port_timeout,
        port_range: config.port_range,
        max_concurrent_scans: config.max_concurrent_port_scans,
    };
    let port_results = scan_ports(&port_config).await;
    let open_ports: Vec<u16> = port_results
        .iter()
        .filter_map(|r| 
            if r.state == PortState::Open {
                Some(r.port)
            } else {
                None
            })
        .collect();

    // Web application scanning
    let web_config = WebAppScanConfig {
        timeout: config.web_timeout,
        max_concurrent_scans: config.max_concurrent_web_scans,
    };
    let mut detected_services = Vec::new();
    let mut vulnerabilities = Vec::new();
    if open_ports.contains(&80) || open_ports.contains(&443) {
        let schemes = if open_ports.contains(&443) {
            vec!["https"]
        } else {
            vec!["http"]
        };
        let urls: Vec<String> = schemes.into_iter()
            .map(|scheme| format!(
                "{}://{}",
                scheme,
                config.target,
            ))
            .collect();
        
        let web_results = scan_web_app(&web_config, urls).await;
        for result in web_results {
            match result {
                Ok(scan_result) => {
                    detected_services.extend(scan_result.detected_services);
                    vulnerabilities.extend(scan_result.vulnerabilities);
                }
                Err(e) => println!("Error Scanning: {}", e),
            }
        }
    }


    Ok(ScanResults {
        open_ports,
        detected_services,
        vulnerabilities,
        firewall_profile,
    })
}
