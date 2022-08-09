use super::super::editor::editing_ui::EditingUI;
use super::{
    debug_ui::DebugUI, settings_ui::SettingsUI, welcome_ui::WelcomeUI,
};
use bevy::prelude::*;

pub enum editor_ui_wrapper {
    None,
    Editor(EditingUI),
    Welcome(WelcomeUI),
    Settings(SettingsUI),
    Debug(DebugUI),
}
#[derive(Component)]
pub struct EditorUiWrapper {
    pub tab_ui: editor_ui_wrapper,
}
