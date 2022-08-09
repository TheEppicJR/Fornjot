use super::super::ecs::tabs::EditorTab;
use bevy::prelude::*;

#[derive(Default)]
pub struct EditingUI {
    project: String,
    name: String,
}

impl EditingUI {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.set_height(0.0);
            ui.label("editing placeholder");
        });
    }
}
