use eframe::egui;

mod app;
mod store;

use crate::app::*;


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
