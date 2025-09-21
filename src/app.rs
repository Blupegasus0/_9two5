use eframe::egui;

use crate::store::*;

pub struct App {
    store: Store,
}

impl Default for App {
    fn default() -> Self {
        Self {
            store: Store::load(),
        }
    }
}

impl eframe::App for App {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.add(egui::widgets::Button::new("Start")).clicked() {
                    self.store.start();
                }
                if ui.add(egui::widgets::Button::new("Stop")).clicked() {
                    self.store.stop();
                }
                if ui.add(egui::widgets::Button::new("Reset Today")).clicked() {
                    self.store.reset_today();
                }
            });

            ui.separator();

            let secs = self.store.total_today_seconds();
            let hours = secs / 3600;
            let minutes = (secs % 3600) / 60;
            let seconds = secs % 60;
            ui.label(format!("Today: {:02}h:{:02}m:{:02}s", hours, minutes, seconds));

            ui.separator();
            ui.heading("Last 7 days");
            for (day, s) in self.store.totals_for_last_7_days() {
                let h = s / 3600;
                let m = (s % 3600) / 60;
                ui.label(format!("{} — {:02}h:{:02}m", day, h, m));
            }

            ui.separator();
            ui.label("Note: data is saved locally in your platform data directory.");
        });

        // small refresh so running timer updates display
        ctx.request_repaint_after(std::time::Duration::from_millis(250));
    }
}
