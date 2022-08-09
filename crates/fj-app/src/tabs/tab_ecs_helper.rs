use super::tree::Tree;
use crate::ecs::tabs::{EditorTab, EditorTabId};
use bevy::prelude::*;

pub fn check_for_new_tabs(
    mut query: Query<(&EditorTab, Entity), (Without<EditorTabId>)>,
    mut tree: ResMut<Tree>,
    mut commands: Commands,
) {
    for (edit_tab, entity) in query.iter_mut() {
        tree.append_tab_to_tabs(
            edit_tab.name.clone(),
            entity.id(),
            entity.generation(),
        );
        commands.get_or_spawn(entity).insert(EditorTabId::new(0));
    }
}
