use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub struct PortScanner {
    target: String,
    timeout: Duration,
    port_range: (u16, u16),
}

impl PortScanner {
    pub fn new(
        target: &str
    ) -> Self {
        PortScanner {
            target: target.to_string(),
            timeout: Duration::from_secs(1),
            port_range: (1, 1024),
        }
    }

    pub fn set_timeout(
        &mut self,
        timeout: Duration
    ) {
        self.timeout = timeout;
    }

    pub fn set_port_range(
        &mut self,
        start: u16,
        end: u16,
    ) {
        self.port_range = (start, end);
    }

    pub async fn scan(
        &self,
    ) -> Vec<u16> {
        println!(
            "scanning ports for target: {}, from port {} to {}",
            self.target,
            self.port_range.0,
            self.port_range.1,
        );

        (self.port_range.0..=self.port_range.1)
            .filter(|&port| self.is_port_open(port))
            .collect()
    }

    fn is_port_open(
        &self,
        port: u16
    ) -> bool {
        let addr = format!("{}:{}", self.target, port);
        addr.to_socket_addrs()
            .map(|mut addrs| addrs.any(|addr| TcpStream::connect_timeout(&addr, self.timeout).is_ok()))
            .unwrap_or(false)
    }
}