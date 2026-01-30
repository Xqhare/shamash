use horae::Utc;

use crate::{
    config::Config,
    log::{EventType, Logger},
    utils::is_answering_ping,
};

use super::{isp_outage::write_isp_outage_file, local_outage::write_local_outage_file, sleep_outage, ConnectionState};

const COMPLETE_NETWORK_OUTAGE_FILE: &str = "/complete_network_outage_ongoing";

pub fn complete_network_outage(config: &Config, logger: &mut Logger) -> Option<ConnectionState> {
    if is_answering_ping(
        &config
            .secondary_internal_target
            .clone()
            .expect("Complete network outage only reachable with secondary internal target set"),
        config.interval_recovery,
        logger,
        &ConnectionState::CompleteNetworkOutage,
    ) {
        Some(end_complete_network_outage(config, logger))
    } else {
        // Ping timeout - 50ms to prevent tight looping
        sleep_outage()
    }
}

fn end_complete_network_outage(config: &Config, logger: &mut Logger) -> ConnectionState {
    let now = Utc::now();

    logger.add_log_line(format!(
        "🟢 Connection with secondary internal target established at {now}"
    ));
    logger.add_small_separator();
    logger.add_log_line(format!("Checking router at {}", &config.router_ip));

    if is_answering_ping(
        &config.router_ip,
        config.interval_recovery,
        logger,
        &ConnectionState::CompleteNetworkOutage,
    ) {
        if is_answering_ping(
            &config.current_target(),
            config.interval_recovery,
            logger,
            &ConnectionState::CompleteNetworkOutage,
        ) {
            move_to_online(logger)
        } else {
            move_to_isp_outage(logger)
        }
    } else {
        move_to_local_outage(logger)
    }
}

fn move_to_local_outage(logger: &mut Logger) -> ConnectionState {
    let now = Utc::now();

    logger.add_small_separator();
    logger.add_log_line(format!("🔴 Declaring local outage at {now}"));
    logger.add_large_separator();
    logger.event_type = EventType::LocalOutage;

    delete_complete_network_outage_file(&logger.log_dir_path);
    write_local_outage_file(&logger.log_dir_path);

    ConnectionState::LocalOutage
}

fn move_to_isp_outage(logger: &mut Logger) -> ConnectionState {
    let now = Utc::now();

    logger.add_small_separator();
    logger.add_log_line(format!("🔴 Declaring ISP outage at {now}"));
    logger.add_large_separator();
    logger.event_type = EventType::IspOutage;

    delete_complete_network_outage_file(&logger.log_dir_path);
    write_isp_outage_file(&logger.log_dir_path);

    ConnectionState::IspOutage
}

fn move_to_online(logger: &mut Logger) -> ConnectionState {
    let now = Utc::now();

    logger.add_small_separator();
    logger.add_log_line(format!("🟢 Declaring router online at {now}"));
    logger.add_large_separator();
    logger.event_type = EventType::Online;

    delete_complete_network_outage_file(&logger.log_dir_path);

    ConnectionState::Online
}

pub fn write_complete_network_outage_file(path: &str) {
    let _ = std::fs::write(path.to_owned() + COMPLETE_NETWORK_OUTAGE_FILE, []);
}

pub fn delete_complete_network_outage_file(path: &str) {
    let _ = std::fs::remove_file(path.to_owned() + COMPLETE_NETWORK_OUTAGE_FILE);
}
