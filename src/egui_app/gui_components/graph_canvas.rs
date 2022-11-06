extern crate nalgebra as na;

use eframe::egui::Ui;
use egui::*;

use crate::gen_graph:: {Graph};

pub struct Canvas {
    graph_layout: GraphLayout,
    stroke: Stroke,
    fill: Color32,
    background: (Rounding, Color32),
    pub c: f32,
}


impl Default for Canvas {
    fn default() -> Self {

        let graph = Graph::example();
        let graph_layout = GraphLayout::new(graph);
        Self { 
            graph_layout: graph_layout, 
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
        for node in self.graph_layout.positions() {
            let pos = size.center() + Vec2 {x: node.x as f32, y: node.y as f32};
            p.circle(pos, 10.0, self.fill, self.stroke);
        }
        //for edge in self.graph_layout.edges() {
        //    p.line_segment(edge, Stroke::new(2.0, Color32::from_rgb(255, 255, 255)));
        //}
    }
}


//Move below code to own place later

struct GraphLayout {
    node_positions: Vec<na::Vector2<f64>>,
    graph: Graph,
}

impl GraphLayout {
    pub fn new(graph: Graph) -> GraphLayout {
        let n = graph.nbr_nodes();
        let rot = na::Rotation2::<f64>::new(std::f64::consts::PI*2.0/(n as f64));
        let mut vec = na::Vector2::<f64>::x()*50.0;
        let mut positions = Vec::new();
        for i in 0..n {
            positions.push(vec.clone());
            vec = rot * vec;
        }
             
        GraphLayout {
            node_positions: positions, 
            graph: graph,
        }
    }

    pub fn positions(&self) -> &Vec<na::Vector2<f64>> {
        &self.node_positions
    }
}
