mod simulation;
mod jeu_de_la_vie;
mod gui;

use gui::GOLApp;
use eframe::NativeOptions;

fn main() {
    gui::launch_gui_application().unwrap();
}