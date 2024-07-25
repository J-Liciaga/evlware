use crate::models::results::ScanResults;
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

pub struct EVLScanner {
    target: String,
    timeout: Duration,
    start_port: u16,
    end_port: u16,
}

impl EVLScanner {
    pub fn new(
        target: &str,
    ) -> Self {
        EVLScanner {
            target: target.to_string(),
            timeout: Duration::from_millis(200),
            start_port: 1,
            end_port: 1024,
        }
    }

    pub fn set_port_range(
        &mut self,
        start: u16,
        end: u16,
    ) {
        self.start_port = start;
        self.end_port = end;
    }

    pub fn set_timeout(
        &mut self,
        timeout: Duration,
    ) {
        self.timeout = timeout;
    }

    pub fn scan(
        &self
    ) -> ScanResults {
        println!("running scan, please wait...");
        
        let mut open_ports = Vec::new();
        let mut detected_services = Vec::new();
        let vulnerabilities = Vec::new();
        
        let ip = match self.resolve_host() {
            Some(ip) => ip,
            None => return ScanResults {
                open_ports,
                detected_services,
                vulnerabilities
            },
        };

        for port in self.start_port..=self.end_port {
            let socket = SocketAddr::new(ip, port);

            if TcpStream::connect_timeout(&socket, self.timeout).is_ok() {
                open_ports.push(port);
                // TODO: add logic to detect services and vulnerabilities
                detected_services.push(
                    format!("unknown service on port: {}", port)
                )
                // vulnerabilities.push(
                //     format!("unknown vulnerability on port: {}", port)
                // )
            }
        }

        ScanResults {
            open_ports,
            detected_services,
            vulnerabilities
        }
    }

    pub fn scan_single_port(
        &self, 
        port: u16
    ) -> bool {
        let ip = match self.resolve_host() {
            Some(ip) => ip,
            None => return false,
        };

        let socket = SocketAddr::new(ip, port);
        TcpStream::connect_timeout(&socket, self.timeout).is_ok()
    }

    fn resolve_host(
        &self
    ) -> Option<IpAddr> {
        // if the host is already an ip address, parse and return it
        if let Ok(ip) = self.target.parse::<IpAddr>() {
            return Some(ip);
        }

        // otherwise, try to resolve the hostname
        let socket_addr = format!("{}:80", self.target);

        socket_addr.to_socket_addrs().ok()?.next().map(|addr| addr.ip())
    }
}
