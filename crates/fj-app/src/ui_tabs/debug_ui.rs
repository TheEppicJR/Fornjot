use bevy::prelude::*;

#[derive(Default)]
pub struct DebugUI;

impl DebugUI {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.set_height(0.0);
            ui.label("debug sys placeholder");
        });
    }
}
