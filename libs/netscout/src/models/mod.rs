use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanConfig {
    pub target: String,
    pub scan_type: ScanType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ScanType {
    Quick,
    Full,
    Custom,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub vulnerabilities: Vec<Vulnerability>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vulnerability {
    pub name: String,
    pub severity: String,
    pub description: String,
}
