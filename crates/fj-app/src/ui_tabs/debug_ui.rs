use super::super::ecs::tabs::EditorTab;
use bevy::prelude::*;

#[derive(Default)]
pub struct DebugUI;

#[derive(Component)]
pub struct DebugData {
    fps: f64,
    log: String,
}

impl DebugUI {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.set_height(0.0);
            ui.label("debug sys placeholder");
        });
    }
}
