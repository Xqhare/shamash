use std::{error::Error, sync::{atomic::{AtomicBool, Ordering}, Arc}, thread};

use network_adapter::NetworkTrafficHandler;
use no_internet::no_internet;
use signal_hook::{flag, consts::TERM_SIGNALS};

mod no_internet;
mod network_adapter;

/// 1000 == 1 second of sleep / wait
const WAIT_TIME: u64 = 1_000;
/// 100 == 0.1 second of sleep / wait
/// used for no internet check
const SHORT_WAIT_TIME: u64 = 250;
/// Storage directory
const STORAGE_DIR: &str = "./shamash-logs";
/// Measurement interval in seconds
const MEASUREMENT_INTERVAL: usize = 60;
/// No internet threshold in packets over the measurement interval
const NO_INTERNET_THRESHOLD: u64 = 0;
/// Internet restored threshold in packets over the measurement interval
const INTERNET_RESTORED_THRESHOLD: u64 = 10;

fn main() -> Result<(), Box<dyn Error>> {
    // largely SIGTERM handling
    let term_now = Arc::new(AtomicBool::new(false));
    for signal in TERM_SIGNALS {
        // in eris this code has run for months on end without any problems...
        // definitely not production ready, but good enough for now
        flag::register_conditional_shutdown(*signal, 1, Arc::clone(&term_now)).expect("Failed to set conditional shutdown flag");
        // Order of the two is important
        flag::register(*signal, Arc::clone(&term_now)).expect("Failed to set shutdown flag");
    }

    let mut network_handler = NetworkTrafficHandler::new(WAIT_TIME);
    let mut no_internet_monitored: Vec<String> = Vec::new();
    // Main loop
    while !term_now.load(Ordering::Relaxed) {
        // do stuff, fuck bitches

        // 1. update incoming
        network_handler.update();

        let tmp_network_bind = network_handler.load_map.clone();

        // 2. calculate if no internet
        for name in network_handler.active_adapters.clone() {
            if let Some(last_interval_incoming) = tmp_network_bind.get(&name) {
                if last_interval_incoming.len() == MEASUREMENT_INTERVAL {
                    println!("Buffer full. Incoming bytes: {}", last_interval_incoming.iter().sum::<u64>());
                    if last_interval_incoming.iter().sum::<u64>() <= NO_INTERNET_THRESHOLD && !no_internet_monitored.contains(&name) {
                        no_internet_monitored.push(name.clone());
                        no_internet(term_now.clone(), name, tmp_network_bind.clone());
                    }
                }
            }
        }
        
        // sleep if no shut down is requested
        if !term_now.load(Ordering::Relaxed) {
            thread::sleep(std::time::Duration::from_millis(WAIT_TIME));
        }
    }

    Ok(())
}

