use std::collections::{BTreeMap, VecDeque};

use sysinfo::Networks;

pub struct NetworkTrafficHandler {
    pub load_map: BTreeMap<String, VecDeque<u64>>,
    last_received_map: BTreeMap<String, u64>,
}

impl NetworkTrafficHandler {
    pub fn new() -> Self {
        let networks = Networks::new_with_refreshed_list();

        let mut last_received_map = BTreeMap::new();

        for network in networks.iter() {
            let name = network.0.to_string();
            let last_received = network.1.total_received();
            last_received_map.insert(name, last_received);
        }

        Self {
            load_map: BTreeMap::new(),
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
            if self.load_map.len() == 60 {
                if let Some(load_vec) = self.load_map.get_mut(&name) {
                    load_vec.pop_front();
                    load_vec.push_back(load);
                } else {
                    panic!("No value to update!");
                }
            } else {
                if let Some(load_vec) = self.load_map.get_mut(&name) {
                    load_vec.push_back(load);
                } else {
                    panic!("No value to update!");
                }
            }
        }
    }
}

