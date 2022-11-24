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
    wnds: &Windows,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    map: &MapWrapper,
) -> Option<(i32, i32)> {
    // Code similar to https://bevy-cheatbook.github.io/cookbook/cursor2world.html

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate() + Vec2::new(0.5, 0.5);
        return if
        world_pos.x < map.world.width() as f32 &&
            world_pos.y < map.world.height() as f32 &&
            world_pos.x >= 0. &&
            world_pos.y >= 0. {
            Some((world_pos.x as i32, world_pos.y as i32))
        } else {
            None
        };
    }
    return None;
}