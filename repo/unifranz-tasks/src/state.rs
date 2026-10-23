use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Seen {
    pub ids: HashSet<String>,
}

impl Seen {
    pub fn load() -> Result<Self> {
        let mut p = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        p.push("unifranz-tasks");
        fs::create_dir_all(&p)?;
        p.push("seen.json");
        if !p.exists() {
            return Ok(Seen::default());
        }
        let s = fs::read_to_string(&p)?;
        Ok(serde_json::from_str(&s)?)
    }
    pub fn save(&self) -> Result<()> {
        let mut p = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        p.push("unifranz-tasks");
        fs::create_dir_all(&p)?;
        p.push("seen.json");
        let s = serde_json::to_string_pretty(self)?;
        fs::write(&p, s)?;
        Ok(())
    }
}
