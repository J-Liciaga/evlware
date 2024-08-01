use clap::{
    Command,
    Arg,
    ArgAction,
    ArgMatches,
};

pub fn command() -> Command {
    Command::new("report")
        .arg(
            Arg::new("generate")
                .short('g')
                .long("generate")
                .value_name("GENERATE")
                .required(true)
                .action(ArgAction::Set)
                .help("Generate, list, and view reports")
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .required(true)
                .action(ArgAction::Set)
                .help("Report file format")
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .required(true)
                .action(ArgAction::Set)
                .help("Report name")
        )
}

pub async fn execute(
    matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let generate = matches.get_one::<String>("generate").unwrap();

    println!(
        "Creating Report for target: {}", 
        generate, 
    );

    Ok(())
}
