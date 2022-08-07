use super::super::ecs::tabs::EditorTab;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct SettingsUI;

impl SettingsUI {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.set_height(0.0);
            ui.label("Settings placeholder");
        });
    }
}

pub fn add_settings_tab(mut commands: Commands) {
    commands
        .spawn()
        .insert(EditorTab::new("Settings".to_string()))
        .insert(SettingsUI::default());
}
