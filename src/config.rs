use std::{env, time::Duration};

use crate::next_index;

const ROUTER_IP: &str = "192.168.178.1";
/// High availability targets of completely different providers - google, cloudflare etc
const TARGETS: [&str; 10] = [
    "1.1.1.1",
    "149.112.112.112",
    "8.8.4.4",
    "94.140.14.14",
    "1.0.0.1",
    "208.67.220.220",
    "8.8.8.8",
    "94.140.15.15",
    "9.9.9.9",
    "208.67.222.222",
];

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
        let router_ip = env::var("ROUTER_IP").unwrap_or_else(|_| ROUTER_IP.to_string());
        let log_dir_path =
            env::var("LOG_DIR_PATH").unwrap_or_else(|_| "./shamash-logs/".to_string());
        let secondary_internal_target = env::var("SECONDARY_INTERNAL_TARGET").ok();
        Self {
            router_ip,
            secondary_internal_target,
            targets: TARGETS.iter().map(|s| s.to_string()).collect(),
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
