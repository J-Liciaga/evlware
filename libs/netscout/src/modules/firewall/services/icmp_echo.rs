use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::{Duration, Instant};
use pnet::packet::icmp::echo_request::{MutableEchoRequestPacket, IcmpCodes};
use pnet::packet::icmp::{IcmpTypes, IcmpPacket};
use pnet::packet::icmpv6::Icmpv6Types;
use pnet::packet::icmpv6::echo_request::MutableEchoRequestPacket as Icmpv6EchoRequestPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;
use pnet::transport::{icmp_packet_iter, transport_channel, icmpv6_packet_iter};
use pnet::transport::TransportChannelType::Layer4;
use pnet::transport::TransportProtocols::{Ipv4, Ipv6};

/**
 * Description:
 * 1. parses the host string into an Ipv4Addr
 * 2. creates a transport channel for sending and receiving ICMP packets
 * 3. creates an ICMP Echo request packet
 * 4. sends the packet to the destination IP
 * 5. waits for a response, timing out after the specified duration
 * 6. if a response is received, it checks if it's an Echo Reply from the correct address
 * 7. returns the round-trip time if successful, or an error if it times out or fails
 * 
 * Notes:
 * 1. this fn requires root admin priviledges due to its use of raw sockets
 * 2. some systesm may block or filter ICMP traffic for security reasons, this test may not always be conclusive
 */

 pub async fn send_icmp_echo(
    host: &str,
    timeout: Duration,
) -> Result<Duration, std::io::Error> {
    let dest_ip: IpAddr = host.parse()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

    match dest_ip {
        IpAddr::V4(ipv4) => send_icmpv4_echo(
            ipv4,
            timeout,
        ).await,
        IpAddr::V6(ipv6) => send_icmpv6_echo(
            ipv6,
            timeout,
        ).await,
    }
} 

async fn send_icmpv4_echo(
    dest_ip: Ipv4Addr,
    timeout: Duration,
) -> Result<Duration, std::io::Error> {
    // create a new channel, dealing with layer 4 packets
    let (mut tx, mut rx) = transport_channel(4096, Layer4(Ipv4(IpNextHeaderProtocols::Icmp)))
        .map__err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut echo_packet = [0u8; 64];
    let mut echo_packet = MutableEchoRequestPacket::new(&mut echo_packet)
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Failed to create echo packet"))?;

    echo_packet.set_icmp_type(IcmpTypes::EchoRequest);
    echo_packet.set_icmp_code(IcmpCodes::NoCode);
    echo_packet.set_sequence_number(1);
    echo_packet.set_identifier(1);

    let start_time = Instant::now();

    tx.send_to(echo_packet, IpAddr::V4(dest_ip))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut iter = icmp_packet_iter(&mut rx);

    let timeout_future = tokio::time::sleep(timeout);

    tokio::pin!(timeout_future);

    loop {
        tokio::select! {
            _ = &mut timeout_future => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "ICMP echo timed out",
                ));
            }
            packet = tokio::task::spawn_blocking(move || iter.next()) => {
                if let Ok(Some((packet, addr))) = packet {
                    if addr == IpAddr::V4(dest_ip) {
                        let icmp_packet = IcmpPacket::new(packet.packet()).unwrap();

                        if icmp_packet.get_icmp_type() == IcmpTypes::EchoReply {
                            return Ok(start_time.elapsed());
                        }
                    }
                }
            }
        }
    }
}

async fn send_icmpv6_echo(
    dest_ip: Ipv6Addr,
    timeout: Duration,
) -> Result<Duration, std::io::Error> {
    let (mut tx, mut rx) = transport_channel(4096,Layer4(Ipv6(IpNextHeaderProtocols::icmpv6)))
        .map__err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut echo_packet = [0u8; 64];
    let mut echo_packet = Icmpv6EchoRequestPacket::new(&mut echo_packet)
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Failed to create echo packet"))?;

    echo_packet.set_icmpv6_type(Icmpv6Types::EchoRequest);
    echo_packet.set_icmpv6_code(IcmpCodes::NoCode);
    echo_packet.set_sequence_number(1);
    echo_packet.set_identifier(1);

    let start_time = Instant::now();

    tx.send_to(echo_packet, IpAddr::V6(dest_ip))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut iter = icmpv6_packet_iter(&mut rx);

    let timeout_future = tokio::time::sleep(timeout);

    tokio::pin!(timeout_future);

    loop {
        tokio::select! {
            _ = &mut timeout_future => {
                return Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "ICMPv6 echo timed out"));
            }
            packet = tokio::task::spawn_blocking(move || iter.next()) => {
                if let Ok(Some((packet, addr))) = packet {
                    if addr = IpAddr::V6(dest_ip) {
                        let icmpv6_packet = pnet::packet::icmpv6::Icmpv6Packet::new(packet.packet()).unwrap();

                        if icmpv6_packet.get_icmpv6_type() == Icmpv6Types::EchoReply {
                            return Ok(start_time.elapsed());
                        }
                    }
                }
            }
        }
    }
}
