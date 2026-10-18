mod plots;
mod simulation;
mod statistics;
mod utils;
// mod tests;

use crate::{
    simulation::simulator::start_simulation,
};

const LAMBDA: f64 = 8.0;
const MU: f64 = 9.0;
const C: usize = 1;

fn main() {
    let (receiver, command_sender) = start_simulation(LAMBDA, MU, C);
    let _ = plots::plots::make_plots(receiver, command_sender);
}
