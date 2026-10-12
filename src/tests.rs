#[cfg(test)]
mod tests {
    use crate::{Node, simulation::simulator::Simulator};

    #[test]
    fn simulation_stays_within_max_time() {
        let node = Node::new(1);
        let mut simulator = Simulator::new(node, 8.0, 9.0).expect("Failed to create simulator");

        simulator.run(10.0);

        println!("{}", simulator._current_time());
        assert!(simulator._current_time() <= 10.0);
    }

    #[test]
    fn simulation_preserves_server_invariant() {
        let node = Node::new(2);
        let mut simulator = Simulator::new(node, 8.0, 9.0).expect("Failed to create simulator");

        simulator.run(10.0);

        assert!(simulator._node()._active_servers() <= simulator._node()._total_servers());
        assert_eq!(simulator._node()._buffer().len(), 0);
    }

    #[test]
    fn same_seed_is_reproducible() {
        let node1 = Node::new(1);
        let node2 = Node::new(1);

        let mut simulator1 = Simulator::new(node1, 8.0, 9.0).expect("Failed to create simulator");
        let mut simulator2 = Simulator::new(node2, 8.0, 9.0).expect("Failed to create simulator");

        simulator1.run(10.0);
        simulator2.run(10.0);

        assert_eq!(simulator1._current_time(), simulator2._current_time());
        assert_eq!(simulator1._node()._active_servers(), simulator2._node()._active_servers());
        assert_eq!(simulator1._node()._buffer().len(), simulator2._node()._buffer().len());
    }
}