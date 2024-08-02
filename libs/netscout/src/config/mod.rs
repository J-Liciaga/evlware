mod settings;

use std::str::FromStr;
use clap::ArgMatches;
use config::ConfigError;
use crate::models::noise::NoiseLevel;

pub use settings::Settings;

pub fn load(cli_matches: &ArgMatches) -> Result<Settings, ConfigError> {
    let mut settings = Settings::new()?;
    
    // Override settings with CLI arguments //
    if let Some(noise) = cli_matches.get_one::<String>("noise") {
        settings.noise_level = NoiseLevel::from_str(noise)
            .map_err(|e| ConfigError::Message(e))?;
    }
    
    Ok(settings)
}
