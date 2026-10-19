use crate::simulation::{events::EventType, packet::Packet};

const DEFAULT_VEC_SIZE: usize = 24;

pub struct EventCounter {
    number_in_system: [f64; DEFAULT_VEC_SIZE],
    total_event_time: f64,
    pub total_arrivals: usize,
    pub total_departures: usize,

    pub total_queued_packets: usize,
    pub total_served_packets: usize,

    queue_length_time: f64,
    serving_packets_time: f64,
    packets_in_system_time: f64,

    in_queue_time: f64,
    in_system_time: f64,
}

pub struct StatisticsSample {
    pub p_k: [f64; DEFAULT_VEC_SIZE],
    pub l_q: f64,
    pub l_x: f64,
    pub l_s: f64,
    pub w_q: f64,
    pub w_s: f64,
    pub p_queue: f64,

    pub total_arrivals: usize,
    pub total_departures: usize,
}

impl EventCounter {
    pub fn new() -> Self {
        EventCounter {
            number_in_system: [0.0; DEFAULT_VEC_SIZE],
            total_event_time: 0.0,
            total_arrivals: 0,
            total_departures: 0,

            total_queued_packets: 0,
            total_served_packets: 0,
            
            queue_length_time: 0.0,
            serving_packets_time: 0.0,
            packets_in_system_time: 0.0,

            in_queue_time: 0.0,
            in_system_time: 0.0,
        }
    }

    pub fn update(
        &mut self,
        dt: f64,
        queue_length: usize,
        active_servers: usize,
        event_type: &EventType,
    ) -> StatisticsSample {
        self.total_event_time += dt;

        let (w_q, w_s) = match event_type {
            EventType::Arrival => (self.in_queue_time / self.total_served_packets as f64, self.in_system_time / self.total_served_packets as f64),
            EventType::Departure(packet) => (self.compute_w_q(packet), self.compute_w_s(packet))
        };
        let p_k: [f64; DEFAULT_VEC_SIZE] = self.compute_p_k(dt, queue_length + active_servers);

        StatisticsSample {
            p_k,
            l_q: self.compute_l_q(queue_length, self.total_event_time, dt),
            l_x: self.compute_l_x(active_servers, self.total_event_time, dt),
            l_s: self.compute_l_s(queue_length + active_servers, self.total_event_time, dt),
            w_q,
            w_s,
            p_queue: self.compute_p_queue(),

            total_arrivals: self.total_arrivals,
            total_departures: self.total_departures
        }
    }

    #[inline]
    fn compute_p_queue(&self) -> f64 {
        self.total_queued_packets as f64 / self.total_arrivals as f64
    }

    #[inline]
    fn compute_p_k(&mut self, dt: f64, total_packets: usize) -> [f64; DEFAULT_VEC_SIZE] {
        if total_packets < DEFAULT_VEC_SIZE {
            self.number_in_system[total_packets] += dt;
        }

        self.number_in_system.map(|time| time / self.total_event_time)
    }

    #[inline]
    fn compute_l_q(&mut self, queue_length: usize, total_event_time: f64, dt: f64) -> f64 {
        self.queue_length_time += queue_length as f64 * dt;

        self.queue_length_time / total_event_time
    }

    #[inline]
    fn compute_l_x(&mut self, serving_packets: usize, total_event_time: f64, dt: f64) -> f64 {
        self.serving_packets_time += serving_packets as f64 * dt;
        
        self.serving_packets_time / total_event_time
    }

    #[inline]
    fn compute_l_s(&mut self, packets_in_system: usize, total_event_time: f64, dt: f64) -> f64 {
        self.packets_in_system_time += packets_in_system as f64 * dt;

        self.packets_in_system_time / total_event_time
    }

    #[inline]
    fn compute_w_q(&mut self, packet: &Packet) -> f64 {
        let wait: f64 = packet.get_start_service_time().unwrap_or(0.0) - packet.get_arrival_time().unwrap_or(0.0);
        self.in_queue_time += wait;

        self.in_queue_time / self.total_served_packets as f64
    }

    #[inline]
    fn compute_w_s(&mut self, packet: &Packet) -> f64 {
        let wait: f64 = packet.get_departure_time().unwrap_or(0.0) - packet.get_arrival_time().unwrap_or(0.0);
        self.in_system_time += wait;

        self.in_system_time / self.total_served_packets as f64
    }
}
