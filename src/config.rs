use std::{env, time::Duration};

const ROUTER_IP: &str = "192.168.178.1";
const TARGETS: [&str; 10] = [
    "1.1.1.1",
    "1.0.0.1",
    "8.8.4.4",
    "8.8.8.8",
    "9.9.9.9",
    "94.140.14.14",
    "94.140.15.15",
    "149.112.112.112",
    "208.67.222.222",
    "208.67.220.220",
];

pub struct Config {
    pub router_ip: String,
    pub targets: Vec<String>,
    pub interval_normal: Duration,
    pub interval_recovery: Duration,
    pub log_dir_path: String,
}

impl Config {
    pub fn new() -> Self {
        let router_ip = env::var("ROUTER_IP").unwrap_or_else(|_| ROUTER_IP.to_string());
        let log_dir_path =
            env::var("LOG_DIR_PATH").unwrap_or_else(|_| "./shamash-logs/".to_string());
        Self {
            router_ip,
            targets: TARGETS.iter().map(|s| s.to_string()).collect(),
            interval_normal: Duration::from_secs(1),
            interval_recovery: Duration::from_millis(333),
            log_dir_path,
        }
    }
}
