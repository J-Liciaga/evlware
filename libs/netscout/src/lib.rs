pub mod core;
pub mod models;

use crate::core::scanners::EVLScanner;
use crate::models::results::ScanResults;
pub use models::common::{Vulnerability, Severity};

pub async fn scan_ports(
    target: &str, 
    start_port: u16,
    end_port: u16
) ->  Result<ScanResults, Box<dyn std::error::Error>> {
    let mut scanner = EVLScanner::new(target).await;

    scanner.set_port_range(start_port, end_port);
    
    Ok(scanner.scan().await?)
}
