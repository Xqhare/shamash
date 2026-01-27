use std::thread;

use horae::Utc;

use crate::{config::Config, log::{EventType, Logger}, utils::is_answering_ping};

use super::ConnectionState;


pub fn local_outage(config: &Config, logger: &mut Logger) -> Option<ConnectionState> {
    if is_answering_ping(&config.router_ip, config.interval_recovery, logger, ConnectionState::LocalOutage) {
        let now = Utc::now();
        logger.add_log_line(format!("🟢 Connection with Router established at {}", now));
        logger.add_log_line(format!(
            "🟢 Local Outage end declared, duration: {} seconds - checking outside connection", logger.log_start.elapsed().as_secs_f64()
        ));
        logger.add_small_separator();
        if is_answering_ping(&config.current_target(), config.interval_recovery, logger, ConnectionState::LocalOutage) {
            let now = Utc::now();
            logger
                .add_log_line(format!("🟢 Outside test connection successful with target '{}' at {}", &config.current_target(), now));
            logger.add_large_separator();
            logger.end_log(format!("🟢 Local Outage end at {}", now));
            logger.add_large_separator();
            let _ = std::fs::remove_file(logger.log_dir_path.clone() + "/local_outage");
            Some(ConnectionState::Online)
        } else {
            logger.add_log_line(format!(
                "🟡 Outside test connection unsuccessful with target '{}' at {}",
                &config.current_target(),
                now
            ));
            logger.add_log_line(format!(
                "Retrying in {} seconds",
                config.interval_recovery.as_secs_f64()
            ));
            logger.add_small_separator();
            thread::sleep(config.interval_recovery);
            if is_answering_ping(&config.current_target(), config.interval_recovery, logger, ConnectionState::LocalOutage) {
                let now = Utc::now();
                logger
                    .end_log(format!("🟢 Outside test connection successful with target '{}' at {}",
                        &config.current_target(),
                        now
                    ));
                let _ = std::fs::remove_file(logger.log_dir_path.clone() + "/local_outage");
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
                let _ = std::fs::remove_file(logger.log_dir_path.clone() + "/local_outage");
                let _ = std::fs::write(logger.log_dir_path.clone() + "/isp_outage", []);
                Some(ConnectionState::IspOutage)
            }
        }
    } else {
        thread::sleep(config.interval_recovery);
        None
    }
}
