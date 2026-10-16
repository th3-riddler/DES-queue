use std::collections::Vec;
use crate::simulation::{events::EventType, packet::Packet};

pub struct EventCounter {
    number_in_system: Vec<f64>,
    total_queued_packets: usize,
    total_served_packets: usize,

    event_count: usize,
    temp_ls: f64,

    temp_wq: f64,
    temp_ws: f64,
}

pub struct StatisticsSample {
    p_k: f64,
    l_q: f64,
    l_x: f64,
    l_s: f64,

    w_q: Option<f64>,
    w_s: Option<f64>,
}

impl EventCounter {
    pub fn new() -> Self {
        EventCounter {
            number_in_system: Vec::with_capacity(24),
            total_queued_packets: 0,
            total_served_packets: 0,
            event_count: 0,
            temp_ls: 0.0,
            temp_wq: 0.0,
            temp_ws: 0.0,
        }
    }

    pub fn update(
        &mut self,
        event_time: usize,
        queue_length: usize,
        active_servers: usize,
        event_type: EventType,
    ) -> StatisticsSample {
        self.event_count += 1;

        let w_q: Option<f64>;
        let w_s: Option<f64>;

        match event_type {
            EventType::Arrival => {
                self.total_queued_packets += 1;
                w_q = None;
                w_s = None;
            }
            EventType::Departure(packet) => {
                self.total_served_packets += 1;
                w_q = Some(self.compute_w_q(&packet));
                w_s = Some(self.compute_w_s(&packet));
            }
        }

        StatisticsSample {
            p_k: self.compute_p_k(event_time),
            l_q: self.compute_l_q(),
            l_x: self.compute_l_x(),
            l_s: self.compute_l_s(queue_length + active_servers),
            w_q,
            w_s,
        }
    }

    #[inline]
    fn compute_p_k(&mut self, event_time: usize) -> Vec< {
        for i in 0..(self.number_in_system.length)
    }

    #[inline]
    fn compute_l_q(&self) -> f64 {
        self.total_queued_packets as f64
            / (self.total_queued_packets + self.total_served_packets) as f64
    }

    #[inline]
    fn compute_l_x(&self) -> f64 {
        self.total_served_packets as f64
            / (self.total_queued_packets + self.total_served_packets) as f64
    }

    #[inline]
    fn compute_l_s(&mut self, packets: usize) -> f64 {
        self.temp_ls = (self.temp_ls * (self.event_count - 1) as f64 + packets as f64)
            / self.event_count as f64;

        self.temp_ls
    }

    #[inline]
    fn compute_w_q(&mut self, packet: &Packet) -> f64 {
        self.temp_wq = ((self.temp_wq * self.total_queued_packets as f64)
            + (packet.get_start_service_time().unwrap_or(0.0)
                - packet.get_arrival_time().unwrap_or(0.0)))
            / self.event_count as f64;

        self.temp_wq
    }

    #[inline]
    fn compute_w_s(&mut self, packet: &Packet) -> f64 {
        self.temp_ws = ((self.temp_ws * self.event_count as f64)
            + (packet.get_departure_time().unwrap_or(0.0)
                - packet.get_arrival_time().unwrap_or(0.0)))
            / self.event_count as f64;

        self.temp_ws
    }
}
