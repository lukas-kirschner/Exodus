use bevy::prelude::*;
use crate::AppState;
use crate::game::tilewrapper::MapWrapper;
use crate::game::world::WorldPlugin;
use crate::mapeditor::mapeditor_ui::MapEditorUiPlugin;

mod mapeditor_ui;

pub struct MapEditorPlugin;

impl Plugin for MapEditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MapWrapper>()
            // .add_plugin(WorldPlugin) // This plugin is already added
            .add_plugin(MapEditorUiPlugin)
        ;
    }
}