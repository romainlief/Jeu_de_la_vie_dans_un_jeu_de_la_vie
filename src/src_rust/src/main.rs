mod simulation;
mod jeu_de_la_vie;
mod gui;

use gui::GOLApp;
use eframe::NativeOptions;

fn main() {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Game of Life in Game of Life",
        native_options,
        Box::new(|_cc| Ok(Box::new(GOLApp::new(20, 10, 0.2, 200)))),
    );
}