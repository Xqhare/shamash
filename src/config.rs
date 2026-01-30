use std::{env, path::PathBuf, time::Duration};

use mawu::read::json;

use crate::{next_index, utils::generate_and_write_config_file};

pub const CONFIG_FILE_PATH: &str = "./config.json";

pub struct Config {
    pub router_ip: String,
    pub secondary_internal_target: Option<String>,
    pub targets: Vec<String>,
    index: usize,
    pub interval_normal: Duration,
    pub interval_recovery: Duration,
    pub log_dir_path: String,
}

impl Config {
    pub fn new() -> Self {
        let config_file_path = PathBuf::from(CONFIG_FILE_PATH);
        if !config_file_path.exists() {
            generate_and_write_config_file();
        }
        let config_file = json(config_file_path).expect("Failed to read config file");
        let config = config_file
            .to_object()
            .expect("Failed to parse config file");
        let router_ip = env::var("ROUTER_IP").unwrap_or_else(|_| {
            config
                .get("router_ip")
                .expect("No Key: ROUTER_IP in config")
                .to_string()
        });
        let log_dir_path = env::var("LOG_DIR_PATH").unwrap_or_else(|_| {
            config
                .get("log_dir_path")
                .expect("No Key: LOG_DIR_PATH in config")
                .to_string()
        });
        let targets = config
            .get("targets")
            .expect("No Key: targets in config")
            .to_array();
        let secondary_internal_target: Option<String> = {
            if let Ok(s) = env::var("SECONDARY_INTERNAL_TARGET") {
                Some(s)
            } else {
                config
                    .get("secondary_internal_target")
                    .map(mawu::mawu_value::MawuValue::to_string)
            }
        };
        Self {
            router_ip,
            secondary_internal_target,
            targets: targets.iter().map(mawu::mawu_value::MawuValue::to_string).collect(),
            index: 0,
            interval_normal: Duration::from_secs(1),
            interval_recovery: Duration::from_millis(333),
            log_dir_path,
        }
    }

    pub fn current_target(&self) -> String {
        self.targets[self.index].clone()
    }

    pub fn next_target(&self) -> String {
        self.targets[next_index(self.index, self.targets.len())].clone()
    }

    pub fn iter_targets(&mut self) {
        self.index = next_index(self.index, self.targets.len());
    }
}
