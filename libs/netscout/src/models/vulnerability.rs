use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vulnerability {
    pub name: String,
    pub severity: Severity,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl Vulnerability {
    pub fn new(name: String, severity: Severity, description: String) -> Self {
        Vulnerability {
            name,
            severity,
            description,
        }
    }
}
