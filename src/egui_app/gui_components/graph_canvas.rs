
use eframe::egui::Ui;
use egui::*;

use crate::gen_graph;

pub struct Canvas {
    graph: gen_graph::Graph,

    stroke: Stroke,
    fill: Color32,
    background: (Rounding, Color32),
    pub c: f32,
}


impl Default for Canvas {
    fn default() -> Self {
        Self { 
            graph: gen_graph::Graph::example(),
            stroke: Stroke::new(2.0, Color32::from_rgb(25, 200, 100)),
            fill: Color32::from_rgb(255, 100, 100),
            background: (Rounding::none(), Color32::BLACK),
            c: 100.0,
         }
    }
}


impl Canvas {
    pub fn populate(&mut self, ui: &mut Ui) {
        let size = ui.max_rect();
        let p = ui.painter_at(size);
        let (r, c) = self.background;
        p.rect_filled(size, r, c);
        p.circle(size.center(), self.c, self.fill, self.stroke);
    }
}


//Move below code to own place later

//struct GraphLayout {
//    node_positions: Vec< 
//[dependencies] nalgebra = "*" # replace * by the latest version of the crate.
//}