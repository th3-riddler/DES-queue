pub enum ArrivalResult {
    Served,
    Queued,
    Blocked,
}

pub enum DepartureResult {
    ServerContinuedProcessing,
    ServerBecameIdle,
}

pub struct Node {
    buffer_size: usize,
    active_servers: usize,
    total_servers: usize,
}

impl Node {
    pub fn new(c: usize) -> Self {
        Node {
            buffer_size: 0,
            active_servers: 0,
            total_servers: c,
        }
    }

    #[inline]
    fn is_a_server_available(&self) -> bool {
        self.total_servers - self.active_servers > 0
    }

    #[inline]
    fn is_buffer_empty(&self) -> bool {
        self.buffer_size == 0
    }

    #[inline]
    fn total_packets_number(&self) -> usize {
        self.active_servers + self.buffer_size
    }

    /// Processes arrival events.
    /// 
    /// Checks if there's at least one server available. If so, serves the packet immediately, 
    /// otherwise queues it if the buffer size allows it.
    pub fn process_arrival(&mut self) -> ArrivalResult {
        if self.is_a_server_available() {
            self.active_servers += 1;
            
            ArrivalResult::Served
        } else {
            self.buffer_size += 1;

            ArrivalResult::Queued
        }
    }

    /// Processes departure events.
    /// 
    /// Checks if the buffer has some packets. If so, processes it immediately, without letting the server go idle
    pub fn process_departure(&mut self) -> DepartureResult {
        if !self.is_buffer_empty() {
            self.buffer_size -= 1;

            DepartureResult::ServerContinuedProcessing
        } else {
            self.active_servers -= 1;
            
            DepartureResult::ServerBecameIdle
        }
    }
}