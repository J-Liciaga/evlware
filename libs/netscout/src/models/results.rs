use serde::{Serialize, Deserialize};
use super::vulnerability::Vulnerability;
use super::firewall::FirewallProfile;

#[derive(Serialize, Deserialize, Debug)]
pub struct ScanResults {
    pub open_ports: Vec<u16>,
    pub detected_services: Vec<String>,
    pub vulnerabilities: Vec<Vulnerability>,
    pub firewall_profile: Option<FirewallProfile>,
}
