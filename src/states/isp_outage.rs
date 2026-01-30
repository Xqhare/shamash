use horae::Utc;

use crate::{config::Config, log::Logger, utils::is_answering_ping};

use super::{sleep_outage, ConnectionState};

const ISP_OUTAGE_FILE: &str = "/isp_outage_ongoing";

pub fn write_isp_outage_file(path: &str) {
    let _ = std::fs::write(path.to_owned() + ISP_OUTAGE_FILE, []);
}

pub fn delete_isp_outage_file(path: &str) {
    let _ = std::fs::remove_file(path.to_owned() + ISP_OUTAGE_FILE);
}

pub fn isp_outage(config: &Config, logger: &mut Logger) -> Option<ConnectionState> {
    if is_answering_ping(
        &config.current_target(),
        config.interval_recovery,
        logger,
        &ConnectionState::IspOutage,
    ) {
        let now = Utc::now();

        logger.add_log_line(format!(
            "🟢 Connection established with target '{}' at {}",
            &config.current_target(),
            now
        ));

        secondary_connection_test(config, logger)
    } else {
        // Ping timeout - 50ms to prevent tight looping
        sleep_outage()
    }
}

fn secondary_connection_test(config: &Config, logger: &mut Logger) -> Option<ConnectionState> {
    if is_answering_ping(
        &config.next_target(),
        config.interval_recovery,
        logger,
        &ConnectionState::IspOutage,
    ) {
        secondary_test_successful(config, logger)
    } else {
        secondary_test_unsuccessful(config, logger)
    }
}

fn secondary_test_successful(config: &Config, logger: &mut Logger) -> Option<ConnectionState> {
    let now = Utc::now();

    logger.end_log(format!(
        "🟢 Connection established with second target '{}' at {}",
        &config.next_target(),
        now
    ));

    delete_isp_outage_file(&logger.log_dir_path);

    Some(ConnectionState::Online)
}

fn secondary_test_unsuccessful(config: &Config, logger: &mut Logger) -> Option<ConnectionState> {
    let now = Utc::now();

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
