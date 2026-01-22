use std::thread;

use horae::Utc;

use crate::{config::Config, log::{EventType, Logger}, utils::is_answering_ping};

use super::ConnectionState;


pub fn diagnosing(config: &mut Config, logger: &mut Logger) -> ConnectionState {
    let now = Utc::now();
    logger.add_log_line(format!("🟡 {} - Diagnosing", now));
    logger.add_small_separator();

    if is_answering_ping(&config.router_ip, config.interval_recovery, logger) {

        let mut check_list = vec![];
        check_list.push(is_answering_ping(&config.current_target(), config.interval_recovery, logger));
        config.iter_targets();
        thread::sleep(config.interval_recovery);
        check_list.push(is_answering_ping(&config.current_target(), config.interval_recovery, logger));
        config.iter_targets();
        thread::sleep(config.interval_recovery);
        check_list.push(is_answering_ping(&config.current_target(), config.interval_recovery, logger));
        config.iter_targets();
        thread::sleep(config.interval_recovery);
        check_list.push(is_answering_ping(&config.current_target(), config.interval_recovery, logger));
        config.iter_targets();
        thread::sleep(config.interval_recovery);
        check_list.push(is_answering_ping(&config.current_target(), config.interval_recovery, logger));
        config.iter_targets();

        if check_list.iter().any(|b| b == &true) {
            logger.clear();
            ConnectionState::Online
        } else {
            let now = Utc::now();
            logger.add_log_line("🔴 Mr. President, 5 more targets have failed to answer - we are cut off".to_string());
            logger.add_log_line(format!("🔴 Declaring ISP outage at {}", now));
            logger.add_large_separator();
            logger.event_type = EventType::IspOutage;
            ConnectionState::IspOutage
        }

    } else {
        let now = Utc::now();
        logger.add_log_line(format!("🔴 Router is down"));
        logger.add_log_line(format!(
            "🔴 Declaring local outage at {} - Roll the Trucks!",
            now
        ));
        logger.add_large_separator();
        logger.event_type = EventType::LocalOutage;
        ConnectionState::LocalOutage
    }
}
