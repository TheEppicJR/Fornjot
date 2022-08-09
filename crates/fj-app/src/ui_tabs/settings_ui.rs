use super::super::ecs::tabs::EditorTab;
use crate::editor::code_editor::syntax_highlighting::CodeTheme;
use bevy::prelude::*;
#[derive(Default)]
pub struct SettingsUI;

#[derive(Component, Default)]
pub struct AppSetting;

impl SettingsUI {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.set_height(0.0);
            ui.label("Settings placeholder");
        });
        ui.add_space(16.0);
        let mut theme = CodeTheme::from_memory(ui.ctx());
        ui.collapsing("Theme", |ui| {
            ui.group(|ui| {
                theme.ui(ui);
                theme.clone().store_in_memory(ui.ctx());
            });
        });
    }
}

pub fn init_app_settings(mut commands: Commands) {
    todo!();
}
