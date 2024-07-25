/**
 * CLI Entry Point
*/

use env_logger;
use clap::{Command, Arg, ArgAction};

use netscout;
use netscout::models::results::ScanResults;
use netscout::core::scanners::EVLScanner;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let matches = Command::new("NetScout")
        .version("0.1.0")
        .author("EVLWARE")
        .about("Network penetration testing tool")
        .subcommand(
            Command::new("scan")
                .about("Run a full scan")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("URL")
                        .help("Sets the target URL to scan")
                        .required(true)
                        .action(ArgAction::Set)
                )
        )
        .subcommand(
            Command::new("enumerate")
                .about("Run enumeration only")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("URL")
                        .help("Sets the target URL to enumerate")
                        .required(true)
                        .action(ArgAction::Set)
                )
        )
        .subcommand(
            Command::new("vulnerabilities")
                .about("Run vulnerability scan only")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("URL")
                        .help("Sets the target URL to scan for vulnerabilities")
                        .required(true)
                        .action(ArgAction::Set)
                )
        )
        .get_matches();

        match matches.subcommand() {
            Some(("scan", scan_matches)) => {
                let target = scan_matches.get_one::<String>("target").unwrap();
                let start_port: u16 = 1;
                let end_port: u16 = 1024;

                println!("Scanning target: {}, From port: {} to {}", target, start_port, end_port);

                let scan_results: ScanResults = netscout::scan_ports(target, start_port, end_port);

                if scan_results.open_ports.is_empty() {
                    println!("No open ports found");
                } else {
                    println!("Open ports: {:?}", scan_results.open_ports);
                    println!("Detected services: {:?}", scan_results.detected_services);

                    if scan_results.vulnerabilities.is_empty() {
                        println!("No vulnerabilities detected");
                    } else {
                        println!("Detected vulnerabilities: {:?}", scan_results.vulnerabilities);
                    }
                }

                let scanner = EVLScanner::new(target);
                let direct_scan_results = scanner.scan();
                
                println!("Direct scan results: {:?}", direct_scan_results);
            },
            Some(("enumerate", enum_matches)) => {
                let target = enum_matches.get_one::<String>("target").unwrap();
                // enumeration::enumerate(target).await?;\
                println!("enumerating target: {}", target)
            },
            Some(("vulnerabilities", vuln_matches)) => {
                let target = vuln_matches.get_one::<String>("target").unwrap();
                // vulnerability_scanner::scan(target).await?;
                println!("scanning for vulnerabilities on target: {}", target)
            },
            _ => println!("Please specify a valid subcommand. Use --help for more information."),
        }

        Ok(())
}
