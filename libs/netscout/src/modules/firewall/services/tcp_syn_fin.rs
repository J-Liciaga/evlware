use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time::timeout;
use pnet::packet::tcp::{MutableTcpPacket, TcpFlags};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::transport::transport_channel;
use dns_lookup::lookup_host;

pub async fn send_tcp_syn_fin(
    host: &str, 
    port: u16, 
    timeout_duration: Duration
) -> Result<Duration, Box<dyn std::error::Error>> {
    // Resolve the hostname to an IP address
    let ip = lookup_host(host)?
        .into_iter()
        .filter(|ip| ip.is_ipv4())
        .next()
        .ok_or("Failed to resolve hostname to IPv4")?;

    let ipv4 = match ip {
        IpAddr::V4(ipv4) => ipv4,
        _ => return Err("Expected IPv4 address".into()),
    };

    // Create a raw socket
    let (mut tx, _) = transport_channel(
        4096,
        pnet::transport::TransportChannelType::Layer4(
            pnet::transport::TransportProtocol::Ipv4(IpNextHeaderProtocols::Tcp),
        ),
    )?;

    // Prepare the TCP packet
    let mut tcp_buffer = [0u8; 20];
    let mut tcp_packet = MutableTcpPacket::new(&mut tcp_buffer).unwrap();

    tcp_packet.set_source(12345); // Use an arbitrary source port
    tcp_packet.set_destination(port);
    tcp_packet.set_flags(TcpFlags::SYN | TcpFlags::FIN);
    tcp_packet.set_window(64240);
    tcp_packet.set_data_offset(5);
    tcp_packet.set_sequence(0);

    // Calculate checksum
    let checksum = pnet::packet::tcp::ipv4_checksum(
        &tcp_packet.to_immutable(),
        &ipv4,
        &ipv4,
    );

    tcp_packet.set_checksum(checksum);

    // Send the packet and measure the time
    let start = Instant::now();

    tx.send_to(tcp_packet, IpAddr::V4(ipv4))?;

    // Wait for a response or timeout
    let result = timeout(
        timeout_duration,
        TcpStream::connect(SocketAddr::new(ip, port))
    ).await;

    let duration = start.elapsed();

    match result {
        Ok(_) => Ok(duration),
        Err(_) => Err("Connection timed out".into()),
    }
}
