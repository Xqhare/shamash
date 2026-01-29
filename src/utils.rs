use std::{
    process::{Command, Stdio},
    time::Duration,
};

use crate::{log::Logger, states::ConnectionState};

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
        Ok(status) => match status.success() {
            true => {
                if state != ConnectionState::Online {
                    logger.add_log_line(format!("🟢 Target '{}' is answering", addr));
                }
                true
            }
            false => {
                logger.add_log_line(format!("🔴 Target '{}' is not answering", addr));
                false
            }
        },
        Err(e) => {
            logger.add_log_line(format!("{}", e));
            false
        }
    }
}
