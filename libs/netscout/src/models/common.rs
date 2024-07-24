use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vulnerability {
    pub name: String,
    pub severity: Severity,
    pub description: String,
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
