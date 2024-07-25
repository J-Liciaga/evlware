use serde::{Serialize, Deserialize};
use thiserror::Error;
use super::common::Severity;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum NetworkError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Network timeout: {0}")]
    Timeout(String),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    #[error("SSL/TLS error: {0}")]
    SSLError(String),
    #[error("DNS resolution failed: {0}")]
    DNSError(String),
}

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum InvalidTargetError {
    #[error("Malformed URL: {0}")]
    MalformedURL(String),
    #[error("Unsupported protocol: {0}")]
    UnsupportedProtocol(String),
    #[error("Invalid IP address: {0}")]
    InvalidIPAddress(String),
    #[error("Host not found: {0}")]
    HostNotFound(String),
}

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum ScanError {
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
    #[error("Invalid target: {0}")]
    InvalidTarget(#[from] InvalidTargetError),
    #[error("Scan timeout")]
    Timeout,
    #[error("Enumeration failed: {0}")]
    EnumerationFailed(String),
    #[error("Vulnerability analysis error: {0}")]
    VulnerabilityAnalysis(#[from] VulnerabilityAnalysisError),
    #[error("Insufficient permissions: {0}")]
    InsufficientPermissions(String),
}

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum VulnerabilityAnalysisError {
    #[error("Scan failed: {0}")]
    ScanFailed(String),
    #[error("Result parsing failed: {0}")]
    ResultParsingFailed(String),
    #[error("Unknown vulnerability: {0} (Severity: {1:?})")]
    UnknownVulnerability(String, Severity),
    #[error("Vulnerability database error: {0}")]
    DatabaseError(String),
}

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum Error {
    #[error("Scan error: {0}")]
    Scan(#[from] ScanError),
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
    #[error("Invalid target error: {0}")]
    InvalidTarget(#[from] InvalidTargetError),
    #[error("Vulnerability analysis error: {0}")]
    VulnerabilityAnalysis(#[from] VulnerabilityAnalysisError),
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl Error {
    pub fn is_network_error(
        &self,
    ) -> bool {
        matches!(
            self,
            Error::Network(_) |
            Error::Scan(ScanError::Network(_))
        )
    }

    pub fn is_target_error(
        &self,
    ) -> bool {
        matches!(
            self,
            Error::InvalidTarget(_) |
            Error::Scan(ScanError::InvalidTarget(_))
        )
    }
}
