use std::sync::mpsc::Receiver;

use eframe::egui;
use egui_plot::{Bar, BarChart, Line, Plot};

use crate::{
    C, LAMBDA, MU, RHO,
    simulation::simulator::{FAST_FORWARD_EVENTS, SimulationCommand},
    statistics::collector::StatisticsSample,
    utils::{erlang_c, l_q, l_s, l_x, p_k, w_q, w_s},
};

use crate::utils::DEFAULT_VEC_SIZE;

const ZERO_TOLERANCE: f64 = 1e-6;

fn relative_error(simulated_value: f64, theory_value: f64) -> Option<f64> {
    if theory_value.abs() <= ZERO_TOLERANCE {
        return (simulated_value.abs() <= ZERO_TOLERANCE).then_some(0.0);
    }

    Some((simulated_value - theory_value).abs() / theory_value.abs() * 100.0)
}

pub struct DisplayPlots {
    stats: Option<StatisticsSample>,
    theoretical_values: [f64; 6],
    theoretical_p_k: [f64; DEFAULT_VEC_SIZE],
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
            theoretical_values: [
                l_q(LAMBDA, MU, C),
                l_x(LAMBDA, MU, C),
                l_s(LAMBDA, MU, C),
                w_q(LAMBDA, MU, C),
                w_s(LAMBDA, MU, C),
                erlang_c(C, C as f64 * RHO),
            ],
            theoretical_p_k: p_k(LAMBDA, MU, RHO, C),
            command_sender,
        }
    }
}

impl eframe::App for DisplayPlots {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut has_new_data: bool = false;

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
                    && self
                        .command_sender
                        .send(SimulationCommand::FastForward(FAST_FORWARD_EVENTS))
                        .is_err()
                {
                    eprintln!("Unable to fast-forward: simulation has stopped");
                }
            });
            ui.separator();

            if let Some(stats) = &self.stats {
                egui::Frame::group(ui.style())
                    .fill(ui.visuals().faint_bg_color)
                    .inner_margin(egui::Margin::same(12))
                    .show(ui, |ui| {
                        egui::Grid::new("statistics_comparison")
                            .num_columns(4)
                            .striped(true)
                            .min_col_width(150.0)
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new("Statistic"));
                                ui.label(
                                    egui::RichText::new("Simulated")
                                        .strong()
                                        .color(ui.visuals().hyperlink_color),
                                );

                                ui.label(egui::RichText::new("Theory").strong());
                                ui.label(egui::RichText::new("Error").strong());
                                ui.end_row();
                                for ((name, simulated_value), theory_value) in [
                                    ("L_q", stats.l_q),
                                    ("L_x", stats.l_x),
                                    ("L_s", stats.l_s),
                                    ("W_q", stats.w_q),
                                    ("W_s", stats.w_s),
                                    ("P_queue", stats.p_queue),
                                ]
                                .into_iter()
                                .zip(self.theoretical_values.iter())
                                {
                                    ui.label(egui::RichText::new(name).strong());
                                    ui.label(format!("{simulated_value:.4}"));
                                    ui.label(
                                        egui::RichText::new(format!("{theory_value:.4}")).italics(),
                                    );

                                    let error: String =
                                        relative_error(simulated_value, *theory_value).map_or_else(
                                            || "N/A".to_string(),
                                            |value| format!("{value:.2}%"),
                                        );
                                    ui.label(egui::RichText::new(error).italics());
                                    ui.end_row();
                                }
                            });

                        ui.add_space(8.0);
                        ui.separator();
                        ui.label(egui::RichText::new("Event totals").strong());
                        egui::Grid::new("event_totals")
                            .num_columns(2)
                            .spacing([32.0, 4.0])
                            .show(ui, |ui| {
                                ui.label("Total arrivals");
                                ui.label(
                                    egui::RichText::new(stats.total_arrivals.to_string())
                                        .strong()
                                        .color(ui.visuals().hyperlink_color),
                                );
                                ui.end_row();

                                ui.label("Total departures");
                                ui.label(
                                    egui::RichText::new(stats.total_departures.to_string())
                                        .strong()
                                        .color(ui.visuals().hyperlink_color),
                                );
                                ui.end_row();
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

                let bar_width: f64 = 0.7;
                let half_w: f64 = bar_width / 2.0;

                Plot::new("p_k_plot")
                    .height(ui.available_height().max(250.0))
                    .default_y_bounds(0.0, 0.05)
                    .include_y(0.0)
                    .include_x(0.0)
                    .show(ui, |plot_ui| {
                        plot_ui.bar_chart(chart);

                        for (k, &p_theo) in self.theoretical_p_k.iter().enumerate() {
                            let k_f = k as f64;
                            let seg = Line::new(
                                format!("theo_bar_{k}"),
                                vec![[k_f - half_w, p_theo], [k_f + half_w, p_theo]],
                            )
                            .color(egui::Color32::GREEN)
                            .width(2.5);

                            plot_ui.line(seg);
                        }
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
