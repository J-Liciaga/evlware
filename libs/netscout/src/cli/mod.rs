mod analyze;
mod enumerate;
mod report;
mod scan;
mod vulnerability;
use crate::models::common::NoiseLevel;
use crate::config::Settings;

use clap::{
    Command,
    Arg,
    ArgMatches,
};

pub fn build_cli() -> Command {
    Command::new("NETSCOUT")
        .version("0.2.0")
        .author("EVLWARE")
        .about("Network Penetration Testing Tool")
        .arg(
            Arg::new("noise")
                .short('n')
                .long("noise")
                .value_name("NOISE")
                .value_parser(["stealthy", "moderate", "aggresive"])
                .default_value("moderate")
                .global(true)
                .help("Sets the noise level of operations.")
        )
        .subcommand(analyze::command())
        .subcommand(enumerate::command())
        .subcommand(report::command())
        .subcommand(scan::command())
        .subcommand(vulnerability::command())
}

pub fn parse_noise_level(
    matches: &clap::ArgMatches,
) -> NoiseLevel {
    match matches.get_one::<String>("noise").map(|s| s.as_str()) {
        Some("stealthy")=> NoiseLevel::Stealthy,
        Some("moderate") => NoiseLevel::Moderate,
        Some("aggressive") => NoiseLevel::Aggressive,
        _ => NoiseLevel::Moderate,
    }
}

pub async fn execute(
    matches: &ArgMatches,
    config: &Settings,
) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("analyze", sub_matches)) => analyze::execute(sub_matches, config).await,
        Some(("enum", sub_matches)) => enumerate::execute(sub_matches, config).await,
        Some(("report", sub_matches)) => report::execute(sub_matches, config).await,
        Some(("scan", sub_matches)) => scan::execute(sub_matches, config).await,
        Some(("vuln", sub_matches)) => vulnerability::execute(sub_matches, config).await,
        _ => {
            println!("Please specify a valid subcommand. Use --help for more information.");
            Ok(())
        }
    }
}
