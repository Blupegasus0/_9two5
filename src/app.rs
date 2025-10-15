use eframe::egui;
use serde::{Deserialize, Serialize};

use crate::store::*;

pub struct App {
    store: Store,
    ui_shown: Visible,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Visible {
    timer: bool,
    menu: bool,
    past_log: bool,
}


impl Default for App {
    fn default() -> Self {
        Self {
            store: Store::load(),
            ui_shown: Visible::new(),
        }
    }
}

impl eframe::App for App {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let secs = self.store.total_today_seconds();
            let hours = secs / 3600;
            let minutes = (secs % 3600) / 60;
            let seconds = secs % 60;

            if self.ui_shown.timer {
                let timer_counter = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
                let available = ui.available_size();
                let text_size: f32 = available.x * 0.25;

                let timer_button = egui::Button::new(egui::RichText::new(&timer_counter).size(text_size))
                    .fill(egui::Color32::TRANSPARENT) // no background
                    .frame(false); // no borders or shadows
                let timer_button = ui.add_sized(available, timer_button);

                if timer_button.clicked() {
                    self.store.toggle();
                }
            }

            if self.ui_shown.menu {
                let reset_button = ui.add(egui::widgets::Button::new("Reset Today"));

                if reset_button.clicked() {
                    self.store.reset_today();
                }
            }


            if self.ui_shown.past_log {
                ui.separator();

                ui.heading("Last 7 days");
                for (day, s) in self.store.totals_for_last_7_days() {
                    let h = s / 3600;
                    let m = (s % 3600) / 60;
                    ui.label(format!("{} — {:02}h:{:02}m", day, h, m));
                }

                ui.separator();

                ui.label("Note: data is saved locally in your platform data directory.");
            }
        });

        // small refresh so running timer updates display
        ctx.request_repaint_after(std::time::Duration::from_millis(250));
    }
}

impl Visible {
    fn new() -> Self {
        Visible {
            timer: true,
            menu: false,
            past_log: false,
        }
    }
}
