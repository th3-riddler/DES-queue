#[cfg(test)]
mod tests {
    use crate::{Node, simulation::Simulator};

    #[test]
    fn simulation_stays_within_max_time() {
        let mut node = Node::new(1);
        let mut simulator = Simulator::new(&mut node, 5.0);

        simulator.run(10.0);

        println!("{}", simulator.current_time());
        assert!(simulator.current_time() <= 9.0);
    }

    #[test]
    fn simulation_preserves_server_invariant() {
        let mut node = Node::new(2);
        let mut simulator = Simulator::new(&mut node, 5.0);

        simulator.run(10.0);

        assert!(simulator.node().active_servers <= simulator.node().total_servers);
        assert_eq!(simulator.node().buffer_size, 0);
    }

    #[test]
    fn same_seed_is_reproducible() {
        let mut node1 = Node::new(1);
        let mut node2 = Node::new(1);

        let mut simulator1 = Simulator::new(&mut node1, 5.0);
        let mut simulator2 = Simulator::new(&mut node2, 5.0);

        simulator1.run(10.0);
        simulator2.run(10.0);

        assert_eq!(simulator1.current_time(), simulator2.current_time());
        assert_eq!(simulator1.node().active_servers, simulator2.node().active_servers);
        assert_eq!(simulator1.node().buffer_size, simulator2.node().buffer_size);
    }
}