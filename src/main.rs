
use std::{collections::VecDeque, error::Error, sync::{atomic::{AtomicBool, Ordering}, Arc}};

use no_internet::no_internet;
use signal_hook::{flag, consts::TERM_SIGNALS};
use sysinfo::Networks;

mod no_internet;

/// 1000 == 1 second of sleep / wait
const WAIT_TIME: u64 = 1_000;
/// Storage directory
const STORAGE_DIR: &str = "./shamash-logs";
/// Measurement interval in seconds
const MEASUREMENT_INTERVAL: usize = 60;
/// No internet threshold in packets over the measurement interval
const NO_INTERNET_THRESHOLD: usize = 0;

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

    let mut last_minute_incoming: VecDeque<usize> = VecDeque::new();
    // used to stop spawning new no_internet threads if no internet is detected, until reconnected
    let internet_restored = Arc::new(AtomicBool::new(true));
    // Main loop
    while !term_now.load(Ordering::Relaxed) {
        // do stuff, fuck bitches

        // 1. update incoming
        let networks = Networks::new_with_refreshed_list();
        for network in networks.iter() {
            let usage: usize = network.1.packets_received() as usize;
            if last_minute_incoming.len() == MEASUREMENT_INTERVAL {
                last_minute_incoming.pop_front();
            }
            last_minute_incoming.push_back(usage);
        }

        // 2. calculate if no internet
        let mut no_internet_detected = false;
        if last_minute_incoming.len() != MEASUREMENT_INTERVAL {
            continue;
        } else {
            if last_minute_incoming.iter().sum::<usize>() == NO_INTERNET_THRESHOLD {
                no_internet_detected = true;
            }
        }

        
        // 3. if no internet, call new function for smaller interval check for reconnection
        if no_internet_detected && !internet_restored.load(Ordering::Relaxed) {
            let term_clone = term_now.clone();
            let internet_restored_clone = internet_restored.clone();
            std::thread::spawn(move || {
                no_internet(term_clone, internet_restored_clone);
            });
        }

        // sleep if no shut down is requested
        if !term_now.load(Ordering::Relaxed) {
            std::thread::sleep(std::time::Duration::from_millis(WAIT_TIME));
        }
    }

    Ok(())
}

