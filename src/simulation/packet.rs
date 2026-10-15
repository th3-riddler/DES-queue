pub struct Packet {
    arrival_time: Option<f64>,
    start_service_time: Option<f64>,
    departure_time: Option<f64>,
}

impl Packet {
    pub fn new(arrival_time: f64) -> Self {
        Packet {
            arrival_time: Some(arrival_time),
            start_service_time: None,
            departure_time: None
        }
    }

    pub fn set_arrival_time(&mut self, arrival_time: f64) {
        self.arrival_time = Some(arrival_time);
    }

    pub fn set_start_service_time(&mut self, start_service_time: f64) {
        self.start_service_time = Some(start_service_time);
    }

    pub fn set_departure_time(&mut self, departure_time: f64) {
        self.departure_time = Some(departure_time);
    }
}