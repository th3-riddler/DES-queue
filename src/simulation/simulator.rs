use std::{collections::BinaryHeap};
use rand_simple::Exponential;

use crate::simulation::{events::{Event, EventType::*}, node::{ArrivalResult, DepartureResult, Node}};

pub const SEED: u32 = 42;

pub struct Simulator<'a> {
    current_time: f64,
    event_queue: BinaryHeap<Event>,
    node: &'a mut Node,
    exp: Exponential,
}

impl<'a> Simulator<'a> {
    pub fn new(node: &'a mut Node, lambda: f64) -> Self {
        let mut exp: Exponential = Exponential::new(SEED);

        exp.try_set_params(1.0 / lambda).expect("lambda must be a positive value");

        Simulator {
            current_time: 0.0,
            event_queue: BinaryHeap::new(),
            node,
            exp
        }
    }

    pub fn current_time(&self) -> f64 { self.current_time }
    pub fn event_queue(&self) -> &BinaryHeap<Event> { &self.event_queue }
    pub fn node(&self) -> &Node { self.node }

    #[inline]
    fn schedule_arrival(&mut self, time: f64) {
        let event: Event = Event { time, event_type: Arrival };
        self.event_queue.push(event);
    }

    #[inline]
    fn schedule_departure(&mut self, time: f64) {
        let event: Event = Event { time, event_type: Departure };
        self.event_queue.push(event);
    }

    /// Handles the arrival events
    fn handle_arrival(&mut self) {
        // Generate the next arrival time, to keep the queue filling up
        let next_arrival_time: f64 = self.current_time + self.sample_exp_time();
        self.schedule_arrival(next_arrival_time);

        match self.node.process_arrival() {
            ArrivalResult::Served => {
                let departure_time: f64 = self.current_time + self.sample_exp_time();
                self.schedule_departure(departure_time);
            },
            _ => {},
        }
    }

    /// Handles the departure events
    fn handle_departure(&mut self) {
        match self.node.process_departure() {
            DepartureResult::ServerContinuedProcessing => {
                let departure_time: f64 = self.current_time + self.sample_exp_time();
                self.schedule_departure(departure_time);
            },
            _ => {},
        }
    }

    #[inline]
    fn sample_exp_time(&mut self) -> f64 {
        self.exp.sample()
    }

    pub fn run(&mut self, max_time: f64) {
        // Schedules the first arrival
        let first_arrival_time: f64 = self.sample_exp_time();

        if first_arrival_time <= max_time {
            self.schedule_arrival(first_arrival_time);
        }

        while let Some(event) = self.event_queue.pop() {
            if event.time > max_time {
                break;
            }

            let elapsed: f64 = event.time - self.current_time;

            self.current_time = event.time;

            match event.event_type {
                Arrival => self.handle_arrival(),
                Departure => self.handle_departure()
            }
        }
    }
}