use eframe::egui;

mod app;
mod store;

use crate::app::*;


fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(350.0, 160.0)),
        always_on_top: true,
        decorated: true, // small frameless look
        resizable: true,
        ..Default::default()
    };
    let _ = eframe::run_native(
        "_9two5",
        native_options,
        Box::new(|_cc| Box::new(App::default())),
    );
}
