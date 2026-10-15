use std::collections::VecDeque;

use crate::simulation::{events::Event, packet::Packet};

pub enum ArrivalResult {
    Served(Packet),
    Queued,
}

pub enum DepartureResult {
    ServerContinuedProcessing(Packet),
    ServerBecameIdle,
}

pub struct Node {
    buffer: VecDeque<Packet>,
    active_servers: usize,
    total_servers: usize,
}

impl Node {
    pub fn new(c: usize) -> Self {
        Node {
            buffer: VecDeque::new(),
            active_servers: 0,
            total_servers: c,
        }
    }

    pub fn _buffer(&self) -> &VecDeque<Packet> { &self.buffer }
    pub fn _active_servers(&self) -> usize { self.active_servers }
    pub fn _total_servers(&self) -> usize { self.total_servers }

    #[inline]
    fn is_a_server_available(&self) -> bool {
        self.total_servers - self.active_servers > 0
    }

    #[inline]
    fn is_buffer_empty(&self) -> bool {
        self.buffer.len() == 0
    }

    #[inline]
    fn total_packets_number(&self) -> usize {
        self.active_servers + self.buffer.len()
    }

    /// Processes arrival events.
    /// 
    /// Checks if there's at least one server available. If so, serves the packet immediately, 
    /// otherwise queues it if the buffer size allows it.
    pub fn process_arrival(&mut self, event: &Event) -> ArrivalResult {
        let mut packet: Packet = Packet::new(event.time);

        if self.is_a_server_available() {
            self.active_servers += 1;
            packet.set_start_service_time(event.time);
            ArrivalResult::Served(packet)
        } else {
            self.buffer.push_back(packet);
            ArrivalResult::Queued
        }
    }

    /// Processes departure events.
    /// 
    /// Checks if the buffer has some packets. If so, processes it immediately, without letting the server go idle
    pub fn process_departure(&mut self) -> DepartureResult {
        if let Some(packet) = self.buffer.pop_front() {
            DepartureResult::ServerContinuedProcessing(packet)
        } else {
            self.active_servers -= 1;
            DepartureResult::ServerBecameIdle
        }
    }
}