use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use config::Config;
use horae::Utc;
use log::Logger;
use signal_hook::{consts::TERM_SIGNALS, flag};
use states::{diagnosing, isp_outage, local_outage, online, complete_network_outage, ConnectionState};

mod config;
mod log;
mod states;
mod utils;

fn main() {
    let term_now = Arc::new(AtomicBool::new(false));
    for signal in TERM_SIGNALS {
        // significant error if signal cannot be registered
        // It compromises the shutdown and with that partial writes to disk
        if let Err(e) = flag::register(*signal, Arc::clone(&term_now)) {
            panic!("Unable to register signal handler - Error: {}", e);
        }
    }

    let mut config = Config::new();
    let mut state = ConnectionState::Online;
    let mut logger = Logger::new(config.log_dir_path.clone());

    while !term_now.load(Ordering::Relaxed) {
        config.iter_targets();

        match state {
            ConnectionState::Online => {
                if let Some(new_state) = online(&config, &mut logger) {
                    state = new_state;
                }
            }
            ConnectionState::Diagnosing => {
                state = diagnosing(&mut config, &mut logger);
            }
            ConnectionState::IspOutage => {
                if let Some(new_state) = isp_outage(&config, &mut logger) {
                    state = new_state;
                }
            }
            ConnectionState::LocalOutage => {
                if let Some(new_state) = local_outage(&config, &mut logger) {
                    state = new_state;
                }
            }
            ConnectionState::CompleteNetworkOutage => {
                if let Some(new_state) = complete_network_outage(&config, &mut logger) {
                    state = new_state;
                }
            }
        }
    }

    // ------ SHUT DOWN CODE --------
    if logger.has_unsaved_log() && state != ConnectionState::Online {
        let now = Utc::now();
        logger.end_log(format!("Shamash shutting down at {}", now));
    }
}

fn next_index(index: usize, len: usize) -> usize {
    (index + 1) % len
}
