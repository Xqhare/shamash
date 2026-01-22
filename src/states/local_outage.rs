use std::thread;

use horae::Utc;

use crate::{config::Config, log::{EventType, Logger}, utils::is_answering_ping};

use super::ConnectionState;


pub fn local_outage(config: &Config, logger: &mut Logger) -> Option<ConnectionState> {
    if is_answering_ping(&config.router_ip, config.interval_recovery, logger) {
        let now = Utc::now();
        logger.add_log_line(format!("🟢 Connection with Router established at {}", now));
        logger.add_log_line(format!(
            "🟢 Local Outage end declared - checking outside connection"
        ));
        logger.add_small_separator();
        if is_answering_ping(&config.current_target(), config.interval_recovery, logger) {
            let now = Utc::now();
            logger
                .add_log_line(format!("🟢 Outside test connection successful with target '{}' at {}", &config.current_target(), now));
            logger.add_large_separator();
            logger.end_log(format!("🟢 Local Outage end at {}", now));
            logger.add_large_separator();
            Some(ConnectionState::Online)
        } else {
            logger.add_log_line(format!(
                "🟡 Outside test connection unsuccessful with target '{}' at {}",
                &config.current_target(),
                now
            ));
            logger.add_log_line(format!(
                "Retrying in {} seconds",
                config.interval_recovery.as_secs()
            ));
            logger.add_small_separator();
            thread::sleep(config.interval_recovery);
            if is_answering_ping(&config.current_target(), config.interval_recovery, logger) {
                let now = Utc::now();
                logger
                    .end_log(format!("🟢 Outside test connection successful with target '{}' at {}",
                        &config.current_target(),
                        now
                    ));
                Some(ConnectionState::Online)
            } else {
                let now = Utc::now();
                logger.add_log_line(format!(
                    "🔴 Outside test connection unsuccessful at {}",
                    now
                ));
                logger.add_log_line(format!(
                    "🔴 Declaring ISP outage at {}, continuing the outage",
                    now
                ));
                logger.add_large_separator();
                logger.event_type = EventType::IspOutage;
                Some(ConnectionState::IspOutage)
            }
        }
    } else {
        thread::sleep(config.interval_recovery);
        None
    }
}
