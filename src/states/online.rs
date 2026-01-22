use std::{thread, time::Instant};

use horae::Utc;

use crate::{config::Config, log::Logger, utils::is_answering_ping};

use super::ConnectionState;


/// Checks if the target is online
///
/// Returns `Some(ConnectionState::Diagnosing)` if the target is not online
/// Returns `None` otherwise
pub fn online(config: &Config, logger: &mut Logger) -> Option<ConnectionState> {
    if is_answering_ping(&config.current_target(), config.interval_normal, logger) {
        thread::sleep(config.interval_normal);
        None
    } else {
        if is_answering_ping(&config.next_target(), config.interval_normal, logger) {
            thread::sleep(config.interval_normal);
            None
        } else {
            logger.reset();
            logger.add_large_separator();
            logger.add_log_line(format!("Start of log"));
            let now = Utc::now();
            logger.add_log_line(format!("{}", now));
            logger.add_large_separator();
            logger.add_log_line(format!("🟡 Target '{}' and secondary target '{}' failed to answer", &config.current_target(), &config.next_target()));
            logger.add_large_separator();
            Some(ConnectionState::Diagnosing)
        }
    }
}
