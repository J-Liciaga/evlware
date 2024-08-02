use serde::Deserialize;
use config::{
    ConfigError, 
    Config, 
    Environment, 
    File, 
    Value
};
use std::env;
use std::str::FromStr;
use crate::models::noise::NoiseLevel;

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub debug: bool,
    #[serde(default)]
    pub noise_level: NoiseLevel,
    #[serde(default)]
    pub threads: usize,
    #[serde(default)]
    pub timeout: u64,
    #[serde(default)]
    pub user_agent: String,
    #[serde(default)]
    pub network: NetworkSettings,
    #[serde(default)]
    pub web: WebSettings,
    #[serde(default)]
    pub enumeration: EnumerationSettings,
    #[serde(default)]
    pub vulnerability: VulnerabilitySettings,
}

#[derive(Debug, Deserialize, Default)]
pub struct NetworkSettings {
    #[serde(default)]
    pub port_scan_range: String,
    #[serde(default)]
    pub os_detection: bool,
}

#[derive(Debug, Deserialize, Default)]
pub struct WebSettings {
    #[serde(default)]
    pub max_depth: usize,
    #[serde(default)]
    pub respect_robots_txt: bool,
}

#[derive(Debug, Deserialize, Default)]
pub struct EnumerationSettings {
    #[serde(default)]
    pub subdomain_wordlist: String,
    #[serde(default)]
    pub directory_wordlist: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct VulnerabilitySettings {
    #[serde(default)]
    pub sql_injection: bool,
    #[serde(default)]
    pub xss: bool,
    #[serde(default)]
    pub csrf: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            debug: false,
            noise_level: NoiseLevel::Moderate,
            threads: 4,
            timeout: 30,
            user_agent: "EVLWARE-NETSCOUT/1.0".to_string(),
            network: NetworkSettings::default(),
            web: WebSettings::default(),
            enumeration: EnumerationSettings::default(),
            vulnerability: VulnerabilitySettings::default(),
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("APP"))
            .set_default("debug", false)?
            .set_default("noise_level", Value::new(None, "moderate"))?
            .set_default("threads", 4)?
            .set_default("timeout", 30)?
            .set_default("user_agent", "EVLWARE-Core/1.0")?
            .build()?;

        // Extract noise_level before deserialization
        let noise_level = config.get_string("noise_level")
            .map(|s| NoiseLevel::from_str(&s).unwrap_or(NoiseLevel::Moderate))
            .unwrap_or(NoiseLevel::Moderate);

        let mut settings: Settings = config.try_deserialize()?;

        // Set the extracted noise_level
        settings.noise_level = noise_level;

        Ok(settings)
    }
}
