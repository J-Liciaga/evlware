use crate::config::Settings;
use crate::core::scan::{EvlScannerConfig, scan};
use crate::models::target::Target;
use crate::models::results::ScanResults;
use clap::{
    Command,
    Arg,
    ArgAction,
    ArgMatches,
};
use std::time::Duration;

pub fn command() -> Command {
    Command::new("scan")
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
            Arg::new("noise")
                .short('n')
                .long("noise")
                .value_name("NOISE")
                .required(false)
                .action(ArgAction::Set)
                .help("Sets your scan mode: stealthy, moderate, aggressive")
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
    matches: &ArgMatches,
    config: &Settings,
) -> Result<(), Box<dyn std::error::Error>> {
    let target = matches.get_one::<String>("target").unwrap();
    let target = Target::new(&target)?;
    let start_port: u16 = *matches.get_one::<u16>("start-port").unwrap();
    let end_port: u16 = *matches.get_one::<u16>("end-port").unwrap();

    let config = EvlScannerConfig {
        target: target.clone(),
        noise: config.noise_level.clone(),
        port_range: (start_port, end_port),
        port_timeout: Duration::from_secs(1),
        web_timeout: Duration::from_secs(10),
        max_concurrent_port_scans: 100,
        max_concurrent_web_scans: 10,
        firewall_detection: true,
        firewall_timeout: Duration::from_secs(10),
    };

    println!(
        "Running a Full Scan on target: {:?}", 
        target, 
    );

    let scan_results = scan(&config).await?;
    
    print_scan_results(&scan_results);

    Ok(())
}

fn print_scan_results(scan_results: &ScanResults) {
    if let Some(firewall_profile) = &scan_results.firewall_profile {
        println!("Firewall Detected: {}", firewall_profile);
    } else {
        println!("No firewall detected");
    }

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
