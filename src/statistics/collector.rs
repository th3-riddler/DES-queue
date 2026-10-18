use crate::simulation::{events::EventType, packet::{self, Packet}};

const DEFAULT_VEC_SIZE: usize = 24;

pub struct EventCounter {
    number_in_system: [f64; DEFAULT_VEC_SIZE],
    total_event_time: f64,
    pub total_packets: usize,

    pub total_queued_packets: usize,
    pub total_served_packets: usize,

    event_count: usize,
    temp_ls: f64,
    temp_lq: f64,

    temp_wq: f64,
    temp_ws: f64,
}

pub struct StatisticsSample {
    pub p_k: [f64; DEFAULT_VEC_SIZE],
    pub l_q: f64,
    pub l_x: f64,
    pub l_s: f64,
    pub w_q: f64,
    pub w_s: f64,
}

impl EventCounter {
    pub fn new() -> Self {
        EventCounter {
            number_in_system: [0.0; DEFAULT_VEC_SIZE],
            total_event_time: 0.0,
            total_packets: 0,
            total_queued_packets: 0,
            total_served_packets: 0,
            event_count: 0,
            temp_ls: 0.0,
            temp_lq: 0.0,
            temp_wq: 0.0,
            temp_ws: 0.0,
        }
    }

    pub fn update(
        &mut self,
        dt: f64,
        queue_length: usize,
        active_servers: usize,
        event_type: &EventType,
    ) -> StatisticsSample {
        self.event_count += 1;
        self.total_event_time += dt;

        let (w_q, w_s) = match event_type {
            EventType::Arrival => (self.temp_wq, self.temp_ws),
            EventType::Departure(packet) => (self.compute_w_q(&packet), self.compute_w_s(&packet))
        };
        let p_k: [f64; DEFAULT_VEC_SIZE] = self.compute_p_k(dt, queue_length + active_servers);

        StatisticsSample {
            p_k,
            l_q: self.compute_l_q(queue_length),
            l_x: self.compute_l_x(),
            l_s: self.compute_l_s(queue_length + active_servers),
            w_q,
            w_s,
        }
    }

    #[inline]
    fn compute_p_k(&mut self, dt: f64, total_packets: usize) -> [f64; DEFAULT_VEC_SIZE] {
        if total_packets < DEFAULT_VEC_SIZE {
            self.number_in_system[total_packets] += dt;
        }

        self.number_in_system
            .map(|time| time / self.total_event_time)
    }

    #[inline]
    fn compute_l_q(&mut self, packets: usize) -> f64 {
        self.temp_lq = (self.temp_lq * self.total_packets as f64 + packets as f64)
            / (self.total_packets as f64 + 1.0);

        self.temp_lq
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
        self.temp_wq = ((self.temp_wq * self.total_served_packets as f64)
            + (packet.get_start_service_time().unwrap_or(0.0)
                - packet.get_arrival_time().unwrap_or(0.0)))
            / (self.total_served_packets as f64 + 1.0);

        self.temp_wq
    }

    #[inline]
    fn compute_w_s(&mut self, packet: &Packet) -> f64 {
        self.temp_ws = ((self.temp_ws * self.total_served_packets as f64)
            + (packet.get_departure_time().unwrap_or(0.0)
                - packet.get_arrival_time().unwrap_or(0.0)))
            / (self.total_served_packets as f64 + 1.0);

        self.temp_ws
    }
}
