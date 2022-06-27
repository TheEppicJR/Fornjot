use crate::main_ui::{Fornjot, FornjotConfig};

use eframe::{
    egui::containers::{CentralPanel, SidePanel, TopBottomPanel},
    egui::Vec2,
    run_native, App, CreationContext, NativeOptions,
};

impl App for Fornjot {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        SidePanel::left("demo_ui").show(ctx, |ui| ui.label("hi"));
    }
}

pub fn run_app() {
    let app = Fornjot::default();
    let mut win_option = NativeOptions::default();
    // win_option.initial_window_size = Some(Vec2::new(540., 960.));
    run_native("Fornjot", win_option, Box::new(|cc| Box::new(app.init(cc))));
}
