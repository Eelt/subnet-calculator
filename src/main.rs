#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use app::SubnetCalculatorApp;


    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Subnet Calculator",
        native_options,
            Box::new(|cc| Box::new(SubnetCalculatorApp::new(cc))),
    );
}
