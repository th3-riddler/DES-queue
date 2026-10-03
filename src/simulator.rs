use std::collections::BinaryHeap;
use rand_simple::Exponential;

use crate::{Event, EventType::{Arrival, Departure}, Node};

pub const SEED: u32 = 42;

pub struct Simulator<'a> {
    pub current_time: f64,
    pub event_queue: BinaryHeap<Event>,
    pub node: &'a mut Node,
    pub arrivals: Exponential,
}

impl<'a> Simulator<'a> {
    pub fn new(node: &'a mut Node, lambda: f64) -> Self {
        let mut arrivals = Exponential::new(SEED);

        arrivals.try_set_params(1.0 / lambda).expect("lambda must be a positive value");

        Simulator {
            current_time: 0.0,
            event_queue: BinaryHeap::new(),
            node,
            arrivals
        }
    }

    #[inline]
    pub fn schedule_arrival(&mut self, time: f64) {
        let event: Event = Event { time, event_type: Arrival };
        self.event_queue.push(event);
    }

    #[inline]
    pub fn schedule_departure(&mut self, time: f64) {
        let event: Event = Event { time, event_type: Departure };
        self.event_queue.push(event);
    }

    pub fn handle_arrival(&mut self) {
        let next_arrival_time: f64 = self.current_time + self.sample_interarrival_time();
        self.schedule_arrival(next_arrival_time);

        if self.node.total_servers - self.node.active_servers > 0 {
            self.node.active_servers += 1;

            let departure_time: f64 = self.current_time + self.sample_service_time();
            self.schedule_departure(departure_time);
        } else {
            self.node.buffer_size += 1;
        }
    }

    pub fn handle_departure(&mut self) {
        // Checks if the buffer has some packets. If so, processes it immediately, without letting the server go idle
        if self.node.buffer_size > 0 {
            self.node.buffer_size -= 1;

            let departure_time: f64 = self.current_time + self.sample_service_time();
            self.schedule_departure(departure_time);
        } else {
            self.node.active_servers -= 1;
        }
    }

    pub fn sample_interarrival_time(&mut self) -> f64 {
        self.arrivals.sample()
    }

    pub fn sample_service_time(&mut self) -> f64 {
        todo!()
    }

    pub fn run(&mut self, max_time: f64) {
        // Schedules the first arrival
        let first_arrival_time: f64 = self.sample_interarrival_time();

        if first_arrival_time <= max_time {
            self.schedule_arrival(first_arrival_time);
        }

        while let Some(event) = self.event_queue.pop() {
            if self.current_time > max_time {
                break;
            }

            self.current_time += event.time;

            match event.event_type {
                Arrival => self.handle_arrival(),
                Departure => self.handle_departure()
            }
        }
    }
}