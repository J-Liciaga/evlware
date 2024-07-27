use clap::{Command, Arg, ArgAction, ArgMatches};
use netscout::models::results::ScanResults;
use netscout::core::scanners::EVLScanner;

pub fn command() -> Command {
    Command::new("enum")
        .about("Focuses on enumeration of the target.")
        .arg(
            Arg::new("target")
                .short('t')
                .long("target")
                .value_name("URL")
                .help("Sets the target URL or IP address to scan")
                .required(true)
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("start-port")
                .short('s')
                .long("start-port")
                .value_name("START_PORT")
                .value_parser(clap::value_parser!(u16).range(1..))
                .help("Defines starting port number")
                .default_value("1")
                .required(true)
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("end-port")
                .short('e')
                .long("end-port")
                .value_name("END_PORT")
                .value_parser(clap::value_parser!(u16).range(1..))
                .help("Defines ending port number")
                .default_value("8000")
                .required(true)
                .action(ArgAction::Set)
        )
}

pub async fn execute(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let target = matches.get_one::<String>("target").unwrap();
    let start_port: u16 = *matches.get_one::<u16>("start-port").unwrap();
    let end_port: u16 = *matches.get_one::<u16>("end-port").unwrap();

    println!("Enumerating target: {}, From port: {} to {}", target, start_port, end_port);

    let mut scanner = EVLScanner::new(target).await;
    
    scanner.set_port_range(start_port, end_port);

    let scan_results = scanner.scan().await?;

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