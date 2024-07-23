use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ScanResults {
    pub open_ports: Vec<u16>,
    pub detected_services: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnumerationResults {
    pub directories: Vec<String>,
    pub subdomains: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VulnerabilityResults {
    pub vulnerabilities: Vec<Vulnerability>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vulnerability {
    pub name: String,
    pub severity: String,
    pub description: String,
}
