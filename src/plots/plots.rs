use std::sync::mpsc::Receiver;

use eframe::egui;
use egui_plot::{Bar, BarChart, Corner, Legend, Plot};

use crate::{
    simulation::simulator::{FAST_FORWARD_EVENTS, SimulationCommand},
    statistics::collector::StatisticsSample,
};

pub struct DisplayPlots {
    stats: Option<StatisticsSample>,
    receiver: Receiver<StatisticsSample>,
    command_sender: std::sync::mpsc::Sender<SimulationCommand>,
}

impl DisplayPlots {
    pub fn new(
        receiver: Receiver<StatisticsSample>,
        command_sender: std::sync::mpsc::Sender<SimulationCommand>,
    ) -> Self {
        Self {
            receiver,
            stats: None,
            command_sender,
        }
    }
}

impl eframe::App for DisplayPlots {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut has_new_data = false;

        // Save the last StatisticsSample update
        for new_stats in self.receiver.try_iter() {
            self.stats = Some(new_stats);
            has_new_data = true;
        }

        if has_new_data {
            ui.ctx().request_repaint();
        }

        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Queue Simulation Stats");
                ui.add_space(ui.available_width() - 190.0);
                if ui
                    .add_sized(
                        [180.0, 32.0],
                        egui::Button::new(egui::RichText::new("Fast-forward 200k events").strong()),
                    )
                    .clicked()
                {
                    if self
                        .command_sender
                        .send(SimulationCommand::FastForward(FAST_FORWARD_EVENTS))
                        .is_err()
                    {
                        eprintln!("Unable to fast-forward: simulation has stopped");
                    }
                }
            });
            ui.separator();

            if let Some(stats) = &self.stats {
                egui::Frame::group(ui.style())
                    .fill(ui.visuals().faint_bg_color)
                    .inner_margin(egui::Margin::same(12))
                    .show(ui, |ui| {
                        egui::Grid::new("statistics_comparison")
                            .num_columns(3)
                            .striped(true)
                            .min_col_width(150.0)
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new("Statistic"));
                                ui.label(
                                    egui::RichText::new("Simulated")
                                        .strong()
                                        .color(ui.visuals().hyperlink_color),
                                );
                                ui.label(egui::RichText::new("Theory").strong().weak());
                                ui.end_row();

                                for (name, value) in [
                                    ("L_q", stats.l_q),
                                    ("L_x", stats.l_x),
                                    ("L_s", stats.l_s),
                                    ("W_q", stats.w_q),
                                    ("W_s", stats.w_s),
                                ] {
                                    ui.label(egui::RichText::new(name).strong());
                                    ui.label(format!("{value:.4}"));
                                    ui.label(egui::RichText::new("-").italics());
                                    ui.end_row();
                                }
                            });
                    });

                ui.add_space(6.0);
                ui.label(egui::RichText::new("Probability Distribution P_k").strong());

                let bars: Vec<Bar> = stats
                    .p_k
                    .iter()
                    .enumerate()
                    .map(|(k, &p)| Bar::new(k as f64, p))
                    .collect();

                let chart = BarChart::new("Probability distribution p_k", bars).width(0.7);

                Plot::new("p_k_plot")
                    .height(ui.available_height().max(250.0))
                    .include_y(0.0)
                    .include_x(0.0)
                    .show(ui, |plot_ui| {
                        plot_ui.bar_chart(chart);
                    });
            } else {
                ui.label("Waiting for simulation data...");
            }
        });
    }
}

pub fn make_plots(
    receiver: Receiver<StatisticsSample>,
    command_sender: std::sync::mpsc::Sender<SimulationCommand>,
) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_title("Simulation Stats"),
        ..Default::default()
    };

    eframe::run_native(
        "Simulation Stats",
        options,
        Box::new(|_cc| Ok(Box::new(DisplayPlots::new(receiver, command_sender)))),
    )
}
