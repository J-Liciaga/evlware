use crate::config::Settings;
use clap::{
    Command,
    Arg,
    ArgAction,
    ArgMatches,
};

pub fn command() -> Command {
    Command::new("enum")
        .arg(
            Arg::new("target")
                .short('t')
                .long("target")
                .value_name("URL")
                .required(true)
                .action(ArgAction::Set)
                .help("Sets the target URL or IP address to scan")
        )
}

pub async fn execute(
    matches: &ArgMatches,
    config: &Settings,
) -> Result<(), Box<dyn std::error::Error>> {
    let target = matches.get_one::<String>("target").unwrap();

    println!(
        "Creating Report for target: {}, with Settings: {:?}", 
        target, 
        config,
    );

    Ok(())
}
