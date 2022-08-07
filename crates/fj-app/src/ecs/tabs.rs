use bevy::prelude::*;

#[derive(Component)]
pub struct EditorTab(String);

impl EditorTab {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}
