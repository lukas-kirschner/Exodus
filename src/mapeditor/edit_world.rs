use bevy::prelude::*;
use libexodus::tiles::Tile;
use crate::{AppState, CurrentMapTextureAtlasHandle};
use crate::game::tilewrapper::MapWrapper;
use crate::game::world::{spawn_tile, WorldTile};
use crate::mapeditor::{compute_cursor_position_in_world, SelectedTile};
use crate::mapeditor::player_spawn::PlayerSpawnComponent;

pub struct EditWorldPlugin;

impl Plugin for EditWorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(AppState::MapEditor)
                .with_system(mouse_down_handler)
            )
            .add_system_set(SystemSet::on_update(AppState::MapEditor)
                .with_system(mouse_down_handler_playerspawn)
            )
        ;
    }
}

/// Delete the first tile with the given position from the view.
fn delete_tile_at(
    pos: &Vec2,
    commands: &mut Commands,
    tile_entity_query: &Query<(Entity, &mut Transform, &mut TextureAtlasSprite), With<WorldTile>>,
) {
    for (entity, transform, _) in tile_entity_query.iter() {
        if transform.translation.x as i32 == pos.x as i32 && transform.translation.y as i32 == pos.y as i32 {
            commands.entity(entity).despawn();
            return;
        }
    }
}

/// Update the first tile with the given tile in the view.
fn update_texture_at(
    pos: &Vec2,
    tile_entity_query: &mut Query<(Entity, &mut Transform, &mut TextureAtlasSprite), With<WorldTile>>,
    new_tile: &Tile,
) {
    if let Some(new_atlas_index) = new_tile.atlas_index() {
        for (_, transform, mut texture_atlas_sprite) in tile_entity_query.iter_mut() {
            if transform.translation.x as i32 == pos.x as i32 && transform.translation.y as i32 == pos.y as i32 {
                texture_atlas_sprite.index = new_atlas_index;
                return;
            }
        }
    } else {
        panic!("The tile {} does not have an atlas index", *new_tile);
    }
}

/// Replace the world tile at the given position.
fn replace_world_tile_at(
    pos: Vec2,
    new_tile: &Tile,
    mut commands: &mut Commands,
    map: &mut MapWrapper,
    tile_entity_query: &mut Query<(Entity, &mut Transform, &mut TextureAtlasSprite), With<WorldTile>>,
    atlas: &CurrentMapTextureAtlasHandle,
) {
    if let Some(current_world_tile) = map.world.get(pos.x as i32, pos.y as i32) {
        assert_ne!(*new_tile, *current_world_tile, "replace_world_tile_at() must be called with a tile different from the tile currently present in the world at {} ({})", pos, *new_tile);
        match *new_tile {
            Tile::AIR => {
                // If a tile is replaced with air, it should just be deleted from the view:
                delete_tile_at(&pos, &mut commands, tile_entity_query);
            }
            Tile::PLAYERSPAWN => {
                // Delete the tile at the given position. This action does not do anything if the current tile is Air, so we skip that check
                delete_tile_at(&pos, &mut commands, tile_entity_query);
            }
            _ => {
                if *current_world_tile == Tile::AIR || *current_world_tile == Tile::PLAYERSPAWN {
                    // The world currently contains air at the given place, i.e. just create a new tile
                    spawn_tile(commands, &atlas, new_tile.atlas_index().unwrap(), &pos, new_tile);
                } else {
                    // The world currently contains a different tile than the new one. We need to update the texture:
                    update_texture_at(&pos, tile_entity_query, new_tile);
                }
            }
        }
        map.world.set(pos.x as usize, pos.y as usize, new_tile.clone());
    }
}

fn mouse_down_handler(
    mut commands: Commands,
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut map: ResMut<MapWrapper>,
    buttons: Res<Input<MouseButton>>,
    current_tile: Res<SelectedTile>,
    mut tile_entity_query: Query<(Entity, &mut Transform, &mut TextureAtlasSprite), With<WorldTile>>,
    atlas: Res<CurrentMapTextureAtlasHandle>,
) {
    let (camera, camera_transform) = q_camera.get_single().expect("There were multiple cameras spawned");
    if buttons.pressed(MouseButton::Left) {
        if let Some((world_x, world_y)) = compute_cursor_position_in_world(&*wnds, camera, camera_transform, &*map) {
            if let Some(current_world_tile) = map.world.get(world_x, world_y) {
                if *current_world_tile != current_tile.tile {
                    replace_world_tile_at(Vec2::new(world_x as f32, world_y as f32), &current_tile.tile, &mut commands, &mut *map, &mut tile_entity_query, &*atlas);
                    map.world.set_dirty();
                }
            }
        }
    } else if buttons.pressed(MouseButton::Right) { // On Right Click, replace the current tile with air
        if let Some((world_x, world_y)) = compute_cursor_position_in_world(&*wnds, camera, camera_transform, &*map) {
            if let Some(current_world_tile) = map.world.get(world_x, world_y) {
                if *current_world_tile != Tile::AIR {
                    replace_world_tile_at(Vec2::new(world_x as f32, world_y as f32), &Tile::AIR, &mut commands, &mut *map, &mut tile_entity_query, &*atlas);
                    map.world.set_dirty();
                }
            }
        }
    }
}

fn mouse_down_handler_playerspawn(
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    map: ResMut<MapWrapper>,
    buttons: Res<Input<MouseButton>>,
    current_tile: Res<SelectedTile>,
    mut player_spawn_query: Query<&mut Transform, With<PlayerSpawnComponent>>,
) {
    if current_tile.tile == Tile::PLAYERSPAWN {
        let (camera, camera_transform) = q_camera.single(); // Will crash if there is more than one camera
        if buttons.pressed(MouseButton::Left) {
            if let Some((world_x, world_y)) = compute_cursor_position_in_world(&*wnds, camera, camera_transform, &*map) {
                let mut translation: &mut Vec3 = &mut player_spawn_query.single_mut().translation;
                translation.x = world_x as f32;
                translation.y = world_y as f32;
            }
        }
    }
}