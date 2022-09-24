// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use crate::gen_graph::Graph;

// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen::prelude::*;

struct EApp {
    graph: Graph,
}

pub fn start_app(graph: Graph) {
    let options = eframe::NativeOptions::default();
    eframe::run_native("SamGraphKo", options, Box::new(|_cc| Box::new(EApp::new(graph))),);

}

impl EApp {
    fn new(graph: Graph) -> Self {
        Self {graph: graph}
    }
}

impl eframe::App for EApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("SamGraphKo");
            

            ui.horizontal(|ui| {
                ui.label("Some text: ");
                ui.text_edit_singleline(&mut "Example".to_owned());
            });
            
            
            if ui.button("Click each year").clicked() {
                ui.label("You clicked me!");
            }
            
            ui.label(format!("Hello! {}", "lol"));
        });
    }
}