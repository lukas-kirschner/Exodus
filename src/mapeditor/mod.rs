use crate::game::tilewrapper::MapWrapper;
use crate::mapeditor::edit_world::EditWorldPlugin;
use crate::mapeditor::mapeditor_ui::MapEditorUiPlugin;
use crate::mapeditor::preview_tile::MapEditorPreviewTilePlugin;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use libexodus::tiles::Tile;

mod edit_world;
mod mapeditor_ui;
mod player_spawn;
mod preview_tile;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum MapeditorSystems {
    /// The Game Board mouse handlers
    GameBoardMouseHandlers,
    /// The egui drawing routines
    UiDrawing,
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
            .add_plugin(MapEditorUiPlugin)
            .add_plugin(MapEditorPreviewTilePlugin)
            .add_plugin(EditWorldPlugin);
    }
}

pub fn compute_cursor_position_in_world(
    windows: &Windows,
    main_camera: &Camera,
    main_camera_transform: &GlobalTransform,
    _layer_camera: &Camera,
    _layer_camera_transform: &GlobalTransform,
) -> Option<(i32, i32)> {
    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = main_camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position, then transform it back through both cameras
    if let Some(screen_pos) = wnd.cursor_position() {
        // if let Some(screen_pos) = layer_camera.viewport_to_world(layer_camera_transform, screen_pos)
        // {
        if let Some(screen_pos) = main_camera.viewport_to_world(main_camera_transform, screen_pos) {
            let ret = ((screen_pos.origin.x), (screen_pos.origin.y));
            debug!("Pos {},{}", ret.0, ret.1);
            return Some((ret.0 as i32, ret.1 as i32));
        }
        // }
    }
    None
}
