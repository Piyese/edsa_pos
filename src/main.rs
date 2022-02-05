
mod salepoint;
mod styles;
// pub mod setup;

use salepoint::State;
use eframe::egui::Vec2;

fn main() {
    let app = State::default();
    let mut native_options = eframe::NativeOptions::default();
    // native_options.resizable = false;
    native_options.initial_window_size=Some(Vec2::new(1150.0, 620.0));
    eframe::run_native(Box::new(app), native_options);
}