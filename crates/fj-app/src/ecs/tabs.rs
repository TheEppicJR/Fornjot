use bevy::prelude::*;

#[derive(Component)]
pub struct EditorTab {
    pub name: String,
}

impl EditorTab {
    pub fn new(name: String) -> Self {
        Self { name: name }
    }
}

#[derive(Component)]
pub struct EditorTabId(u32);

impl EditorTabId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}
