/**
 * CLI Entry Point
*/

use env_logger;
use clap::{Command, Arg, ArgAction};

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
                // run_scan(target).await?;
                println!("scanning target: {}", target)
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
