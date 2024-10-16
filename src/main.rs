
use std::{collections::VecDeque, error::Error, sync::{atomic::{AtomicBool, Ordering}, Arc}};

use no_internet::no_internet;
use signal_hook::{flag, consts::TERM_SIGNALS};
use sysinfo::Networks;

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
const NO_INTERNET_THRESHOLD: usize = 0;
/// Internet restored threshold in packets over the measurement interval
const INTERNET_RESTORED_THRESHOLD: usize = 10;

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

    let mut last_interval_incoming: VecDeque<usize> = VecDeque::new();
    // used to stop spawning new no_internet threads if no internet is detected, until reconnected
    let internet_restored = Arc::new(AtomicBool::new(true));
    let internet_thread_spawned = Arc::new(AtomicBool::new(false));
    let mut last_total_incoming: u64 = 0;
    let mut initial_loop = true;
    // Main loop
    while !term_now.load(Ordering::Relaxed) {
        // do stuff, fuck bitches

        // 1. update incoming
        let networks = Networks::new_with_refreshed_list();
        for network in networks.iter() {
            println!("{:?}", network.0);
            println!("{:?}", network.1.total_received());
            if initial_loop {
                initial_loop = false;
                last_total_incoming = network.1.total_received();
            } else {
                /* let total_incoming = network.1.total_received();
                let usage = total_incoming - last_total_incoming;
                last_total_incoming = total_incoming; */
                if last_interval_incoming.len() == MEASUREMENT_INTERVAL {
                    last_interval_incoming.pop_front();
                }
                //last_interval_incoming.push_back(usage as usize);
                last_interval_incoming.push_back(network.1.total_received() as usize);
            }
        }

        // 2. calculate if no internet
        if last_interval_incoming.len() == MEASUREMENT_INTERVAL {
            if last_interval_incoming.iter().sum::<usize>() <= NO_INTERNET_THRESHOLD {
                println!("No internet detected");
                println!("{:?}", last_interval_incoming);
                internet_restored.store(false, Ordering::Relaxed);

                // 3. if no internet, call new function for smaller interval check for reconnection
                if !internet_restored.load(Ordering::Relaxed) && !internet_thread_spawned.load(Ordering::Relaxed) {
                    internet_thread_spawned.store(true, Ordering::Relaxed);
                    let term_clone = term_now.clone();
                    let internet_restored_clone = internet_restored.clone();
                    let internet_thread_spawned_clone = internet_thread_spawned.clone();
                    println!("Thread spawned");
                    std::thread::spawn(move || {
                        no_internet(term_clone, internet_restored_clone, internet_thread_spawned_clone);
                    });
                }
            }
        }

        // sleep if no shut down is requested
        if !term_now.load(Ordering::Relaxed) {
            std::thread::sleep(std::time::Duration::from_millis(WAIT_TIME));
        }
    }

    Ok(())
}

