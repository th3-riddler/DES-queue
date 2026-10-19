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
const RHO: f64 = LAMBDA / (C as f64 * MU);

fn main() -> Result<(), String> {
    if LAMBDA <= 0.0 {
        return Err("LAMBDA must be a positive value".to_string());
    }
    if MU <= 0.0 {
        return Err("MU must be a positive value".to_string());
    }
    if C == 0 {
        return Err("C must be a positive value".to_string());
    }
    if RHO >= 1.0 {
        return Err(format!("rho: ({:.4}) must be positive and less than 1.0", RHO).to_string());
    }

    let (receiver, command_sender) = start_simulation(LAMBDA, MU, C);
    let _ = plots::plotter::make_plots(receiver, command_sender);

    Ok(())
}
