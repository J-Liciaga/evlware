use clap::{
    Command,
    Arg,
    ArgAction,
    ArgMatches,
};
use std::time::Duration;
use crate::models::results::ScanResults;
use crate::core::enumerate::evl_enum::{EvlEnumerationConfig, scan};

pub fn command() -> Command {
    Command::new("scan")
        .about("")
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
                .help("Sets how loud your enumeration scan will be: quiet, default, noisy, loud")
        )
}

pub async fn execute(
    matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let target = matches.get_one::<String>{"target"}.unwrap();

    let default_mode = String::from("default");
    let mode = matches.get_one::<String>("mode").unwrap_or(&default_mode);

    println!(
        "Enumerating target: {}, Mode: {}",
        target,
        mode,
    );

    let config = EvlEnumerationConfig {};

    let scan_results = scan(&config).await?;

    print_scan_results(&scan_results);

    Ok(())
}

#[use_unused_variables]
fn print_scan_results(
    scan_results: &ScanResults,
) {
    Ok(())
}
