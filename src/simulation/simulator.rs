use std::{
    collections::BinaryHeap,
    sync::mpsc::{self, Receiver, Sender, TryRecvError},
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

pub const FAST_FORWARD_EVENTS: usize = 200_000;

pub enum SimulationCommand {
    FastForward(usize),
}

pub struct Simulator {
    current_time: f64,
    event_queue: BinaryHeap<Event>,
    node: Node,
    counter: EventCounter,
}

impl Simulator {
    pub fn new(node: Node) -> Self {
        Simulator {
            current_time: 0.0,
            event_queue: BinaryHeap::new(),
            node,
            counter: EventCounter::new(),
        }
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
        let next_arrival_time: f64 = self.current_time + self.node.sample_interarrival_time();
        self.schedule_arrival(next_arrival_time);

        self.counter.total_arrivals += 1;
        match self.node.process_arrival(event) {
            ArrivalResult::Served(mut packet) => {
                let departure_time: f64 = self.current_time + self.node.sample_service_time();
                packet.set_departure_time(departure_time);
                self.schedule_departure(departure_time, packet);
            }
            ArrivalResult::Queued => {
                self.counter.total_queued_packets += 1;
            }
        }
    }

    /// Handles the departure events
    fn handle_departure(&mut self, event: &Event) {
        self.counter.total_departures += 1;

        // If the server continued processing after the departure, computes the departure time of the packet that just entered
        if let DepartureResult::ServerContinuedProcessing(mut next_packet) = self.node.process_departure() {
            next_packet.set_start_service_time(event.time);

            let departure_time: f64 = self.current_time + self.node.sample_service_time();
            next_packet.set_departure_time(departure_time);
            self.schedule_departure(departure_time, next_packet);
        }
    }

    pub fn step(&mut self) -> Option<StatisticsSample> {
        let event: Event = self.event_queue.pop()?;

        let dt: f64 = event.time - self.current_time;
        self.current_time = event.time;
        
        let statistics: StatisticsSample = self.counter.update(
            dt,
            self.node._buffer().len(),
            self.node._active_servers(),
            &event.event_type,
        );

        match event.event_type {
            EventType::Arrival => {
                let interarrival_time: f64 = event.time - self.counter.previous_arrival_time;
                self.counter.insert_interarrival_time(interarrival_time);
                self.counter.previous_arrival_time = event.time;

                self.handle_arrival(&event)
            },
            EventType::Departure(ref packet) => {
                let service_time: f64 = packet.get_departure_time().unwrap_or(0.0) - packet.get_start_service_time().unwrap_or(0.0);
                self.counter.insert_service_time(service_time);

                self.handle_departure(&event);
            },
        }

        Some(statistics)
    }
}

pub fn start_simulation(
    lambda: f64,
    mu: f64,
    c: usize,
) -> (Receiver<StatisticsSample>, Sender<SimulationCommand>) {
    let (sender, receiver) = mpsc::channel();
    let (command_sender, command_receiver) = mpsc::channel();

    thread::spawn(move || {
        let node: Node = Node::new(lambda, mu, c).expect("Failed to create Node");
        let mut simulator: Simulator = Simulator::new(node);
        let mut fast_forward_events = 0;

        // Schedules the first arrival
        let first_arrival_time: f64 = simulator.node.sample_interarrival_time();
        simulator.schedule_arrival(first_arrival_time);

        while let Some(sample) = simulator.step() {
            if fast_forward_events > 0 {
                fast_forward_events -= 1;
                if fast_forward_events == 0 && sender.send(sample).is_err() {
                    break;
                }
                continue;
            }

            match command_receiver.try_recv() {
                Ok(SimulationCommand::FastForward(events)) => {
                    fast_forward_events = events;
                }
                Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => {}
            }

            if sender.send(sample).is_err() {
                break;
            }

            // Slows the visualization
            thread::sleep(Duration::from_millis(10));
        }
    });

    (receiver, command_sender)
}
