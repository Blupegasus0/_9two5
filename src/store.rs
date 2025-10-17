use chrono::{DateTime, Datelike, Local};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Record {
    start: DateTime<Local>,
    end: Option<DateTime<Local>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    records: Vec<Record>,
    timer_state: TimerState,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TimerState {
    Work,
    Break,
    Done,
}

impl Store {
    pub fn get_timer_state(&self) -> &TimerState {
        &self.timer_state
    }

    fn path() -> Option<PathBuf> {
        if let Some(proj) = ProjectDirs::from("com", "you", "_9two5") {
            let dir = proj.data_local_dir();
            let _ = std::fs::create_dir_all(dir);
            return Some(dir.join("times.json"));
        }
        None
    }

    pub fn load() -> Self {
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

    pub fn toggle(&mut self) {
        // if already running, stop
        let last = self.records.last_mut().unwrap();
        if last.end.is_none() {
            last.end = Some(Local::now());
            self.persist();
            self.timer_state = TimerState::Break;
            return;
        }

        self.records.push(Record {
            start: Local::now(),
            end: None,
        });
        self.timer_state = TimerState::Work;
        self.persist();
    }

    pub fn reset_today(&mut self) {
        let today = Local::now();
        self.records.retain(|r| r.start != today);
        self.persist();
    }

    pub fn total_today_seconds(&self) -> i64 {
        let today = Local::now().day();
        self.records
            .iter()
            .filter(|r| r.start.day() == today)
            .map(|r| {
                let end = r.end.unwrap_or_else(|| Local::now());
                let secs = (end - r.start).num_seconds();
                if secs > 0 { secs } else { 0 }
            })
            .sum()
    }

    pub fn totals_for_last_7_days(&self) -> Vec<(DateTime<Local>, i64)> {
        let mut sums = std::collections::BTreeMap::new();
        let today = Local::now();
        for d in 0..7 {
            sums.insert(today - chrono::Duration::days(d), 0i64);
        }
        for r in &self.records {
            let start_date = r.start;
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

impl Default for Store {
    fn default() -> Self {
        Store {
            records: vec![
                Record {
                    start: Local::now(),
                    end: None,
                }
            ],
            timer_state: TimerState::Work,
        }
    }
}
