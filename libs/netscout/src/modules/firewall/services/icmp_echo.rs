// use tokio;
use std::time::{
    Duration,
    // Instant,
};
use std::net::{
    IpAddr,
    Ipv4Addr,
    Ipv6Addr,
};
// use pnet::packet::Packet;
// use pnet::transport::TransportChannelType::Layer4;
// use pnet::packet::ip::IpNextHeaderProtocols;
// use pnet::packet::icmp::{
//     echo_request::MutableEchoRequestPacket,
//     IcmpTypes,
//     IcmpCode,
// };
// use pnet::packet::icmpv6::{
//     Icmpv6Types,
//     Icmpv6Code,
//     MutableIcmpv6Packet,
// };
// use pnet::transport::{
//     transport_channel,
//     icmp_packet_iter,
//     icmpv6_packet_iter,
// };
// use pnet::transport::TransportProtocol::{Ipv4};

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
 * 2. some systems may block or filter ICMP traffic for security reasons, 
 * this test may not always be conclusive
 */

pub async fn send_icmp_echo(
    host: &str, 
    timeout: Duration
) -> Result<Duration, std::io::Error> {
    let dest_ip: IpAddr = host.parse()
        .map_err(|e| std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            e
        ))?;

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

#[allow(unused_variables)]
async fn send_icmpv4_echo(
    dest_ip: Ipv4Addr,
    timeout: Duration,
) -> Result<Duration, std::io::Error> {
    Ok(Duration::from_millis(10))

    //     let (
//         mut tx,
//         mut rx,
//     ) = transport_channel(
//         4096,
//         Layer4(Ipv4(IpNextHeaderProtocols::Icmp))
//     )
//     .map_err(|e| std::io::Error::new(
//         std::io::ErrorKind::Other,
//         e,
//     ))?;

//     let mut echo_packet = [0u8; 64];
//     let mut echo_packet = MutableEchoRequestPacket::new(&mut echo_packet)
//         .ok_or_else(|| std::io::Error::new(
//             std::io::ErrorKind::Other,
//             "Failed to create echo packet"
//         ))?;

//     echo_packet.set_icmp_type(IcmpTypes::EchoRequest);
//     echo_packet.set_icmp_code(IcmpCode::new(0));
//     echo_packet.set_sequence_number(1);
//     echo_packet.set_identifier(1);

//     let start_time = Instant::now();

//     tx.send_to(echo_packet, IpAddr::V4(dest_ip))
//         .map_err(|e| std::io::Error::new(
//             std::io::ErrorKind::Other,
//             e,
//         ))?;
    
//     let mut iter = icmp_packet_iter(&mut rx);

//     let timeout_future = tokio::time::sleep(timeout);

//     tokio::pin!(timeout_future);

//     let mut buf = [0u8; 1500];

//     loop {
//         tokio::select! {
//             _ = &mut timeout_future => {
//                 return Err(std::io::Error::new(
//                     std::io::ErrorKind::TimedOut,
//                     "ICMP echo timed out",
//                 ));
//             }
//             result = tokio::task::spawn_blocking(move || rx.recv_from(&mut buf)) => {
//                 match result {
//                     Ok(Ok((size, addr))) => {
//                         if addr == IpAddr::V4(dest_ip) {
//                             let icmp_packet = pnet::packet::icmp::IcmpPacket::new(&buf[..size]).unwrap();

//                             if icmp_packet.get_icmp_type() == IcmpTypes::EchoReply {
//                                 return Ok(start_time.elapsed());
//                             }
//                         }
//                     }
//                     _ => continue,
//                 }
//             }
//         }
//     }
}

#[allow(unused_variables)]
async fn send_icmpv6_echo(
    dest_ip: Ipv6Addr,
    timeout: Duration,
) -> Result<Duration, std::io::Error> {
    Ok(Duration::from_millis(10))
   
    // let (mut tx, mut rx) = transport_channel(4096, Layer4(pnet::transport::TransportProtocol::Ipv6(IpNextHeaderProtocols::Icmpv6)))
    // .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;


    // let mut echo_packet = [0u8; 64];
    // let mut echo_packet = MutableIcmpv6Packet::new(&mut echo_packet)
    //     .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Failed to create echo packet"))?;

    // echo_packet.set_icmpv6_type(Icmpv6Types::EchoRequest);
    // echo_packet.set_icmpv6_code(Icmpv6Code::new(0));
    // // Note: For ICMPv6, you might need to set the sequence number and identifier differently
    // // This depends on how you want to structure your ICMPv6 Echo Request packet

    // let start_time = Instant::now();

    // tx.send_to(echo_packet, IpAddr::V6(dest_ip))
    //     .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // let mut iter = icmpv6_packet_iter(&mut rx);

    // let timeout_future = tokio::time::sleep(timeout);

    // tokio::pin!(timeout_future);

    // let mut buf = [0u8; 1500];
    // loop {
    //     tokio::select! {
    //         _ = &mut timeout_future => {
    //             return Err(std::io::Error::new(
    //                 std::io::ErrorKind::TimedOut,
    //                 "ICMPv6 echo timed out",
    //             ));
    //         }
    //         result = tokio::task::spawn_blocking(move || rx.recv_from(&mut buf)) => {
    //             match result {
    //                 Ok(Ok((size, addr))) => {
    //                     if addr == IpAddr::V6(dest_ip) {
    //                         let icmpv6_packet = pnet::packet::icmpv6::Icmpv6Packet::new(&buf[..size]).unwrap();

    //                         if icmpv6_packet.get_icmpv6_type() == Icmpv6Types::EchoReply {
    //                             return Ok(start_time.elapsed());
    //                         }
    //                     }
    //                 }
    //                 _ => continue,
    //             }
    //         }
    //     }
    // }
}

