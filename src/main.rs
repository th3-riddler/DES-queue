use core::time;
use std::thread::sleep;

use rand_simple::Exponential;

mod simulator;

pub struct Node {
    pub buffer_size: usize,
    pub active_servers: usize,
    pub total_servers: usize,
}

impl Node {
    pub fn new(c: usize) -> Self {
        Node {
            buffer_size: 0,
            active_servers: 0,
            total_servers: c
        }
    }
}

pub enum EventType {
    Arrival,
    Departure
}

pub struct Event {
    pub time: f64,
    pub event_type: EventType
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.time.partial_cmp(&self.time).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for Event {}

const LAMBDA: f64 = 5.0;

fn main() {
    let mut e = Exponential::new(42);

    e.try_set_params(1.0 / LAMBDA).expect("d");
    loop {
        println!("x: {}", e.sample());
        sleep(time::Duration::from_secs(1));
    }
}
