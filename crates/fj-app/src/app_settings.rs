use bevy::prelude::*;
use crate::editor::viewport::viewport_control_scheme;

#[derive(Component)]
struct Setting;

#[derive(Component)]
struct Darkmode(bool);

#[derive(Component)]
struct ViewportControl(viewport_control_scheme);

#[derive(Default)]
pub struct EguiOptionsState {
    pub show_debug_window: bool,
    pub show_trace: bool,
    pub show_layout_debug_on_hover: bool,
    pub show_debug_text_example: bool,
    pub show_original_ui: bool,
    pub show_settings_ui: bool,
}

#[derive(Component)]
struct DebugSettings(EguiOptionsState);