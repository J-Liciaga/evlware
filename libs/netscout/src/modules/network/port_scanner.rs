use std::time::{Duration, Instant};
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::net::TcpStream;
use tokio::time::timeout;
use futures::stream::{self, StreamExt};
use crate::models::noise::NoiseLevel;

#[derive(Debug, Clone)]
pub struct PortScanConfig {
    pub noise_level: NoiseLevel,
    pub target: String, 
    pub timeout: Duration,
    pub port_range: (u16, u16),
    pub max_concurrent_scans: usize, 
}

#[derive(Debug)]
pub struct PortScanResult {
    pub port: u16,
    pub state: PortState,
    pub response_time: Duration,
}

#[derive(Debug, PartialEq)]
pub enum PortState {
    Open,
    Closed,
    Filtered,
}

pub async fn port_scanner(
    config: &PortScanConfig,
) -> Vec<PortScanResult> {
    let semaphore = Arc::new(
        Semaphore::new(
            config.max_concurrent_scans,
        )
    );

    let ports_to_scan = match config.noise_level {
        NoiseLevel::Stealthy => vec![80, 443],
        NoiseLevel::Moderate => (1..1000).collect(),
        NoiseLevel::Aggressive => (1..65535).collect(),
    };

    let results: Vec<PortScanResult> = stream::iter(ports_to_scan)
        .map(|port| {
            let sem_clone = semaphore.clone();
            let config_clone = config.clone();

            async move {
                let _permit = sem_clone
                    .acquire()
                    .await
                    .unwrap();

                check_port(
                    config_clone,
                    port,
                ).await
            }
        })
        .buffer_unordered(
            config.max_concurrent_scans
        )
        .collect()
        .await;


    let filtered_count = results
        .iter()
        .filter(|r| matches!(
            r.state,
            PortState::Filtered
        ))
        .count();

    if filtered_count > (config.port_range.1 - config.port_range.0) as usize / 2 {
        println!("Firewall Detected: More than half of the ports seem to be filtered.");
    }

    results
}

async fn check_port(
    config: PortScanConfig,
    port: u16,
) -> PortScanResult {
    let addr = format!(
        "{}:{}",
        config.target,
        port,
    );

    let start_time = Instant::now();

    let state = match addr.to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.next() {
                match timeout(
                    config.timeout,
                    TcpStream::connect(&addr)
                ).await {
                    Ok(Ok(_)) => PortState::Open,
                    Ok(Err(_)) => PortState::Closed,
                    Err(_) => PortState::Filtered,
                }
            } else {
                PortState::Filtered
            }
        },
        Err(_) => PortState::Filtered,
    };

    PortScanResult {
        port,
        state,
        response_time: start_time.elapsed(),
    }
}
