use std::net::{IpAddr};
use std::time::{Duration, Instant};
use pnet::packet::tcp::MutableTcpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::transport::transport_channel;
use dns_lookup::lookup_host;

pub async fn send_tcp_null(
    host: &str, 
    port: u16, 
    timeout: Duration
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

    tcp_packet.set_source(12346); // Use an arbitrary source port
    tcp_packet.set_destination(port);
    tcp_packet.set_flags(0); // No flags set for NULL packet
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

    // For NULL packets, we don't expect a response, so we'll just wait for a short time
    tokio::time::sleep(timeout).await;

    let duration = start.elapsed();

    // We consider it a success if we were able to send the packet without error
    Ok(duration)
}
