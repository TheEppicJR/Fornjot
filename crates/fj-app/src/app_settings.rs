use crate::editor::viewport::viewport_control_scheme;
use bevy::prelude::*;

#[derive(Component)]
struct Setting;

#[derive(Component)]
struct Darkmode(bool);

#[derive(Component)]
struct ViewportControl(viewport_control_scheme);

#[derive(Component)]
struct DebugSettings(EguiOptionsState);
