mod simulation;

use crate::simulation::{node::Node, simulator::Simulator};

fn main() {
    let mut node = Node::new(1);
    let mut simulator = Simulator::new(&mut node, 5.0);

    simulator.run(10.0);
}
