use tokio::net::TcpStream;
use std::net::ToSocketAddrs;
use std::time::Duration;
use url::Url;

pub struct PortScanner {
    target: String,
    timeout: Duration,
    port_range: (u16, u16),
}

impl PortScanner {
    pub fn new(
        target: &str
    ) -> Self {
        let hostname = Url::parse(target)
            .map(|url| url.host_str().unwrap_or(target).to_string())
            .unwrap_or_else(|_| target.to_string());

        PortScanner {
            target: hostname,
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

        let mut open_ports = Vec::new();

        for port in self.port_range.0..=self.port_range.1 {
            if self.is_port_open(port).await {
                open_ports.push(port);
            }
        }

        println!("Debug: Open ports found: {:?}", open_ports);
        open_ports

        // (self.port_range.0..=self.port_range.1)
        //     .filter(|&port| self.is_port_open(port))
        //     .collect()
    }

    async fn is_port_open(
        &self,
        port: u16
    ) -> bool {
        let addr = format!("{}:{}", self.target, port);

        match addr.to_socket_addrs() {
            Ok(mut addrs) => {
                if let Some(addr) = addrs.next() {
                    match tokio::time::timeout(
                        self.timeout,
                        TcpStream::connect(&addr)
                    ).await {
                        Ok(Ok(_)) => {
                            println!("Debug: Port {} is open", port);
                            true
                        },
                        _ => false,
                    }
                } else {
                    false
                }
            },
            Err(_) => false,
        }
        // addr.to_socket_addrs()
        //     .map(|mut addrs| addrs.any(|addr| TcpStream::connect_timeout(&addr, self.timeout).is_ok()))
        //     .unwrap_or(false)
    }
}