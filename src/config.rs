use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub log_level: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self { log_level: None }
    }
}

impl AppConfig {
    pub fn load_from(path: Option<&Path>) -> Result<Self> {
        if let Some(p) = path {
            if p.exists() {
                let bytes = fs::read(p)?;
                let cfg: AppConfig = serde_json::from_slice(&bytes)?;
                return Ok(cfg);
            }
        }
        if let Ok(env_path) = std::env::var("MONKEY_TYPER_CONFIG") {
            let p = Path::new(&env_path);
            if p.exists() {
                let bytes = fs::read(p)?;
                let cfg: AppConfig = serde_json::from_slice(&bytes)?;
                return Ok(cfg);
            }
        }
        Ok(Self::default())
    }
}