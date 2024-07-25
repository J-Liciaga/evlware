pub mod core;
pub mod models;

use crate::core::scanners::EVLScanner;
use crate::models::results::ScanResults;

pub fn scan_ports(target: &str, start_port: u16, end_port: u16) -> ScanResults {
    let mut scanner = EVLScanner::new(target);

    scanner.set_port_range(start_port, end_port);
    
    scanner.scan()
}

pub fn quick_scan(target: &str, port: u16) -> bool {
    let scanner = EVLScanner::new(target);
    
    scanner.scan_single_port(port)
}
