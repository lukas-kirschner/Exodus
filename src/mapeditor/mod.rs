use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use libexodus::tiles::Tile;
use crate::game::tilewrapper::MapWrapper;
use crate::mapeditor::edit_world::EditWorldPlugin;
use crate::mapeditor::mapeditor_ui::MapEditorUiPlugin;
use crate::mapeditor::preview_tile::MapEditorPreviewTilePlugin;

mod mapeditor_ui;
mod player_spawn;
mod preview_tile;
mod edit_world;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
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
        app
            .init_resource::<MapWrapper>()
            // The world plugin is already added here. Adding it twice causes an error
            .add_plugin(MapEditorUiPlugin)
            .add_plugin(MapEditorPreviewTilePlugin)
            .add_plugin(EditWorldPlugin)
        ;
    }
}

pub fn compute_cursor_position_in_world(
    windows: &Windows,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    _: &MapWrapper,
) -> Option<(i32, i32)> {
    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position, then transform it back through the camera
    if let Some(screen_pos) = wnd.cursor_position() {
        if let Some(screen_pos) = camera.viewport_to_world(camera_transform, screen_pos) {
            return Some(((screen_pos.origin.x + 0.5) as i32, (screen_pos.origin.y + 0.5) as i32));
        }
    }
    return None;
}