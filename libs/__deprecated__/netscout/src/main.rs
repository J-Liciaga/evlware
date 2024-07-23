/**
 * CLI Entry Point
*/

use clap::Parser;
use netscout::NetScout;
use netscout::models::{ScanConfig, ScanType};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    target: String,

    #[arg(short, long, default_type = "quick" )]
    scan_type: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    let scan_type = match cli.scan_type.as_str() {
        "quick" => ScanType::Quick,
        "full" => ScanType::Full,
        _ => ScanType::Custom,
    };

    let config = ScanConfig {
        target: cli.target,
        scan_type,
    };

    let netscout = NetScout::new().await?;
    let result = netscout.run_scan(config).await?;

    println!("{:#?}", result);

    Ok(())
}
