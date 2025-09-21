use crate::util::expand_tilde;
use anyhow::Result;
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

const CONFIG_FILE: &str = "taso/config";

const D_DEFAULT_LIST: &str = "default";
const D_GLOBAL_STORE: &str = "~/.local/share/taso";
const D_DATA_FILE_NAME: &str = ".todo";
const D_TREE_LINE: u8 = 10;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    default_list: String,
    global_store: PathBuf,
    data_file_name: String,
    tree_line: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_list: D_DEFAULT_LIST.to_owned(),
            global_store: expand_tilde(PathBuf::from(D_GLOBAL_STORE)),
            data_file_name: D_DATA_FILE_NAME.to_owned(),
            tree_line: D_TREE_LINE,
        }
    }
}

impl Config {
    pub fn read() -> Result<Self> {
        let config_file = match env::var("XDG_CONFIG_HOME") {
            Ok(config_dir) => PathBuf::from(config_dir).join(CONFIG_FILE),
            Err(_) => match env::home_dir() {
                Some(home_dir) => home_dir.join(".config").join(CONFIG_FILE),
                None => {
                    warn!(target: "config::read", "fallback to default config");
                    return Ok(Self::default());
                }
            },
        };

        if !config_file.exists() {
            warn!(target: "config::read", "fallback to default config");
            return Ok(Self::default());
        }

        debug!(target: "config::read", "config file at: {config_file:?}");
        let content = fs::read_to_string(config_file)?;
        Ok(toml::from_str(&content).unwrap_or_else(|_| Self::default()))
    }

    pub fn default_list(&self) -> &str {
        &self.default_list
    }

    pub fn global_store(&self) -> &PathBuf {
        &self.global_store
    }

    pub fn data_file_name(&self) -> &str {
        &self.data_file_name
    }

    pub fn tree_line(&self) -> u8 {
        self.tree_line
    }
}
