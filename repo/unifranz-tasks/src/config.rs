use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub ics: Ics,
    pub refresh: Option<Refresh>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ics {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Refresh {
    pub interval_minutes: Option<u64>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let mut p = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        p.push("unifranz-tasks");
        p.push("config.toml");
        let s = fs::read_to_string(&p)?;
        let c: Config = toml::from_str(&s)?;
        Ok(c)
    }
}
