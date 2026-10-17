use std::sync::mpsc::Receiver;

use eframe::egui;
use egui_plot::{Bar, BarChart, Plot};

use crate::statistics::collector::StatisticsSample;

pub struct DisplayPlots {
    stats: Option<StatisticsSample>,
    receiver: Receiver<StatisticsSample>,
}

impl DisplayPlots {
    pub fn new(receiver: Receiver<StatisticsSample>) -> Self {
        Self {
            receiver,
            stats: None,
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
            ui.heading("Queue Simulation Stats");
            ui.separator();

            if let Some(stats) = &self.stats {
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Current Metrics").strong());
                    ui.label(format!("L_q: {:.4}", stats.l_q));
                    ui.label(format!("L_x: {:.4}", stats.l_x));
                    ui.label(format!("L_s: {:.4}", stats.l_s));

                    match stats.w_q {
                        Some(wq) => ui.label(format!("W_q: {:.4}", wq)),
                        None => ui.label("W_q: Pending..."),
                    };

                    match stats.w_s {
                        Some(ws) => ui.label(format!("W_s: {:.4}", ws)),
                        None => ui.label("W_s: Pending..."),
                    };
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("Probability Distribution p_k").strong());

                // Bar chart for p_k
                let bars: Vec<Bar> = stats
                    .p_k
                    .iter()
                    .enumerate()
                    .map(|(k, &p)| Bar::new(k as f64, p).name(format!("k = {}", k)))
                    .collect();

                let chart = BarChart::new("p_k", bars).width(0.7);

                Plot::new("p_k_plot")
                    .height(ui.available_height())
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

pub fn make_plots(receiver: Receiver<StatisticsSample>) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_title("Simulation Stats"),
        ..Default::default()
    };

    eframe::run_native(
        "Simulation Stats",
        options,
        Box::new(|_cc| Ok(Box::new(DisplayPlots::new(receiver)))),
    )
}
