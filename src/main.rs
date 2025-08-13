use chrono::{DateTime, Local, NaiveDate};
use directories::ProjectDirs;
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Record {
    start: DateTime<Local>,
    end: Option<DateTime<Local>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Store {
    records: Vec<Record>,
}

impl Store {
    fn path() -> Option<PathBuf> {
        if let Some(proj) = ProjectDirs::from("com", "you", "_9two5") {
            let dir = proj.data_local_dir();
            let _ = std::fs::create_dir_all(dir);
            return Some(dir.join("times.json"));
        }
        None
    }

    fn load() -> Self {
        if let Some(p) = Store::path() {
            if let Ok(s) = fs::read_to_string(p) {
                if let Ok(store) = serde_json::from_str::<Store>(&s) {
                    return store;
                }
            }
        }
        Store::default()
    }

    fn persist(&self) {
        if let Some(p) = Store::path() {
            if let Ok(js) = serde_json::to_string_pretty(self) {
                let _ = fs::write(p, js);
            }
        }
    }

    fn start(&mut self) {
        // if already running, do nothing
        if self.records.last().map(|r| r.end.is_none()).unwrap_or(false) {
            return;
        }
        self.records.push(Record {
            start: Local::now(),
            end: None,
        });
        self.persist();
    }

    fn stop(&mut self) {
        if let Some(last) = self.records.last_mut() {
            if last.end.is_none() {
                last.end = Some(Local::now());
                self.persist();
            }
        }
    }

    fn reset_today(&mut self) {
        let today = Local::today().naive_local();
        self.records.retain(|r| r.start.date().naive_local() != today);
        self.persist();
    }

    fn total_today_seconds(&self) -> i64 {
        let today = Local::today().naive_local();
        self.records
            .iter()
            .filter(|r| r.start.date().naive_local() == today)
            .map(|r| {
                let end = r.end.unwrap_or_else(|| Local::now());
                let secs = (end - r.start).num_seconds();
                if secs > 0 { secs } else { 0 }
            })
            .sum()
    }

    fn totals_for_last_7_days(&self) -> Vec<(NaiveDate, i64)> {
        let mut sums = std::collections::BTreeMap::new();
        let today = Local::today().naive_local();
        for d in 0..7 {
            sums.insert(today - chrono::Duration::days(d), 0i64);
        }
        for r in &self.records {
            let start_date = r.start.date().naive_local();
            let end = r.end.unwrap_or_else(|| Local::now());
            let secs = (end - r.start).num_seconds();
            // only count if within the last 7 days by start date
            if let Some(val) = sums.get_mut(&start_date) {
                *val += if secs > 0 { secs } else { 0 };
            }
        }
        let mut out: Vec<_> = sums.into_iter().collect();
        out.reverse(); // oldest -> newest
        out
    }
}

struct App {
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

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 160.0)),
        always_on_top: true,
        decorated: true, // small frameless look
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "_9two5",
        native_options,
        Box::new(|_cc| Box::new(App::default())),
    );
}
