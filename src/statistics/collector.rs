pub struct EventCounter {
    queued_packets: usize,
    immediately_served_packets: usize,
    blocked_packets: usize,
    completed_packets: usize,
}
