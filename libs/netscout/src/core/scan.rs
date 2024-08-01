use crate::models::{ScanResult, Target, NoiseLevel};
use crate::modules::network::port_scanner::PortScanner;
use crate::modules::web::crawler::WebCrawler;
use crate::modules::enumeration::subdomain_enumerator::SubdomainEnumerator;
use crate::modules::vulnerability::vuln_scanner::VulnScanner;
use crate::utils::error::EvlwareError;

pub struct Scanner {
    noise_level: NoiseLevel,
    port_scanner: PortScanner,
    web_crawler: WebCrawler,
    subdomain_enumerator: SubdomainEnumerator,
    vuln_scanner: VulnScanner,
}

impl Scanner {
    pub fn new(
        noise_level: NoiseLevel,
    ) -> Self {
        Scanner {
            noise_level,
            port_scanner: PortScanner::new(noise_level),
            web_crawler: WebCrawler::new(noise_level),
            subdomain_enumerator: SubdomainEnumerator:new(noise_level),
            vuln_scanner: VulnScanner::new(noise_level),
        }
    }

    pub async fn scan(
        &self,
        target: Target,
    ) -> Result<ScanResult, EvlwareError> { 
        let mut scan_result = ScanResult::new(target.clone());
        let mut firewall_profile = None;

        // step 1: firewall scan
        if config.firewall_detection {
            match FirewallDetector::new(
                config.target.clone(),
                config.firewall_timeout.as_millis() as u64,
            ) {
                Ok(firewall_detector) => {
                    firewall_profile = Some(firewall_detector.detect().await);
                },
                Err(e) => {
                    println!(
                        "Error creating Firewall Detector: {}",
                        e
                    );
                }
            }
        }

        // step 2: port scan
        let port_config = PortScanConfig {
            target: config.target.clone(),
            timeout: config.port_timeout,
            port_range: config.port_range,
            max_concurrent_scans: config.max_concurrent_port_scans,
        };

        let open_ports = port_scanner(&target).await?;

        scan_result.add_open_ports(open_ports);

        // step 3: subdomain enumeration
        if let Target::Domain(domain) = &target {
            let subdomains = self.subdomain_enumerator.enumerate(domain).await?;

            scan_result.add_subdomains(subdomains);
        }

        // step 4: web crawling
        let crawl_results = self.web_crawler.crawl(&target).await?;

        scan_result.add_crawl_results(crawl_results);

        // step 5: vulnerability scanning
        let vulnerabilities = self.vuln_scanner.scan(&target, &crawl_results).await?;

        scan_result.add_vulnerabilities(vulnerabilities);

        Ok(ScanResults {
            open_ports,
            detected_services,
            vulnerabilities,
            fireall_profile,
        })
    }
}
