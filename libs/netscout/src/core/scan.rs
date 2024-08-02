use crate::models::noise::NoiseLevel;
use crate::models::results::ScanResults;
use crate::models::target::Target;
use crate::modules::firewall::firewall_detector::FirewallDetector;
// use crate::modules::network::port_scan::{PortScanner, PortScanConfig, PortState};
// use crate::modules::web::crawler::WebCrawler;
// use crate::modules::enumeration::subdomain_enumerator::SubdomainEnumerator;
// use crate::modules::vulnerability::vuln_scanner::VulnScanner;
// use crate::models::error::EvlwareError;
use std::time::Duration;

pub struct EvlScannerConfig {
    pub target: Target,
    pub noise: NoiseLevel,
    pub port_range: (u16, u16),
    pub port_timeout: Duration,
    pub web_timeout: Duration,
    pub max_concurrent_port_scans: usize,
    pub max_concurrent_web_scans: usize,
    pub firewall_detection: bool,
    pub firewall_timeout: Duration,
}

pub async fn scan(
   config: &EvlScannerConfig,
) -> Result<ScanResults, Box<dyn std::error::Error>> {

    // step 1: firewall scan
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

    // step 2: scan ports
    // let port_config = PortScanConfig {
    //     target: target.clone(),
    //     timeout: config.port_timeout,
    //     port_range: config.port_range,
    //     max_concurrent_scans: config.max_concurrent_port_scans,
    // };
    
    // let port_results = scan_ports(&port_config).await;
    // let open_ports: Vec<u16> = port_results
    //     .iter()
    //     .filter_map(|r| 
    //         if r.state == PortState::Open {
    //             Some(r.port)
    //         } else {
    //             None
    //         })
    //     .collect();

    // step 3: enumerate subdomains

    // step 4: web crawl

    // step 5: vulnerability scan

    let open_ports = Vec::new();
    let detected_services = Vec::new();
    let vulnerabilities = Vec::new();


    Ok(ScanResults {
        open_ports,
        detected_services,
        vulnerabilities,
        firewall_profile,
    })
}
