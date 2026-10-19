mod plots;
mod simulation;
mod statistics;
mod utils;
// mod tests;

use crate::{
    simulation::simulator::start_simulation,
};

const LAMBDA: f64 = -2.0;
const MU: f64 = -2.0;
const C: usize = 2;

fn main() {
    let (receiver, command_sender) = start_simulation(LAMBDA, MU, C);
    let _ = plots::plotter::make_plots(receiver, command_sender);
}
