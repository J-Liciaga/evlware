use std::fmt;
use std::error::Error;
use std::io;
use reqwest;
use std::net::AddrParseError;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum EvlwareError {
    // IO errors
    Io(io::Error),
    
    // Network errors
    NetworkConnection(String),
    HttpRequest(reqwest::Error),
    
    // Parsing errors
    InvalidIpAddress(AddrParseError),
    InvalidPort(ParseIntError),
    InvalidUrl(url::ParseError),
    
    // Scanning errors
    PortScanFailed(String),
    ServiceDetectionFailed(String),
    
    // Web application errors
    CrawlFailed(String),
    FormAnalysisFailed(String),
    
    // Enumeration errors
    SubdomainEnumerationFailed(String),
    DirectoryEnumerationFailed(String),
    
    // Vulnerability scanning errors
    VulnerabilityScanFailed(String),
    
    // Configuration errors
    ConfigParseError(String),
    
    // Database errors
    DatabaseConnectionFailed(String),
    DatabaseQueryFailed(String),
    
    // External library errors
    ExternalLibraryError(String),
    
    // Generic errors
    UnexpectedError(String),
}

impl fmt::Display for EvlwareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvlwareError::Io(err) => write!(f, "IO error: {}", err),
            EvlwareError::NetworkConnection(msg) => write!(f, "Network connection error: {}", msg),
            EvlwareError::HttpRequest(err) => write!(f, "HTTP request error: {}", err),
            EvlwareError::InvalidIpAddress(err) => write!(f, "Invalid IP address: {}", err),
            EvlwareError::InvalidPort(err) => write!(f, "Invalid port number: {}", err),
            EvlwareError::InvalidUrl(err) => write!(f, "Invalid URL: {}", err),
            EvlwareError::PortScanFailed(msg) => write!(f, "Port scan failed: {}", msg),
            EvlwareError::ServiceDetectionFailed(msg) => write!(f, "Service detection failed: {}", msg),
            EvlwareError::CrawlFailed(msg) => write!(f, "Web crawl failed: {}", msg),
            EvlwareError::FormAnalysisFailed(msg) => write!(f, "Form analysis failed: {}", msg),
            EvlwareError::SubdomainEnumerationFailed(msg) => write!(f, "Subdomain enumeration failed: {}", msg),
            EvlwareError::DirectoryEnumerationFailed(msg) => write!(f, "Directory enumeration failed: {}", msg),
            EvlwareError::VulnerabilityScanFailed(msg) => write!(f, "Vulnerability scan failed: {}", msg),
            EvlwareError::ConfigParseError(msg) => write!(f, "Configuration parse error: {}", msg),
            EvlwareError::DatabaseConnectionFailed(msg) => write!(f, "Database connection failed: {}", msg),
            EvlwareError::DatabaseQueryFailed(msg) => write!(f, "Database query failed: {}", msg),
            EvlwareError::ExternalLibraryError(msg) => write!(f, "External library error: {}", msg),
            EvlwareError::UnexpectedError(msg) => write!(f, "Unexpected error occurred: {}", msg),
        }
    }
}

impl Error for EvlwareError {}

// Implement From traits for easy conversion from other error types
impl From<io::Error> for EvlwareError {
    fn from(err: io::Error) -> EvlwareError {
        EvlwareError::Io(err)
    }
}

impl From<reqwest::Error> for EvlwareError {
    fn from(err: reqwest::Error) -> EvlwareError {
        EvlwareError::HttpRequest(err)
    }
}

impl From<AddrParseError> for EvlwareError {
    fn from(err: AddrParseError) -> EvlwareError {
        EvlwareError::InvalidIpAddress(err)
    }
}

impl From<ParseIntError> for EvlwareError {
    fn from(err: ParseIntError) -> EvlwareError {
        EvlwareError::InvalidPort(err)
    }
}

impl From<url::ParseError> for EvlwareError {
    fn from(err: url::ParseError) -> EvlwareError {
        EvlwareError::InvalidUrl(err)
    }
}

// Helper function to create UnexpectedError
pub fn unexpected_error(msg: &str) -> EvlwareError {
    EvlwareError::UnexpectedError(msg.to_string())
}
