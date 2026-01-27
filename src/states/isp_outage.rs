use std::thread;

use horae::Utc;

use crate::{config::Config, log::Logger, utils::is_answering_ping};

use super::ConnectionState;

pub fn isp_outage(config: &Config, logger: &mut Logger) -> Option<ConnectionState> {
    if is_answering_ping(
        &config.current_target(),
        config.interval_recovery,
        logger,
        ConnectionState::IspOutage,
    ) {
        let now = Utc::now();
        logger.add_log_line(format!(
            "🟢 Connection established with target '{}' at {}",
            &config.current_target(),
            now
        ));
        if is_answering_ping(
            &config.next_target(),
            config.interval_recovery,
            logger,
            ConnectionState::IspOutage,
        ) {
            let now = Utc::now();
            logger.end_log(format!(
                "🟢 Connection established with second target '{}' at {}",
                &config.next_target(),
                now
            ));
            let _ = std::fs::remove_file(logger.log_dir_path.clone() + "/isp_outage_ongoing");
            Some(ConnectionState::Online)
        } else {
            logger.add_log_line(format!(
                "🟡 Connection not established with second target '{}' at {}",
                &config.next_target(),
                now
            ));
            logger.add_log_line("🔴 Continuing ISP outage".to_string());
            logger.add_small_separator();
            // One target reports up, the other down - Should be unreachable, but I
            // guess we'll just run the loop again.
            None
        }
    } else {
        thread::sleep(config.interval_recovery);
        None
    }
}
