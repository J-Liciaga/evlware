pub mod evl_scanner;
pub mod zap_scanner;

pub use evl_scanner::EVLScanner;
pub use zap_scanner::ZAPScanner;

use crate::models::{ScanResults, ScanError, Vulnerability, Severity};

pub trait Scanner {
    fn new (config: ScannerConfig) -> Self;
    fn scan(&scan, target: &str) -> ScanResult;
}

pub struct ScannerConfig {
    pub depth: u32,
    pub timeout: std::time::Duration,
}
