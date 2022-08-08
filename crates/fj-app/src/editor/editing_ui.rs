use super::super::ecs::tabs::EditorTab;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct EditingUI {
    project: String,
    name: String,
}

impl EditingUI {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.set_height(0.0);
            ui.label("debug sys placeholder");
        });
    }
}

pub fn add_debug_tab(mut commands: Commands) {
    commands
        .spawn()
        .insert(EditorTab::new("Debug".to_string()))
        .insert(EditingUI::default());
}
