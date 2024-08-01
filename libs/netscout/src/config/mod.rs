mod settings;

use std::str::FromStr;
use clap::ArgMatches;
use crate::models::common::NoiseLevel;

pub use settings::Settings;

pub fn load(cli_matches: &ArgMatches) -> Result<Settings, ::config::ConfigError> {
    let mut settings = Settings::new()?;
    
    // Override settings with CLI arguments //
    if let Some(noise) = cli_matches.get_one::<String>("noise") {
        settings.noise_level = NoiseLevel::from_str(noise)
            .map_err(|e| ::config::ConfigError::Message(e))?;
    }
    
    Ok(settings)
}
