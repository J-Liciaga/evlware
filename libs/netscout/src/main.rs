use env_logger;
use netscout::cli;
use netscout::config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let ascii_art = r#"
     _   _ _____ ___________   ____ _   _   _ _____ 
    | \ | | ____|_   _/ ___| / ___/ _ \| | | |_   _|
    |  \| |  _|   | | \___ \| |  | | | | | | | | |  
    | |\  | |___  | |  ___) | |__| |_| | |_| | | |  
    |_| \_|_____| |_| |____/ \____\___/ \___/  |_|   
    "#;

    println!("{}", ascii_art);

    let cli = cli::build_cli();
    let matches = cli.get_matches();

    let config = config::load(&matches)?;
    
    cli::execute(&matches, &config).await
}
