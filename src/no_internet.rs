
use std::{path::PathBuf, sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}};

use nabu::{serde::write, XffValue::{self}};
use time::OffsetDateTime;

use crate::{network_adapter::NetworkTrafficHandler, INTERNET_RESTORED_THRESHOLD, MEASUREMENT_INTERVAL, SHORT_WAIT_TIME, STORAGE_DIR};

pub fn no_internet(term_now: Arc<AtomicBool>, adapter_name: String, main_network_handler: Arc<Mutex<NetworkTrafficHandler>>) {

    // logging data
    let date_time_of_incident = OffsetDateTime::now_utc();
    let start_date = date_time_of_incident.date();
    let start_time = date_time_of_incident.time();
    let time_of_incident = std::time::SystemTime::now();

    let mut internet_detected = false;
    let mut new_network_handler = NetworkTrafficHandler::new(SHORT_WAIT_TIME);

    while !internet_detected {
        // This should shut the function down nicely instead of hoping for a timely loop finish
        if !term_now.load(Ordering::Relaxed) {
            break;
        }
        // 1. update incoming
        new_network_handler.update();

        // 2. calculate if no internet
        for (name, last_interval_incoming) in new_network_handler.load_map.iter() {
            if name == &adapter_name && last_interval_incoming.len() == MEASUREMENT_INTERVAL {
                if last_interval_incoming.iter().sum::<u64>() >= INTERNET_RESTORED_THRESHOLD {
                    internet_detected = true;
                    break;
                }

            }
        }


        // 3. sleep if no internet is detected
        if !term_now.load(Ordering::Relaxed) && !internet_detected {
            std::thread::sleep(std::time::Duration::from_millis(SHORT_WAIT_TIME));
        }
    }
    // log data 2
    let duration_of_incident = time_of_incident.elapsed().unwrap().as_millis();
    let date_time_end_of_incident = OffsetDateTime::now_utc();
    let end_date = date_time_end_of_incident.date();
    let end_time = date_time_end_of_incident.time();
    
    let tmp = format!("Incident Report:
    Start: {start_date} {start_time}
    End: {end_date} {end_time}
    Duration: {duration_of_incident}ms");
    println!("{}", tmp);
    
    // create log
    let xff_value = XffValue::from(vec![("start-date", format!("{}", start_date)), ("start-time", format!("{}", start_time)), ("end-date", format!("{}", end_date)), ("end-time", format!("{}", end_time)), ("duration", format!("{}", duration_of_incident))]);
    let mut log_dir_path = PathBuf::from(STORAGE_DIR);
    log_dir_path.extend(vec![format!("{adapter_name}")]);
    if !log_dir_path.exists() {
        std::fs::create_dir_all(&log_dir_path).expect("Failed to create log directory");
    }
    let mut log_path = log_dir_path.clone();
    log_path.extend(vec![format!("{start_date}-{start_time}.xff")]);
    let write_log = write(log_path, xff_value);
    println!("{:?}", write_log);

    let mut main_network_handler_locked = main_network_handler.try_lock().expect("Failed to lock main network handler");

    let internet_restored = main_network_handler_locked.internet_restored_map.get_mut(&adapter_name).expect("No value to update!");
    *internet_restored = true;
    
    let internet_thread_spawned = main_network_handler_locked.thread_spawned_map.get_mut(&adapter_name).expect("No value to update!");
    *internet_thread_spawned = false;

}
