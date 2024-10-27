
use std::{collections::{BTreeMap, VecDeque}, path::PathBuf, sync::{atomic::{AtomicBool, Ordering}, Arc}};

use nabu::{serde::write, XffValue::{self}};
use horae::Utc;

use crate::{network_adapter::NetworkTrafficHandler, INTERNET_RESTORED_THRESHOLD, MEASUREMENT_INTERVAL, SHORT_WAIT_TIME, STORAGE_DIR};

/// While returning a result, this function will always panic instead of error, and return `Ok(())`
/// if internet has returned
pub fn no_internet(term_now: Arc<AtomicBool>, adapter_name: String, load_map: BTreeMap<String, VecDeque<u64>>) -> Result<(), Box<dyn std::error::Error>> {

    // logging data
    let date_time_of_incident = Utc::now();
    let start_date = date_time_of_incident.date();
    let start_time = date_time_of_incident.time();
    let time_of_incident = std::time::SystemTime::now();

    let mut new_network_handler = NetworkTrafficHandler::new_with_load_map(load_map);

    'outer: while !term_now.load(Ordering::Relaxed) {
        // 1. update incoming
        new_network_handler.update();

        // 2. calculate if no internet
        for (name, last_interval_incoming) in new_network_handler.load_map.iter() {
            if name == &adapter_name && last_interval_incoming.len() == MEASUREMENT_INTERVAL {
                if last_interval_incoming.iter().sum::<u64>() >= INTERNET_RESTORED_THRESHOLD {
                    #[cfg(debug_assertions)]
                    println!("Internet restored! {}", last_interval_incoming.iter().sum::<u64>());
                    #[cfg(debug_assertions)]
                    println!("Interval: {:?}", last_interval_incoming);
                    break 'outer;
                }

            }
        }

        // 3. sleep if no internet is detected
        if !term_now.load(Ordering::Relaxed) {
            std::thread::sleep(std::time::Duration::from_millis(SHORT_WAIT_TIME));
        }
    }
    // log data 2
    let duration_of_incident = time_of_incident.elapsed().unwrap().as_millis();
    let date_time_end_of_incident = Utc::now();
    let end_date = date_time_end_of_incident.date();
    let end_time = date_time_end_of_incident.time();
    
    #[cfg(debug_assertions)]
    let tmp = format!("Incident Report:
    Start: {start_date} {start_time}
    End: {end_date} {end_time}
    Duration: {duration_of_incident}ms");
    #[cfg(debug_assertions)]
    println!("{}", tmp);
    
    // create log
    let xff_value = XffValue::from(vec![("start-date", format!("{}", start_date)), ("start-time", format!("{}", start_time)), ("end-date", format!("{}", end_date)), ("end-time", format!("{}", end_time)), ("duration", format!("{}", duration_of_incident))]);

    let mut log_dir_path = PathBuf::from(STORAGE_DIR);
    log_dir_path.extend(vec![adapter_name]);
    if !log_dir_path.exists() {
        std::fs::create_dir_all(&log_dir_path).expect("Failed to create log directory");
    }
    let mut log_path = log_dir_path.clone();
    log_path.extend(vec![format!("{start_date}-{start_time}.xff")]);

    let out = write(log_path, xff_value);
    if out.is_err() {
        panic!("Failed to write log: {}", out.unwrap_err());
    } else {
        return Ok(());
    }

}

