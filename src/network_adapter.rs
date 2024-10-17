use std::{collections::{BTreeMap, VecDeque}, thread, time::Duration};

use sysinfo::Networks;

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
        let mut networks = Networks::new_with_refreshed_list();

        let mut last_received_map = BTreeMap::new();

        for network in networks.iter() {
            let name = network.0.to_string();
            if name == "lo" { continue; }
            let last_received = network.1.total_received();
            last_received_map.insert(name.clone(), last_received);
        }

        // wait for refresh to have new data
        thread::sleep(Duration::from_millis(wait_time));
        networks.refresh();

        let mut load_map: BTreeMap<String, VecDeque<u64>> = BTreeMap::new();
        let mut active_adapters: Vec<String> = Vec::new();

        for network in networks.iter() {
            let name = network.0.to_string();
            if name == "lo" { continue; }
            let now_received = network.1.total_received();
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
        let networks = Networks::new_with_refreshed_list();
        for network in networks.iter() {
            let name = network.0.to_string();
            if name == "lo" { continue; }
            let now_received = network.1.total_received();
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

