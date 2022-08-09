use super::super::ecs::tabs::EditorTab;
use super::super::editor::editing_ui::EditingUI;
use super::{
    debug_ui::DebugUI,
    settings_ui::SettingsUI,
    tab_wrapper::{editor_ui_wrapper, EditorUiWrapper},
    welcome_ui::WelcomeUI,
};
use bevy::prelude::*;

pub fn add_editing_tab(mut commands: Commands) {
    commands
        .spawn()
        .insert(EditorTab::new("Debug".to_string()))
        .insert(EditorUiWrapper {
            tab_ui: editor_ui_wrapper::Editor(EditingUI::default()),
        });
}

pub fn add_welcome_tab(mut commands: Commands) {
    commands
        .spawn()
        .insert(EditorTab::new("Welcome".to_string()))
        .insert(EditorUiWrapper {
            tab_ui: editor_ui_wrapper::Welcome(WelcomeUI::default()),
        });
}

pub fn add_debug_tab(mut commands: Commands) {
    commands
        .spawn()
        .insert(EditorTab::new("Debug".to_string()))
        .insert(EditorUiWrapper {
            tab_ui: editor_ui_wrapper::Debug(DebugUI::default()),
        });
}

pub fn add_settings_tab(mut commands: Commands) {
    commands
        .spawn()
        .insert(EditorTab::new("Settings".to_string()))
        .insert(EditorUiWrapper {
            tab_ui: editor_ui_wrapper::Settings(SettingsUI::default()),
        });
}
