
use std::{collections::VecDeque, path::PathBuf, sync::{atomic::{AtomicBool, Ordering}, Arc}};

use nabu::{serde::write, XffValue::{self}};
use sysinfo::Networks;
use time::OffsetDateTime;

use crate::{MEASUREMENT_INTERVAL, STORAGE_DIR, SHORT_WAIT_TIME, INTERNET_RESTORED_THRESHOLD};

pub fn no_internet(term_now: Arc<AtomicBool>, internet_restored: Arc<AtomicBool>) {
    // interval == (60 / SHORT_WAIT_TIME) * MEASUREMENT_INTERVAL
    // interval == (60 / 250) * 60
    // interval == 10 seconds
    let mut last_interval_incoming: VecDeque<usize> = VecDeque::new();
    let mut internet_detected = false;

    // logging data
    let date_time_of_incident = OffsetDateTime::now_utc();
    let start_date = date_time_of_incident.date();
    let start_time = date_time_of_incident.time();
    let time_of_incident = std::time::SystemTime::now();

    // This should shut the function down nicely instead of hoping for a timely loop finish
    while !term_now.load(Ordering::Relaxed) && !internet_detected {

        // 1. update incoming
        let networks = Networks::new_with_refreshed_list();
        for network in networks.iter() {
            let usage: usize = network.1.packets_received() as usize;
            if last_interval_incoming.len() == MEASUREMENT_INTERVAL {
                last_interval_incoming.pop_front();
            }
            last_interval_incoming.push_back(usage);
        }

        // 2. calculate if internet detected
        if last_interval_incoming.len() != MEASUREMENT_INTERVAL {
            continue;
        } else {
            if last_interval_incoming.iter().sum::<usize>() >= INTERNET_RESTORED_THRESHOLD {
                internet_detected = true;
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
    let log_dir_path = PathBuf::from(STORAGE_DIR);
    if !log_dir_path.exists() {
        std::fs::create_dir_all(&log_dir_path).expect("Failed to create log directory");
    }
    let mut log_path = log_dir_path.clone();
    log_path.extend(vec![format!("{start_date}-{start_time}.xff")]);
    let write_log = write(log_path, xff_value);
    println!("{:?}", write_log);

    internet_restored.store(true, Ordering::Relaxed);
}
