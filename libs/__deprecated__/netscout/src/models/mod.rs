use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub target: String,
    pub vulnerabilities: Vec<Vulnerability>,
    pub exploit_suggestions: Vec<ExploitSuggestion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vulnerability {
    pub name: String,
    pub severity: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExploitSuggestion {
    pub name: String,
    pub description: String,
    pub confidence: f32,
}
