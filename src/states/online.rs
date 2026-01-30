use std::{thread, time::Duration};

use horae::Utc;

use crate::{config::Config, log::Logger, utils::is_answering_ping};

use super::{diagnosing::write_diagnosing_file, ConnectionState};

/// Checks if the target is online
///
/// Returns `Some(ConnectionState::Diagnosing)` if the target is not online
/// Returns `None` otherwise
pub fn online(config: &Config, logger: &mut Logger) -> Option<ConnectionState> {
    if is_answering_ping(
        &config.current_target(),
        config.interval_normal,
        logger,
        &ConnectionState::Online,
    ) {
        online_sleep(config.interval_normal)
    } else {
        // The next IP is guaranteed to be a different provider than the current
        if is_answering_ping(
            &config.next_target(),
            config.interval_normal,
            logger,
            &ConnectionState::Online,
        ) {
            online_sleep(config.interval_normal)
        } else {
            Some(move_to_diagnosing(config, logger))
        }
    }
}

fn move_to_diagnosing(config: &Config, logger: &mut Logger) -> ConnectionState {
    let now = Utc::now();
    logger.reset();

    logger.add_large_separator();
    logger.add_log_line(format!("Start of log: {now}"));
    logger.add_large_separator();
    logger.add_log_line(format!(
        "🟡 Target '{}' and secondary target '{}' failed to answer",
        &config.current_target(),
        &config.next_target()
    ));
    logger.add_large_separator();

    write_diagnosing_file(&logger.log_dir_path);
    ConnectionState::Diagnosing
}

fn online_sleep(dur: Duration) -> Option<ConnectionState> {
    thread::sleep(dur);
    None
}
