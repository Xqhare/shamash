use std::{collections::{BTreeMap, VecDeque}, fs::File, io::Read, path::Path, thread, time::Duration};

use crate::{MEASUREMENT_INTERVAL, SHORT_WAIT_TIME};


#[derive(Clone, Debug)]
/// Network traffic handler
/// Used to store and update network traffic of all adapters
/// Always constructed with new
/// Call update() periodically
pub struct NetworkTrafficHandler {
    pub load_map: BTreeMap<String, VecDeque<u64>>,
    pub active_adapters: Vec<String>,
    last_received_map: BTreeMap<String, u64>,
}

impl NetworkTrafficHandler {
    /// Create new network traffic handler with a wait time in milliseconds
    pub fn new(wait_time: u64) -> Self {

        let mut last_received_map = BTreeMap::new();

        for network in get_network_load_from_procfs() {
            let name = network.0;
            if name == "lo" { continue; }
            let last_received = network.1;
            last_received_map.insert(name.clone(), last_received);
        }

        // wait for refresh to have new data
        thread::sleep(Duration::from_millis(wait_time));

        let mut load_map: BTreeMap<String, VecDeque<u64>> = BTreeMap::new();
        let mut active_adapters: Vec<String> = Vec::new();

        for network in get_network_load_from_procfs() {
            let name = network.0;
            if name == "lo" { continue; }
            let now_received = network.1;
            let load = now_received.saturating_sub(*last_received_map.get(&name).expect("No last value!"));
            if load > 0 {
                active_adapters.push(name.clone());
            }
            load_map.insert(name, VecDeque::from([load]));
        }

        Self {
            load_map,
            active_adapters,
            last_received_map
        }
    }

    /// only called by no_internet
    pub fn new_with_load_map(load_map: BTreeMap<String, VecDeque<u64>>) -> Self {
        // Will only be ever called by no_internet: SHORT_WAIT_TIME
        let mut default_self = Self::new(SHORT_WAIT_TIME);
        default_self.load_map = load_map;
        default_self
    }

    pub fn update(&mut self) {
        for network in get_network_load_from_procfs() {
            let name = network.0;
            if name == "lo" { continue; }
            let now_received = network.1;
            let last_received = self.last_received_map.insert(name.clone(), now_received).expect("No value to update!");
            let load = now_received.saturating_sub(last_received);
            if load > 0 && !self.active_adapters.contains(&name) {
                self.active_adapters.push(name.clone());
            }
            #[cfg(debug_assertions)]
            println!("{name} -> {now_received} - {last_received} = {load}");
            if self.load_map.get(&name).expect("No value found!").len() == MEASUREMENT_INTERVAL {
                let load_vec = self.load_map.get_mut(&name).expect("No value to update!");
                load_vec.pop_front();
                load_vec.push_back(load);
            } else {
                let load_vec = self.load_map.get_mut(&name).expect("No value to update!");
                load_vec.push_back(load);
            }
        }
    }
}

fn get_network_load_from_procfs() -> Vec<(String, u64)> {
    const PROCFS: &str = "/proc/net";
    let mut out: Vec<(String, u64)> = Vec::new();
    // read all channel bond interfaces (network adapter pools)
    let all_dirs = Path::new(PROCFS).read_dir().expect("Failed to read procfs");
    for dir in all_dirs {
        let dir = dir.expect("Failed to read procfs");
        if dir.path().is_dir() && dir.path().file_name().unwrap().to_str().unwrap().contains("dev") {
            for file in dir.path().read_dir().expect("Failed to read procfs") {
                let file = file.expect("Failed to read procfs");
                let mut tmp_file = File::open(file.path()).expect("Failed to read procfs");
                let mut buffer = String::new();
                tmp_file.read_to_string(&mut buffer).expect("Failed to read procfs");
                for line in buffer.lines() {
                    if line.contains("Ip6InReceives") {
                        if file.path().file_name().expect("Failed to read procfs").to_str().unwrap() != "lo" {
                            out.push(
                                (
                                    file.path().file_name().expect("Failed to read procfs").to_str().unwrap().to_string(), 
                                    line.split_whitespace().nth(1).expect("Failed to read procfs").parse().expect("Failed to read procfs")
                                )
                            );
                        }
                    }
                }
            }
        }
    }
    out
}
