use std::{
    process::{Command, Stdio},
    time::Duration,
};

use mawu::{mawu_value::MawuValue, write_pretty};

use crate::{config::CONFIG_FILE_PATH, log::Logger, states::ConnectionState};

/// Ping a host
///
/// # Arguments
///
/// * `addr` - The address to ping
/// * `timeout_duration` - The duration to wait for a response
/// * `logger` - The logger instance to use
/// * `state` - The current state of the connection
///
/// # Returns
///
/// `true` if the ping was successful, `false` otherwise
pub fn is_answering_ping(
    addr: &str,
    timeout_duration: Duration,
    logger: &mut Logger,
    state: ConnectionState,
) -> bool {
    let status = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg("-W")
        .arg(timeout_duration.as_secs_f32().to_string())
        .arg(addr)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match status {
        Ok(status) => if status.success() {
            if state != ConnectionState::Online {
                logger.add_log_line(format!("🟢 Target '{addr}' is answering"));
            }
            true
        } else {
            logger.add_log_line(format!("🔴 Target '{addr}' is not answering"));
            false
        },
        Err(e) => {
            logger.add_log_line(format!("{e}"));
            false
        }
    }
}

/// High availability targets of completely different providers - google, cloudflare, quad9,
/// adguard, opendns, level3, verisign, comododns, dnswatch, controld
/// "scrambled" in a way that no two subsequent targets are by the same provider
const TARGETS: &[&str] = &[
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
    "4.2.2.1",
    "64.6.64.6",
    "4.2.2.2",
    "64.6.65.6",
    "8.26.56.26",
    "76.76.10.0",
    "84.200.69.80",
    "8.20.247.20",
    "84.200.70.40",
    "76.76.2.0",
];
const LOG_DIR_PATH: &str = "./shamash-logs";

// My personal routers - acting as placeholders for the config
const ROUTER_IP: &str = "192.168.178.1";
const SECONDARY_INTERNAL_TARGET: &str = "192.168.178.2";

pub fn generate_and_write_config_file() {
    let mut file = MawuValue::new_object();

    file.object_insert("router_ip", ROUTER_IP);
    file.object_insert("secondary_internal_target", SECONDARY_INTERNAL_TARGET);
    let mut targets = MawuValue::new_array();
    for (i, target) in TARGETS.iter().enumerate() {
        targets.array_insert(i, MawuValue::from(*target));
    }
    file.object_insert("targets", targets);
    file.object_insert("log_dir_path", LOG_DIR_PATH);

    write_pretty(CONFIG_FILE_PATH, &file, 4).expect("Failed to write config file");
}
