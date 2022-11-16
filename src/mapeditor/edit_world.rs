use bevy::prelude::*;
use libexodus::player::Player;
use libexodus::tiles::Tile;
use crate::{App, AppState, CurrentMapTextureAtlasHandle, CurrentPlayerTextureAtlasHandle, MapEditorPlugin, TEXTURE_SIZE};
use crate::CursorIcon::Default;
use crate::game::constants::{MAPEDITOR_PREVIEWTILE_AIR_ATLAS_INDEX, MAPEDITOR_PREVIEWTILE_ALPHA, MAPEDITOR_PREVIEWTILE_Z, TILE_SIZE};
use crate::game::tilewrapper::MapWrapper;
use crate::mapeditor::{compute_cursor_position_in_world, SelectedTile};

pub struct EditWorldPlugin;

impl Plugin for EditWorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(AppState::MapEditor)
                .with_system(mouse_down_handler)
            )
        ;
    }
}

fn mouse_down_handler(
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    map: Res<MapWrapper>,
    buttons: Res<Input<MouseButton>>,
    current_tile: Res<SelectedTile>,
) {
    if buttons.pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single(); // Will crash if there is more than one camera
        if let Some((world_x, world_y)) = compute_cursor_position_in_world(&*wnds, camera, camera_transform, &*map) {
            if let Some(current_world_tile) = map.world.get(world_x, world_y) {
                if *current_world_tile != current_tile.tile {
                    // replace_world_tile_at();
                }
            }
        }
    }
}