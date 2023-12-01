use crate::game::camera::compute_viewport_to_world;
use crate::game::tilewrapper::MapWrapper;
use crate::mapeditor::edit_world::EditWorldPlugin;
use crate::mapeditor::mapeditor_ui::MapEditorUiPlugin;
use crate::mapeditor::preview_tile::MapEditorPreviewTilePlugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use libexodus::tiles::Tile;

mod edit_world;
mod mapeditor_ui;
mod player_spawn;
mod preview_tile;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
enum MapeditorSystems {
    /// The Game Board mouse handlers
    GameBoardMouseHandlers,
    /// The egui drawing routines
    UiDrawing,
    PlayerSpawnPlaceholderInit,
}

#[derive(Resource)]
pub struct SelectedTile {
    pub tile: Tile,
}

impl FromWorld for SelectedTile {
    fn from_world(_: &mut World) -> Self {
        SelectedTile { tile: Tile::AIR }
    }
}

pub struct MapEditorPlugin;

impl Plugin for MapEditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapWrapper>()
            // The world plugin is already added here. Adding it twice causes an error
            .add_plugins(MapEditorUiPlugin)
            .add_plugins(MapEditorPreviewTilePlugin)
            .add_plugins(EditWorldPlugin);
    }
}

pub fn compute_cursor_position_in_world(
    windows: &Query<&Window, With<PrimaryWindow>>,
    main_camera: &Camera,
    main_camera_transform: &GlobalTransform,
    layer_camera: &Camera,
    layer_camera_transform: &GlobalTransform,
    texture_size: f32,
) -> Option<(i32, i32)> {
    // get the window that the camera is displaying to (or the primary window)
    let Ok(wnd) = windows.get_single() else {
        return None;
    };

    // check if the cursor is inside the window and get its position, then transform it back through both cameras
    if let Some(screen_pos) = wnd.cursor_position() {
        if let Some(world_coord) = compute_viewport_to_world(
            screen_pos,
            main_camera,
            main_camera_transform,
            layer_camera,
            layer_camera_transform,
            texture_size,
        ) {
            return Some((world_coord.0.floor() as i32, world_coord.1.floor() as i32));
        }
    }
    None
}
