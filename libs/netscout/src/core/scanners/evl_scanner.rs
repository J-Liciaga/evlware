use crate::models::results::ScanResults;
use super::modules::{PortScanner, WebAppScanner};
use std::time::Duration;

pub struct EVLScanner {
    target: String,
    port_scanner: PortScanner,
    web_app_scanner: WebAppScanner,
}

impl EVLScanner {
    pub async fn new(
        target: &str
    ) -> Self {
        let mut scanner = EVLScanner {
            target: target.to_string(),
            port_scanner: PortScanner::new(target),
            web_app_scanner: WebAppScanner::new(Duration::from_secs(10)).await,
        };
        scanner.set_port_range(80, 443);
        scanner
    }

    pub fn set_port_range(
        &mut self,
        start: u16,
        end: u16,
    ) {
        self.port_scanner.set_port_range(start, end);
    }

    pub async fn scan(
        &self
    ) -> Result<ScanResults, Box<dyn std::error::Error>> {
        let open_ports = self.port_scanner.scan().await;

        println!("Debug: Open ports detected: {:?}", open_ports);

        let mut detected_services = Vec::new();
        let mut vulnerabilities = Vec::new();

        if open_ports.contains(&80) || open_ports.contains(&443) {
            println!("Debug: Found HTTP/HTTPS ports, initiating web app scan");
            let schemes = if open_ports.contains(&443) { vec!["https", "http"] } else { vec!["http"] };

            for scheme in schemes {
                let url = format!("{}://{}", scheme, self.target);

                match self.web_app_scanner.scan(&url).await {
                    Ok(result) => {
                        detected_services.push(format!("Web server at {} ({}) [Services: {:?}]", 
                            result.url, 
                            result.server_info, 
                            result.detected_services
                        ));
                        vulnerabilities.extend(result.vulnerabilities);
                    }
                    Err(e) => return Err(format!("Error scanning {}: {}", url, e).into()),
                }
            }
        } else {
            println!("Debug: No HTTP/HTTPS ports found, skipping web app scan");
        }

        println!("Debug: Final detected_services: {:?}", detected_services);
        println!("Debug: Final vulnerabilities: {:?}", vulnerabilities);

        Ok(ScanResults {
            open_ports,
            detected_services,
            vulnerabilities,
        })        
    }
}
