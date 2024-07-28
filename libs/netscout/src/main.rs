// CLI Entry Point

use env_logger;
use netscout::cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let cli = cli::build_cli();
    let matches = cli.get_matches();

    cli::execute(&matches).await
}
