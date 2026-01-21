use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread};

use config::Config;
use horae::Utc;
use log::Logger;
use signal_hook::{consts::TERM_SIGNALS, flag};
use utils::{is_answering_ping, ConnectionState};

mod utils;
mod config;
mod log;

fn main() {
    let term_now = Arc::new(AtomicBool::new(false));
    for signal in TERM_SIGNALS {
        // significant error if signal cannot be registered
        // It compromises the shutdown and with that partial writes to disk
        if let Err(e) = flag::register(*signal, Arc::clone(&term_now)) {
            panic!("Unable to register signal handler - Error: {}", e);
        }
    }

    let config = Config::new();
    let mut state = ConnectionState::Online;
    let mut logger = Logger::new(config.log_dir_path);
    let mut target_index = 0;

    while !term_now.load(Ordering::Relaxed) {
        let target = &config.targets[target_index];
        target_index = next_index(target_index, config.targets.len());

        match state {
            ConnectionState::Online => {
                if is_answering_ping(target, config.interval_normal) {
                    thread::sleep(config.interval_normal);
                }
                else {
                    let now = Utc::now();
                    logger.add_log_line(format!("{}", now));
                    logger.add_log_line(format!("Target '{}' failed to answer", target));
                    logger.add_separator();
                    state = ConnectionState::Diagnosing;
                }
            }
            ConnectionState::Diagnosing => {
                let now = Utc::now();
                logger.add_log_line(format!("{} - Diagnosing", now));
                logger.add_separator();

                if is_answering_ping(&config.router_ip, config.interval_recovery) {
                    // Router up, external seems out - sanity check
                    // Normal interval to give every benefit
                    if is_answering_ping(target, config.interval_normal) {
                        logger.clear();
                        state = ConnectionState::Online;
                        continue
                    }
                    else {
                        let now = Utc::now();
                        logger.add_log_line(format!("Mr. President: A second target ({}) failed to answer - The local network is under attack.", target));
                        logger.add_log_line(format!("Declaring ISP outage at {}", now));
                        state = ConnectionState::IspOutage;
                    }
                } else {
                    let now = Utc::now();
                    logger.add_log_line(format!("Router is down"));
                    logger.add_log_line(format!("Declaring local outage at {} - Roll the Trucks!", now));
                    state = ConnectionState::LocalOutage;
                }
                logger.add_separator();
            }
            ConnectionState::IspOutage => {
                if is_answering_ping(target, config.interval_recovery) {
                    let now = Utc::now();
                    logger.add_log_line(format!("Connection established with target '{}' at {}", target, now));
                    let next_target = &config.targets[next_index(target_index, config.targets.len())];
                    if is_answering_ping(&next_target, config.interval_recovery) {
                        let now = Utc::now();
                        logger.end_log(format!("Connection established with second target '{}' at {}", next_target, now));
                        state = ConnectionState::Online;
                    } else {
                        // One target reports up, the other down - Should be unreachable, but I
                        // guess we'll just run the loop again.
                        continue;
                    }
                } else {
                    thread::sleep(config.interval_recovery);
                }
            }
            ConnectionState::LocalOutage => {
                if is_answering_ping(&config.router_ip, config.interval_recovery) {
                    let now = Utc::now();
                    logger.add_log_line(format!("Connection with Router established at {}", now));
                    logger.add_log_line(format!("Local Outage end declared - checking outside connection"));
                    if is_answering_ping(&target, config.interval_recovery) {
                        let now = Utc::now();
                        logger.add_log_line(format!("Outside test connection successful at {}", now));
                        logger.end_log(format!("Local Outage end at {}", now));
                        state = ConnectionState::Online;
                    } else {
                        logger.add_log_line(format!("Outside test connection unsuccessful at {}", now));
                        logger.add_log_line(format!("Retrying in {} seconds", config.interval_recovery.as_secs()));
                        thread::sleep(config.interval_recovery);
                        if is_answering_ping(&target, config.interval_recovery) { 
                            let now = Utc::now();
                            logger.end_log(format!("Outside test connection successful at {}", now));
                            state = ConnectionState::Online;
                        } else { 
                            let now = Utc::now();
                            logger.add_log_line(format!("Outside test connection unsuccessful at {}", now));
                            logger.add_log_line(format!("Declaring ISP outage at {}, continuing the outage", now));
                            logger.add_separator();
                            state = ConnectionState::IspOutage; 
                        } 
                    }
                } else {
                    thread::sleep(config.interval_recovery);
                }
            }
        }
    }
}

fn next_index(index: usize, len: usize) -> usize {
    (index + 1) % len
}
