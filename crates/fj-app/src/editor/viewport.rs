use bevy::prelude::*;

#[derive(Component)]
pub struct ViweportSettings {
    control_scheme: viewport_control_scheme,
}

pub enum viewport_control_scheme {
    Solidworks,
    Fusion,
    Inventor,
    Custom,
}
