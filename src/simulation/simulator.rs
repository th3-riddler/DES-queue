use rand_simple::Exponential;
use std::{
    collections::BinaryHeap,
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

use crate::{
    simulation::{
        events::{Event, EventType},
        node::{ArrivalResult, DepartureResult, Node},
        packet::Packet,
    },
    statistics::collector::{EventCounter, StatisticsSample},
};

pub const SEED: u32 = 42;

pub struct Simulator {
    current_time: f64,
    event_queue: BinaryHeap<Event>,
    node: Node,
    counter: EventCounter,

    interarrival: Exponential,
    service: Exponential,
}

impl Simulator {
    pub fn new(node: Node, lambda: f64, mu: f64) -> Result<Self, String> {
        if lambda <= 0.0 {
            return Err("lambda must be a positive value".to_string());
        }
        if mu <= 0.0 {
            return Err("mu must be a positive value".to_string());
        }
        if lambda / mu > 1.0 {
            return Err("rho must be positive and less than 1.0".to_string());
        }

        let mut interarrival: Exponential = Exponential::new(SEED);
        interarrival
            .try_set_params(1.0 / lambda)
            .expect("lambda must be a positive value");

        let mut service: Exponential = Exponential::new(SEED);
        service
            .try_set_params(1.0 / mu)
            .expect("mu must be a positive value");

        Ok(Simulator {
            current_time: 0.0,
            event_queue: BinaryHeap::new(),
            node,
            counter: EventCounter::new(),
            interarrival,
            service,
        })
    }

    pub fn _current_time(&self) -> f64 {
        self.current_time
    }
    pub fn _event_queue(&self) -> &BinaryHeap<Event> {
        &self.event_queue
    }
    pub fn _node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn schedule_arrival(&mut self, arrival_time: f64) {
        let event: Event = Event {
            time: arrival_time,
            event_type: EventType::Arrival,
        };
        self.event_queue.push(event);
    }

    #[inline]
    fn schedule_departure(&mut self, departure_time: f64, packet: Packet) {
        let event: Event = Event {
            time: departure_time,
            event_type: EventType::Departure(packet),
        };
        self.event_queue.push(event);
    }

    /// Handles the arrival events
    fn handle_arrival(&mut self, event: &Event) {
        // Generate the next arrival time, to keep the queue filling up
        let next_arrival_time: f64 = self.current_time + self.sample_interarrival_time();
        self.schedule_arrival(next_arrival_time);

        match self.node.process_arrival(event) {
            ArrivalResult::Served(mut packet) => {
                let departure_time: f64 = self.current_time + self.sample_service_time();
                packet.set_departure_time(departure_time);
                self.schedule_departure(departure_time, packet);
            }
            _ => {}
        }
    }

    /// Handles the departure events
    fn handle_departure(&mut self, _packet: &Packet) {
        match self.node.process_departure() {
            // If the server continued processing after the departure, computes the departure time of the packet that just entered
            DepartureResult::ServerContinuedProcessing(mut next_packet) => {
                let departure_time: f64 = self.current_time + self.sample_service_time();
                next_packet.set_departure_time(departure_time);
                self.schedule_departure(departure_time, next_packet);
            }
            _ => {}
        }
    }

    #[inline]
    fn sample_interarrival_time(&mut self) -> f64 {
        self.interarrival.sample()
    }

    #[inline]
    fn sample_service_time(&mut self) -> f64 {
        self.service.sample()
    }

    pub fn step(&mut self) -> Option<StatisticsSample> {
        let event = self.event_queue.pop()?;

        let dt = event.time - self.current_time;
        self.current_time = event.time;

        let statistics: StatisticsSample = self.counter.update(
            dt,
            self.node._buffer().len(),
            self.node._active_servers(),
            &event.event_type,
        );

        match event.event_type {
            EventType::Arrival => self.handle_arrival(&event),
            EventType::Departure(ref packet) => self.handle_departure(packet),
        }

        Some(statistics)
    }
}

pub fn start_simulation(lambda: f64, mu: f64, c: usize) -> Receiver<StatisticsSample> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let node = Node::new(c);
        let mut simulator = Simulator::new(node, lambda, mu).expect("Failed to create simulator");

        // Schedules the first arrival
        let first_arrival_time: f64 = simulator.sample_interarrival_time();
        simulator.schedule_arrival(first_arrival_time);

        while let Some(sample) = simulator.step() {
            if sender.send(sample).is_err() {
                break;
            }

            // Slows the visualization
            thread::sleep(Duration::from_millis(10));
        }
    });

    receiver
}
