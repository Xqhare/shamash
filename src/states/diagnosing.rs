use std::thread;

use horae::Utc;

use crate::{
    config::Config,
    log::{EventType, Logger},
    utils::is_answering_ping,
};

use super::{isp_outage::write_isp_outage_file, local_outage::write_local_outage_file, complete_network_outage::write_complete_network_outage_file, ConnectionState};

const DIAGNOSING_FILE: &str = "/diagnosing";

pub fn write_diagnosing_file(path: &str) {
    let _ = std::fs::write(path.to_owned() + DIAGNOSING_FILE, []);
}

pub fn delete_diagnosing_file(path: &str) {
    let _ = std::fs::remove_file(path.to_owned() + DIAGNOSING_FILE);
}

pub fn diagnosing(config: &mut Config, logger: &mut Logger) -> ConnectionState {
    let now = Utc::now();

    logger.add_log_line(format!("🟡 Diagnosing - {}", now));
    logger.add_small_separator();
    logger.add_log_line(format!("Checking router at {}", &config.router_ip));

    if is_answering_ping(
        &config.router_ip,
        config.interval_recovery,
        logger,
        ConnectionState::Diagnosing,
    ) {
        diagnose_isp(config, logger)
    } else {
        diagnose_local_outage(config, logger)
    }
}

fn diagnose_isp(config: &mut Config, logger: &mut Logger) -> ConnectionState {
    logger.add_small_separator();

    // Way to complicated - wanted to move away form magic number `5`
    let target_amount = config.targets.len().div_ceil(3);

    for _ in 0..target_amount {
        if is_answering_ping(
            &config.current_target(),
            config.interval_recovery,
            logger,
            ConnectionState::Diagnosing,
        ) {
            // If just one outside target answers, we're good
            return move_to_online(logger);
        }
        config.iter_targets();
        thread::sleep(config.interval_recovery);
    }

    logger.add_small_separator();
    logger.add_log_line(
        format!(
            "🔴 Mr. President, {} more targets have failed to answer",
            target_amount
        )
    );

    move_to_isp_outage(logger)
}

fn move_to_online(logger: &mut Logger) -> ConnectionState {
    logger.reset();

    delete_diagnosing_file(&logger.log_dir_path);
    
    ConnectionState::Online
}

fn move_to_isp_outage(logger: &mut Logger) -> ConnectionState {
    let now = Utc::now();

    logger.add_log_line(format!("🔴 Declaring ISP outage at {}", now));
    logger.add_large_separator();
    logger.event_type = EventType::IspOutage;
    
    delete_diagnosing_file(&logger.log_dir_path);
    write_isp_outage_file(&logger.log_dir_path);
    
    ConnectionState::IspOutage
}

fn diagnose_local_outage(config: &mut Config, logger: &mut Logger) -> ConnectionState {
    logger.add_small_separator();
    logger.add_log_line(format!("🔴 Router is down"));
    logger.add_small_separator();

    if let Some(target) = config.secondary_internal_target.clone() {
        check_secondary_target(config, logger, &target)
    } else {
        move_to_local_outage(logger)
    }
}

fn check_secondary_target(config: &mut Config, logger: &mut Logger, target: &str) -> ConnectionState {
    let now = Utc::now();

    logger.add_log_line(format!("Checking secondary internal target '{}' at {}", target, now));
    logger.add_small_separator();

    if is_answering_ping(
        target,
        config.interval_recovery,
        logger,
        ConnectionState::Diagnosing,
    ) {
        secondary_check_successful(config, logger)

    } else {
        secondary_check_unsuccessful(config, logger)
    }
}

fn secondary_check_successful(config: &mut Config, logger: &mut Logger) -> ConnectionState {
    let now = Utc::now();

    logger.add_log_line(format!(
        "🟢 Connection established with secondary internal target '{}' at {}",
        &config.next_target(),
        now
    ));
    logger.add_small_separator();
    logger.add_log_line(format!("Retrying connection with Router at {} in {} seconds", &config.router_ip, config.interval_recovery.as_secs_f64()));
    logger.add_small_separator();

    thread::sleep(config.interval_recovery);

    if is_answering_ping(
        &config.router_ip,
        config.interval_recovery,
        logger,
        ConnectionState::Diagnosing,
    ) {
        move_to_online(logger)
    } else {
        logger.add_log_line("🟢 🔴 Local network up - ISP Router down".to_string());
        move_to_local_outage(logger)
    }
}

fn secondary_check_unsuccessful(config: &mut Config, logger: &mut Logger) -> ConnectionState {
    let now = Utc::now();

    logger.add_log_line(format!(
        "🔴 Connection not established with secondary internal target '{}' at {}",
        &config.next_target(),
        now
    ));
    logger.add_log_line("🔴 Declaring total network outage".to_string());
    logger.add_small_separator();

    move_to_complete_network_outage(logger)
}

fn move_to_complete_network_outage(logger: &mut Logger) -> ConnectionState {
    let now = Utc::now();

    logger.add_small_separator();
    logger.add_log_line(format!(
        "🔴 Declaring router outage at {} - Roll the Trucks!",
        now
    ));
    logger.add_large_separator();
    logger.event_type = EventType::CompleteNetworkOutage;

    delete_diagnosing_file(&logger.log_dir_path);
    write_complete_network_outage_file(&logger.log_dir_path);
    
    ConnectionState::CompleteNetworkOutage
}

fn move_to_local_outage(logger: &mut Logger) -> ConnectionState {
    let now = Utc::now();

    logger.add_small_separator();
    logger.add_log_line(format!(
        "🔴 Declaring local outage at {} - Roll the Trucks!",
        now
    ));
    logger.add_large_separator();
    logger.event_type = EventType::LocalOutage;

    delete_diagnosing_file(&logger.log_dir_path);
    write_local_outage_file(&logger.log_dir_path);
    
    ConnectionState::LocalOutage
}
