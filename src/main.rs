pub mod egui_app;
pub mod gen_graph;

fn main() {
    // Can't get this to work. Should call it from the HTML?
    //let options = eframe::WebOptions::default();
    //eframe::start_web("SamGraphKo", options, Box::new(|cc| Box::new(MyApp::new(cc))))

    // @TODO Replace this with some real graph?
    let graph = gen_graph::Graph::default();

    egui_app::start_app(graph)    
}

