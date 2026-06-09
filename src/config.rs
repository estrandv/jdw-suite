use serde::Deserialize;
use std::path::PathBuf;

pub const DEFAULT_SAMPLE_PACK_DIR: &str = "/usr/local/share/jdw/sample_packs";
pub const DEFAULT_NRT_OUTPUT_DIR: &str = "./output";

#[derive(Debug, Clone, Deserialize)]
pub struct SuiteConfig {
    #[serde(default = "default_control_port")]
    pub control_port: u16,
    #[serde(default = "default_control_address")]
    pub control_address: String,
    #[serde(default = "default_nrt_listener_port_base")]
    pub nrt_listener_port_base: u16,
    #[serde(default = "default_sequencer_in_port")]
    pub sequencer_in_port: u16,
}

fn default_control_port() -> u16 {
    13340
}

fn default_control_address() -> String {
    "127.0.0.1".to_string()
}

fn default_nrt_listener_port_base() -> u16 {
    13456
}

fn default_sequencer_in_port() -> u16 {
    14441
}

impl Default for SuiteConfig {
    fn default() -> Self {
        SuiteConfig {
            control_port: 13340,
            control_address: "127.0.0.1".to_string(),
            nrt_listener_port_base: 13456,
            sequencer_in_port: 14441,
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
