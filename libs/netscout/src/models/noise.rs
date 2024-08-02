use std::str::FromStr;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, PartialEq)]
pub enum NoiseLevel {
    Stealthy,
    Moderate,
    Aggressive,
}

impl Default for NoiseLevel {
    fn default() -> Self {
        NoiseLevel::Moderate
    }
}

impl FromStr for NoiseLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "stealthy" => Ok(NoiseLevel::Stealthy),
            "moderate" => Ok(NoiseLevel::Moderate),
            "aggressive" => Ok(NoiseLevel::Aggressive),
            _ => Err(format!("Invalid noise level: {}", s)),
        }
    }
}

impl<'de> Deserialize<'de> for NoiseLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NoiseLevel::from_str(&s).map_err(serde::de::Error::custom)
    }
}
