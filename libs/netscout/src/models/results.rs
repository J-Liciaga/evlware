use serde::{Serialize, Deserialize};
use super::common::Vulnerability;

#[derive(Serialize, Deserialize, Debug)]
pub struct ScanResults {
    pub open_ports: Vec<u16>,
    pub detected_services: Vec<String>,
    pub vulnerabilities: Vec<Vulnerability>,
}
