mod simulation;
mod statistics;
// mod tests;

use crate::simulation::{node::Node, simulator::Simulator};

const LAMBDA: f64 = 8.0;
const MU: f64 = 9.0;

fn main() {
    let mut node = Node::new(1);
    let mut simulator = Simulator::new(&mut node, LAMBDA, MU).expect("Failed to create simulator");

    simulator.run(10.0);
}
