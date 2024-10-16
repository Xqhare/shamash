use std::{collections::{BTreeMap, VecDeque}, thread, time::Duration};

use sysinfo::Networks;

use crate::MEASUREMENT_INTERVAL;


#[derive(Clone, Debug)]
/// Network traffic handler
/// Used to store and update network traffic of all adapters
/// Always constructed with new
/// Call update() periodically
pub struct NetworkTrafficHandler {
    pub load_map: BTreeMap<String, VecDeque<u64>>,
    pub thread_spawned_map: BTreeMap<String, bool>,
    pub internet_restored_map: BTreeMap<String, bool>,
    last_received_map: BTreeMap<String, u64>,
}

impl NetworkTrafficHandler {
    /// Create new network traffic handler with a wait time in milliseconds
    pub fn new(wait_time: u64) -> Self {
        let mut networks = Networks::new_with_refreshed_list();

        let mut last_received_map = BTreeMap::new();
        let mut thread_spawned_map = BTreeMap::new();
        let mut internet_restored_map = BTreeMap::new();

        for network in networks.iter() {
            let name = network.0.to_string();
            if name == "lo" { continue; }
            let last_received = network.1.total_received();
            last_received_map.insert(name.clone(), last_received);
            thread_spawned_map.insert(name.clone(), false);
            internet_restored_map.insert(name, false);
        }

        // wait for refresh to have new data
        thread::sleep(Duration::from_millis(wait_time));
        networks.refresh();

        let mut load_map: BTreeMap<String, VecDeque<u64>> = BTreeMap::new();

        for network in networks.iter() {
            let name = network.0.to_string();
            if name == "lo" { continue; }
            let now_received = network.1.total_received();
            let load = now_received.saturating_sub(*last_received_map.get(&name).expect("No last value!"));
            load_map.insert(name, VecDeque::from([load]));
        }

        Self {
            load_map,
            thread_spawned_map,
            internet_restored_map,
            last_received_map
        }
    }

    pub fn update(&mut self) {
        let networks = Networks::new_with_refreshed_list();
        for network in networks.iter() {
            let name = network.0.to_string();
            let now_received = network.1.total_received();
            let last_received = self.last_received_map.insert(name.clone(), now_received).expect("No value to update!");
            let load = now_received.saturating_sub(last_received);
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

