pub struct EventCounter {
    simulation_time: f64,
    total_queued_packets: usize,
    total_served_packets: usize,

    event_count: usize,
    temp_ls: f64
}

pub struct StatisticsSample {
    l_s: f64,
    l_x: f64,
    l_q: f64,

    w_s: f64,
    w_q: f64,

}

impl EventCounter {
    pub fn new() -> Self {
        EventCounter {
            simulation_time: 0.0,
            total_queued_packets: 0,
            total_served_packets: 0,
            event_count: 0,
            temp_ls: 0.0
        }
    }

    // pub fn update(&mut self, queue_length: i32, active_servers: i32) -> 

    fn compute_l_q(&self) -> f64 {
        todo!()
    }
}