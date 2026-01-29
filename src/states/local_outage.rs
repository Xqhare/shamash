use std::{thread, time::Duration};

use horae::Utc;

use crate::{
    config::Config,
    log::{EventType, Logger},
    utils::is_answering_ping,
};

use super::{isp_outage::write_isp_outage_file, ConnectionState};

pub fn write_local_outage_file(path: &str) {
    let _ = std::fs::write(path.to_owned() + "/local_outage_ongoing", []);
}

pub fn delete_local_outage_file(path: &str) {
    let _ = std::fs::remove_file(path.to_owned() + "/local_outage_ongoing");
}

pub fn local_outage(config: &Config, logger: &mut Logger) -> Option<ConnectionState> {
    if is_answering_ping(
        &config.router_ip,
        config.interval_recovery,
        logger,
        ConnectionState::LocalOutage,
    ) {
        let now = Utc::now();
        
        logger.add_log_line(format!("🟢 Connection with Router established at {}", now));
        logger.add_log_line(format!(
            "🟢 Local Outage end declared, duration: {} seconds - checking outside connection",
            logger.log_start.elapsed().as_secs_f64()
        ));
        logger.add_small_separator();

        test_outside_connection(config, logger)
    } else {
        local_outage_sleep(config.interval_recovery)
    }
}

fn test_outside_connection(config: &Config, logger: &mut Logger) -> Option<ConnectionState> {
    if is_answering_ping(
        &config.current_target(),
        config.interval_recovery,
        logger,
        ConnectionState::LocalOutage,
    ) {
        test_outside_connection_successful(config, logger)
    } else {
        test_outside_connection_unsuccessful(config, logger)
    }
}

fn test_outside_connection_successful(config: &Config, logger: &mut Logger) -> Option<ConnectionState> {
    let now = Utc::now();

    logger.add_log_line(format!(
        "🟢 Outside test connection successful with target '{}' at {}",
        &config.current_target(),
        now
    ));
    logger.add_large_separator();
    logger.end_log(format!("🟢 Local Outage end at {}", now));
    logger.add_large_separator();
    
    delete_local_outage_file(&logger.log_dir_path);
    
    Some(ConnectionState::Online)
}

fn test_outside_connection_unsuccessful(config: &Config, logger: &mut Logger) -> Option<ConnectionState> {
    let now = Utc::now();
    
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

    if is_answering_ping(
        &config.current_target(),
        config.interval_recovery,
        logger,
        ConnectionState::LocalOutage,
    ) {
        move_to_online(config, logger)
    } else {
        move_to_isp_outage(config, logger)
    }
}

fn move_to_online(config: &Config, logger: &mut Logger) -> Option<ConnectionState> {
    let now = Utc::now();
    
    logger.end_log(format!(
        "🟢 Outside test connection successful with target '{}' at {}",
        &config.current_target(),
        now
    ));

    delete_local_outage_file(&logger.log_dir_path);
    Some(ConnectionState::Online)
}

fn local_outage_sleep(dur: Duration) -> Option<ConnectionState> {
    thread::sleep(dur);
    None
}

fn move_to_isp_outage(config: &Config, logger: &mut Logger) -> Option<ConnectionState> {
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

    delete_local_outage_file(&logger.log_dir_path);
    write_isp_outage_file(&logger.log_dir_path);

    Some(ConnectionState::IspOutage)
}
