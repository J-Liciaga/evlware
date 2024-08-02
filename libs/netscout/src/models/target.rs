use crate::models::error::EvlwareError;
use std::net::{IpAddr, ToSocketAddrs};
use url::Url;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Target {
    pub url: Option<Url>,
    pub ip: Option<IpAddr>,
    pub hostname: Option<String>,
    pub port: Option<u16>,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Target {
    /// Create a new Target from a string, which could be a URL, IP address, or hostname
    pub fn new(target: &str) -> Result<Self, EvlwareError> {
        if let Ok(url) = Url::parse(target) {
            return Ok(Self::from_url(url));
        }

        if let Ok(ip) = target.parse::<IpAddr>() {
            return Ok(Self::from_ip(ip));
        }

        // If it's not a URL or IP, treat it as a hostname
        Ok(Self::from_hostname(target.to_string()))
    }

    /// Create a Target from a URL
    pub fn from_url(url: Url) -> Self {
        let hostname = url.host_str().map(String::from);
        let port = url.port();
        let ip = hostname.as_ref()
            .and_then(|h| h.to_socket_addrs().ok())
            .and_then(|mut addrs| addrs.next())
            .map(|socket_addr| socket_addr.ip());

        Target {
            url: Some(url),
            ip,
            hostname,
            port,
        }
    }

    /// Create a Target from an IP address
    pub fn from_ip(ip: IpAddr) -> Self {
        Target {
            url: None,
            ip: Some(ip),
            hostname: None,
            port: None,
        }
    }

    /// Create a Target from a hostname
    pub fn from_hostname(hostname: String) -> Self {
        Target {
            url: None,
            ip: None,
            hostname: Some(hostname),
            port: None,
        }
    }

    /// Set the port for this Target
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Validate the Target
    pub fn validate(&self) -> Result<(), EvlwareError> {
        if self.url.is_none() && self.ip.is_none() && self.hostname.is_none() {
            return Err(EvlwareError::UnexpectedError("Target must have either a URL, IP, or hostname".to_string()));
        }
        Ok(())
    }

    /// Attempt to resolve the hostname to an IP address if not already present
    pub fn resolve(&mut self) -> Result<(), EvlwareError> {
        if self.ip.is_none() {
            if let Some(hostname) = &self.hostname {
                let ip = hostname.to_socket_addrs()?
                    .next()
                    .ok_or_else(|| EvlwareError::UnexpectedError("Failed to resolve hostname".to_string()))?
                    .ip();
                self.ip = Some(ip);
            } else if let Some(url) = &self.url {
                if let Some(hostname) = url.host_str() {
                    let ip = hostname.to_socket_addrs()?
                        .next()
                        .ok_or_else(|| EvlwareError::UnexpectedError("Failed to resolve URL hostname".to_string()))?
                        .ip();
                    self.ip = Some(ip);
                }
            }
        }
        Ok(())
    }

    /// Get a string representation of the target
    pub fn to_string(&self) -> String {
        if let Some(url) = &self.url {
            url.to_string()
        } else if let Some(ip) = &self.ip {
            if let Some(port) = self.port {
                format!("{}:{}", ip, port)
            } else {
                ip.to_string()
            }
        } else if let Some(hostname) = &self.hostname {
            if let Some(port) = self.port {
                format!("{}:{}", hostname, port)
            } else {
                hostname.clone()
            }
        } else {
            "Invalid Target".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_from_url() {
        let target = Target::new("https://www.example.com").unwrap();
        assert!(target.url.is_some());
        assert_eq!(target.url.unwrap().host_str(), Some("www.example.com"));
        assert!(target.hostname.is_some());
        assert_eq!(target.hostname.unwrap(), "www.example.com");
    }

    #[test]
    fn test_target_from_ip() {
        let target = Target::new("192.168.1.1").unwrap();
        assert!(target.ip.is_some());
        assert_eq!(target.ip.unwrap().to_string(), "192.168.1.1");
    }

    #[test]
    fn test_target_from_hostname() {
        let target = Target::new("example.com").unwrap();
        assert!(target.hostname.is_some());
        assert_eq!(target.hostname.unwrap(), "example.com");
    }

    #[test]
    fn test_target_with_port() {
        let target = Target::new("example.com").unwrap().with_port(8080);
        assert_eq!(target.port, Some(8080));
    }

    #[test]
    fn test_target_to_string() {
        let target = Target::new("https://www.example.com").unwrap();
        assert_eq!(target.to_string(), "https://www.example.com/");

        let target = Target::new("192.168.1.1").unwrap().with_port(8080);
        assert_eq!(target.to_string(), "192.168.1.1:8080");

        let target = Target::new("example.com").unwrap().with_port(443);
        assert_eq!(target.to_string(), "example.com:443");
    }
}