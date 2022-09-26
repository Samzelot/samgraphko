// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use crate::gen_graph::Graph;
mod gui_components;
use gui_components::graph_canvas::{Canvas};

// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen::prelude::*;

struct EApp {
    graph: Graph,
    text: String,
    clicked: bool,
    canvas: Canvas,
}

pub fn start_app(graph: Graph) {
    let options = eframe::NativeOptions::default();
    eframe::run_native("SamGraphKo", options, Box::new(|_cc| Box::new(EApp::new(graph))),);

}

impl EApp {
    fn new(graph: Graph) -> Self {
        Self {
            graph: graph,
            text: String::from("Example text"),
            clicked: false,
            canvas: Default::default()
        }
    }
}

impl eframe::App for EApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
        ui.heading("SamGraphKo");
            ui.horizontal(|ui| {
                ui.label("Some text: ");
                ui.text_edit_singleline(&mut self.text);
            });
            
            if ui.button("Click each year").clicked() {
                self.clicked = !self.clicked;
            }
            if self.clicked {
                self.canvas.c = 300.0;
                ui.label("You clicked me!");
            }
            
            ui.label(format!("Hello! {}", "lol"));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.canvas.populate(ui);
        });
    }
}