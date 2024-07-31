use clap::{Command, Arg, ArgAction, ArgMatches};
use std::time::Duration;
use crate::models::results::ScanResults;
use crate::core::scanners::evl_scan::{EVLScannerConfig, scan};

pub fn command() -> Command {
    Command::new("scan")
        .about("Runs a full scan on the target.")
        .arg(
            Arg::new("target")
                .short('t')
                .long("target")
                .value_name("URL")
                .required(true)
                .action(ArgAction::Set)
                .help("Sets the target URL or IP address to scan")
        )
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .value_name("MODE")
                .required(false)
                .action(ArgAction::Set)
                .help("Sets your scan mode: quiet, default, noisy, loud")
        )
        .arg(
            Arg::new("start-port")
                .short('s')
                .long("start-port")
                .value_name("START_PORT")
                .value_parser(clap::value_parser!(u16).range(1..))
                .default_value("1")
                .required(false)
                .action(ArgAction::Set)
                .help("Defines starting port number")
        )
        .arg(
            Arg::new("end-port")
                .short('e')
                .long("end-port")
                .value_name("END_PORT")
                .value_parser(clap::value_parser!(u16).range(1..))
                .default_value("1024")
                .required(false)
                .action(ArgAction::Set)
                .help("Defines ending port number")
        )
}

pub async fn execute(
    matches: &ArgMatches
) -> Result<(), Box<dyn std::error::Error>> {
    let target = matches.get_one::<String>("target").unwrap();
    let start_port: u16 = *matches.get_one::<u16>("start-port").unwrap();
    let end_port: u16 = *matches.get_one::<u16>("end-port").unwrap();

    let default_mode = String::from("default");
    let mode = matches.get_one::<String>("mode").unwrap_or(&default_mode);  

    println!(
        "Scanning target: {}, From port: {} to {}, Mode: {}", 
        target, 
        start_port, 
        end_port,
        mode,
    );

    let config = EVLScannerConfig {
        target: target.to_string(),
        port_range: (start_port, end_port),
        port_timeout: Duration::from_secs(1),
        web_timeout: Duration::from_secs(10),
        max_concurrent_port_scans: 100,
        max_concurrent_web_scans: 10,
        mode: mode.to_string(),
        firewall_detection: true,
        firewall_timeout: Duration::from_secs(10),
    };

    let scan_results = scan(&config).await?;

    print_scan_results(&scan_results);

    Ok(()) 
}

fn print_scan_results(scan_results: &ScanResults) {
    if scan_results.open_ports.is_empty() {
        println!("No open ports found");
    } else {
        println!("Open ports: {:?}", scan_results.open_ports);
    }
    
    if scan_results.detected_services.is_empty() {
        println!("No services detected");
    } else {
        println!("Detected services: {:?}", scan_results.detected_services);
    }
    
    if scan_results.vulnerabilities.is_empty() {
        println!("No vulnerabilities detected");
    } else {
        println!("Detected vulnerabilities:");
        for vuln in &scan_results.vulnerabilities {
            println!("- {} (Severity: {:?})", vuln.name, vuln.severity);
            println!("  Description: {}", vuln.description);
        }
    }
}
