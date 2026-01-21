use std::{env, time::Duration};

const ROUTER_IP: &str = "192.168.178.1";

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
        let log_dir_path = env::var("LOG_DIR_PATH").unwrap_or_else(|_| "./shamash-logs/".to_string());
        Self {
            router_ip,
            targets: vec!["1.1.1.1".to_string(), "8.8.8.8".to_string(), "9.9.9.9".to_string()],
            interval_normal: Duration::from_secs(1),
            interval_recovery: Duration::from_millis(333),
            log_dir_path,
        }
    }
}
