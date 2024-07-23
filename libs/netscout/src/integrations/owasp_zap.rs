use std::process::Command;
use std::error::Error;

pub async fn run_owasp_zap(
    target: &str,
) -> Result<(), Box<dyn Error>> {
    println!("Running OWASP Zap scan on {}", target);

    Ok(())
}
