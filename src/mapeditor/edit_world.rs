use crate::game::camera::{LayerCamera, MainCamera};
use crate::game::tilewrapper::MapWrapper;
use crate::game::world::{spawn_tile, WorldTile};
use crate::mapeditor::player_spawn::PlayerSpawnComponent;
use crate::mapeditor::{compute_cursor_position_in_world, MapeditorSystems, SelectedTile};
use crate::{AppState, GameConfig, TilesetManager};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::PrimaryWindow;
use libexodus::tiles::Tile;

pub struct EditWorldPlugin;

impl Plugin for EditWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            mouse_down_handler
                .run_if(in_state(AppState::MapEditor))
                .in_set(MapeditorSystems::GameBoardMouseHandlers),
        )
        .add_systems(
            Update,
            mouse_down_handler_playerspawn
                .run_if(in_state(AppState::MapEditor))
                .in_set(MapeditorSystems::GameBoardMouseHandlers),
        );
    }
}

/// Delete the first tile with the given position from the view.
fn delete_tile_at(
    pos: &Vec2,
    commands: &mut Commands,
    tile_entity_query: &Query<(Entity, &mut Transform, &mut TextureAtlasSprite), With<WorldTile>>,
) {
    for (entity, transform, _) in tile_entity_query.iter() {
        if transform.translation.x as i32 == pos.x as i32
            && transform.translation.y as i32 == pos.y as i32
        {
            commands.entity(entity).despawn_recursive();
            return;
        }
    }
}

/// Update the first tile with the given tile in the view.
fn update_texture_at(
    pos: &Vec2,
    tile_entity_query: &mut Query<
        (Entity, &mut Transform, &mut TextureAtlasSprite),
        With<WorldTile>,
    >,
    new_tile: &Tile,
) {
    if let Some(new_atlas_index) = new_tile.atlas_index() {
        for (_, transform, mut texture_atlas_sprite) in tile_entity_query.iter_mut() {
            if transform.translation.x as i32 == pos.x as i32
                && transform.translation.y as i32 == pos.y as i32
            {
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
    commands: &mut Commands,
    map: &mut MapWrapper,
    tile_entity_query: &mut Query<
        (Entity, &mut Transform, &mut TextureAtlasSprite),
        With<WorldTile>,
    >,
    atlas: &TilesetManager,
    layer_query: Query<&RenderLayers, With<LayerCamera>>,
) {
    if let Some(current_world_tile) = map.world.get(pos.x as i32, pos.y as i32) {
        assert_ne!(*new_tile, *current_world_tile, "replace_world_tile_at() must be called with a tile different from the tile currently present in the world at {} ({})", pos, *new_tile);
        match *new_tile {
            Tile::AIR => {
                // If a tile is replaced with air, it should just be deleted from the view:
                delete_tile_at(&pos, commands, tile_entity_query);
            },
            Tile::PLAYERSPAWN => {
                // Delete the tile at the given position. This action does not do anything if the current tile is Air, so we skip that check
                delete_tile_at(&pos, commands, tile_entity_query);
            },
            _ => {
                if *current_world_tile == Tile::AIR || *current_world_tile == Tile::PLAYERSPAWN {
                    // The world currently contains air at the given place, i.e. just create a new tile
                    let layer: &RenderLayers = layer_query.single();
                    spawn_tile(
                        commands,
                        atlas,
                        new_tile.atlas_index().unwrap(),
                        &pos,
                        new_tile,
                        layer,
                    );
                } else {
                    // The world currently contains a different tile than the new one. We need to update the texture:
                    update_texture_at(&pos, tile_entity_query, new_tile);
                }
            },
        }
        map.world
            .set(pos.x as usize, pos.y as usize, new_tile.clone());
    }
}

fn mouse_down_handler(
    mut commands: Commands,
    wnds: Query<&Window, With<PrimaryWindow>>,
    q_layer_camera: Query<(&Camera, &GlobalTransform), With<LayerCamera>>,
    q_main_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut map: ResMut<MapWrapper>,
    buttons: Res<Input<MouseButton>>,
    current_tile: Res<SelectedTile>,
    mut tile_entity_query: Query<
        (Entity, &mut Transform, &mut TextureAtlasSprite),
        With<WorldTile>,
    >,
    atlas: Res<TilesetManager>,
    layer_query: Query<&RenderLayers, With<LayerCamera>>,
    config: Res<GameConfig>,
) {
    let (layer_camera, layer_camera_transform) = q_layer_camera
        .get_single()
        .expect("There were multiple layer cameras spawned");
    let (main_camera, main_camera_transform) = q_main_camera
        .get_single()
        .expect("There were multiple main cameras spawned");
    if buttons.just_pressed(MouseButton::Left) {
        if let Some((world_x, world_y)) = compute_cursor_position_in_world(
            &wnds,
            main_camera,
            main_camera_transform,
            layer_camera,
            layer_camera_transform,
            config.config.tile_set.texture_size() as f32,
        ) {
            if let Some(current_world_tile) = map.world.get(world_x, world_y) {
                if *current_world_tile != current_tile.tile {
                    replace_world_tile_at(
                        Vec2::new(world_x as f32, world_y as f32),
                        &current_tile.tile,
                        &mut commands,
                        &mut map,
                        &mut tile_entity_query,
                        &atlas,
                        layer_query,
                    );
                    map.world.set_dirty();
                }
            }
        }
    } else if buttons.just_pressed(MouseButton::Right) {
        // On Right Click, replace the current tile with air
        if let Some((world_x, world_y)) = compute_cursor_position_in_world(
            &wnds,
            main_camera,
            main_camera_transform,
            layer_camera,
            layer_camera_transform,
            config.config.tile_set.texture_size() as f32,
        ) {
            if let Some(current_world_tile) = map.world.get(world_x, world_y) {
                if *current_world_tile != Tile::AIR {
                    replace_world_tile_at(
                        Vec2::new(world_x as f32, world_y as f32),
                        &Tile::AIR,
                        &mut commands,
                        &mut map,
                        &mut tile_entity_query,
                        &atlas,
                        layer_query,
                    );
                    map.world.set_dirty();
                }
            }
        }
    }
}

fn mouse_down_handler_playerspawn(
    wnds: Query<&Window, With<PrimaryWindow>>,
    q_layer_camera: Query<(&Camera, &GlobalTransform), With<LayerCamera>>,
    q_main_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    _map: ResMut<MapWrapper>,
    buttons: Res<Input<MouseButton>>,
    current_tile: Res<SelectedTile>,
    mut player_spawn_query: Query<&mut Transform, With<PlayerSpawnComponent>>,
    config: Res<GameConfig>,
) {
    if current_tile.tile == Tile::PLAYERSPAWN {
        let (layer_camera, layer_camera_transform) = q_layer_camera.single();
        let (main_camera, main_camera_transform) = q_main_camera.single();
        if buttons.just_pressed(MouseButton::Left) {
            if let Some((world_x, world_y)) = compute_cursor_position_in_world(
                &wnds,
                main_camera,
                main_camera_transform,
                layer_camera,
                layer_camera_transform,
                config.config.tile_set.texture_size() as f32,
            ) {
                let translation: &mut Vec3 = &mut player_spawn_query.single_mut().translation;
                translation.x = world_x as f32 * config.config.tile_set.texture_size() as f32;
                translation.y = world_y as f32 * config.config.tile_set.texture_size() as f32;
            }
        }
    }
}
