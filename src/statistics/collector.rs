use crate::simulation::{events::EventType, packet::Packet};

pub struct EventCounter {
    simulation_time: f64,
    total_queued_packets: usize,
    total_served_packets: usize,

    event_count: usize,
    temp_ls: f64,

    temp_wq: f64,
    temp_ws: f64,
}

pub struct StatisticsSample {
    l_q: f64,
    l_x: f64,
    l_s: f64,

    w_q: f64,
    w_s: f64,
}

impl EventCounter {
    pub fn new() -> Self {
        EventCounter {
            simulation_time: 0.0,
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
        packet: Packet,
        queue_length: usize,
        active_servers: usize,
        event_type: EventType,
    ) -> StatisticsSample {
        self.event_count += 1;

        match event_type {
            EventType::Arrival => {
                self.total_queued_packets += 1;
            }
            EventType::Departure(p) => {
                self.total_served_packets += 1;
            }
        };

        StatisticsSample {
            l_q: self.compute_l_q(),
            l_x: self.compute_l_q(),
            l_s: self.compute_l_s(queue_length + active_servers),
            w_q: self.compute_w_q(&packet),
            w_s: self.compute_w_s(&packet),
        }
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
