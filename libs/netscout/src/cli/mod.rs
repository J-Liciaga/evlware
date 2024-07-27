mod scan;
mod enumerate;
mod vulnerabilities;

use clap::{Command, ArgMatches};

pub fn build_cli() -> Command {
    Command::new("EVLWARE - NetScout")
        .version("0.1.0")
        .author("EVLWARE")
        .about("Network Penetration Testing Tool")
        .subcommand(scan::command())
        .subcommand(enumerate::command())
        .subcommand(vulnerabilities::command())
}

pub async fn execute(
    matches: &ArgMatches
) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("scan", sub_matches)) => scan::execute(sub_matches).await,
        Some(("enum", sub_matches)) => enumerate::execute(sub_matches).await,
        Some(("vuln", sub_matches)) => vulnerabilities::execute(sub_matches).await,
        _ => {
            println!("Please specify a valid subcommand. Use --help for more information.");
            Ok(())
        } 
    }
}
