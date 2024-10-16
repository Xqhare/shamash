use std::{error::Error, sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, thread};

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

    let network_handler = Arc::new(Mutex::new(NetworkTrafficHandler::new(WAIT_TIME)));
    // Main loop
    while !term_now.load(Ordering::Relaxed) {
        // do stuff, fuck bitches

        // 0. lock the network handler
        let mut network_handler_locked = network_handler.try_lock().expect("Failed to lock network handler");
        let tmp_network_bind = network_handler_locked.load_map.clone();

        // 1. update incoming
        network_handler_locked.update();

        // 2. calculate if no internet
        for (name, last_interval_incoming) in tmp_network_bind.into_iter() {
            // A full measurement interval needs to be done
            // -> the first few seconds could just be no network activity
            // the full interval should capture some activity be it simple update requests or smth
            if last_interval_incoming.len() == MEASUREMENT_INTERVAL {
                if last_interval_incoming.iter().sum::<u64>() <= NO_INTERNET_THRESHOLD && !network_handler_locked.thread_spawned_map.get(&name).expect("No value to update!"){
                    let internet_restored = network_handler_locked.internet_restored_map.get_mut(&name).expect("No value to update!");
                    *internet_restored = false;
                    let thread_spawned = network_handler_locked.thread_spawned_map.get_mut(&name).expect("No value to update!");
                    *thread_spawned = true;

                    println!("Thread spawned");
                    let term_clone = term_now.clone();
                    let network_handler_clone = network_handler.clone();
                    thread::spawn(move || {
                        no_internet(term_clone, name.to_string(), network_handler_clone);
                    });
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

