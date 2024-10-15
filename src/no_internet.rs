
use std::{collections::VecDeque, sync::{atomic::{AtomicBool, Ordering}, Arc}};

use nabu::*;
use sysinfo::Networks;

const SHORT_INTERVAL: u64 = 100;

pub fn no_internet(term_now: Arc<AtomicBool>, internet_restored: Arc<AtomicBool>) {
    let mut last_interval_incoming: VecDeque<usize> = VecDeque::new();
    // This should shut the function down nicely instead of hoping for a timely loop finish
    while !term_now.load(Ordering::Relaxed) {
        let date_time = {
        };
        
    }
}
