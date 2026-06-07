use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct SuiteConfig {
    #[serde(default = "default_control_port")]
    pub control_port: u16,
    #[serde(default = "default_control_address")]
    pub control_address: String,
}

fn default_control_port() -> u16 {
    13340
}

fn default_control_address() -> String {
    "127.0.0.1".to_string()
}

impl Default for SuiteConfig {
    fn default() -> Self {
        SuiteConfig {
            control_port: 13340,
            control_address: "127.0.0.1".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    suite: Option<SuiteConfig>,
}

/// Load the `[suite]` section from `~/.config/jdw.toml`.
pub fn load() -> SuiteConfig {
    let config_path = config_path();
    let content = match std::fs::read_to_string(&config_path) {
        Ok(c) => c,
        Err(_) => return SuiteConfig::default(),
    };
    match toml::from_str::<ConfigFile>(&content) {
        Ok(cfg) => cfg.suite.unwrap_or_default(),
        Err(_) => SuiteConfig::default(),
    }
}

fn config_path() -> PathBuf {
    if let Ok(path) = std::env::var("JDW_CONFIG") {
        return PathBuf::from(path);
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(format!("{}/.config/jdw.toml", home))
}
