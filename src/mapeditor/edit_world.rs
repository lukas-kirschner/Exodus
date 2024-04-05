use crate::dialogs::edit_message_dialog::EditMessageDialog;
use crate::dialogs::DialogResource;
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
            // This ordering of function calls needs to be kept intact, else the player spawn placement will NOT work anymore!!!
            (mouse_down_handler, mouse_down_handler_playerspawn)
                .chain()
                .run_if(in_state(AppState::MapEditor))
                .in_set(MapeditorSystems::GameBoardMouseHandlers),
        );
    }
}

/// Delete the first tile with the given position from the view.
fn view_delete_tile_at(
    pos: &Vec2,
    commands: &mut Commands,
    tile_entity_query: &Query<(Entity, &mut Transform, &mut TextureAtlas), With<WorldTile>>,
    map_texture_atlas: &TilesetManager,
) {
    for (entity, transform, _) in tile_entity_query.iter() {
        if transform.translation.x as i32
            == (pos.x as i32 * map_texture_atlas.current_tileset.texture_size() as i32)
            && transform.translation.y as i32
                == (pos.y as i32 * map_texture_atlas.current_tileset.texture_size() as i32)
        {
            commands.entity(entity).despawn_recursive();
            debug!("Deleted tile at {},{}", pos.x, pos.y);
            return;
        }
    }
}

/// Update the tile texture of the given tile to match the new tile.
fn update_texture_at(
    pos: &Vec2,
    tile_entity_query: &mut Query<(Entity, &mut Transform, &mut TextureAtlas), With<WorldTile>>,
    new_tile: &Tile,
    map_texture_atlas: &TilesetManager,
) {
    if let Some(new_atlas_index) = new_tile.atlas_index() {
        for (_, transform, mut atlas) in tile_entity_query.iter_mut() {
            if transform.translation.x as i32
                == (pos.x as i32 * map_texture_atlas.current_tileset.texture_size() as i32)
                && transform.translation.y as i32
                    == (pos.y as i32 * map_texture_atlas.current_tileset.texture_size() as i32)
            {
                atlas.index = new_atlas_index;
                debug!("Updated tile texture at position {},{}", pos.x, pos.y);
                return;
            }
        }
    } else {
        panic!("The tile {} does not have an atlas index", *new_tile);
    }
}

/// Replace the world tile at the given position. Takes care of handling different tile kinds.
fn replace_world_tile_at(
    pos: Vec2,
    new_tile: &Tile,
    commands: &mut Commands,
    map: &mut MapWrapper,
    tile_entity_query: &mut Query<(Entity, &mut Transform, &mut TextureAtlas), With<WorldTile>>,
    atlas: &TilesetManager,
    layer_query: Query<&RenderLayers, With<LayerCamera>>,
) {
    if let Some(current_world_tile) = map.world.get(pos.x as i32, pos.y as i32) {
        debug!(
            "Replacing world tile {},{} ({}) with {}",
            pos.x, pos.y, current_world_tile, new_tile
        );
        match *new_tile {
            Tile::AIR => {
                // If a tile is replaced with air, it should just be deleted from the view:
                view_delete_tile_at(&pos, commands, tile_entity_query, atlas);
            },
            Tile::PLAYERSPAWN => {
                // Delete the tile at the given position. This action does not do anything if the current tile is Air, so we skip that check
                view_delete_tile_at(&pos, commands, tile_entity_query, atlas);
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
                    update_texture_at(&pos, tile_entity_query, new_tile, atlas);
                }
            },
        }

        // Now, actually replace the tile in the world:
        match *new_tile {
            Tile::MESSAGE { .. } => {
                if matches!(current_world_tile, Tile::MESSAGE { .. }) {
                    // Do nothing, there already is a message tile. Leave it and do not change any ID
                } else {
                    // Create a new Message ID and place the message tile in the world:
                    map.world
                        .set_message_tile(pos.x as usize, pos.y as usize, "".to_string());
                }
            },
            _ => {
                map.world
                    .set(pos.x as usize, pos.y as usize, new_tile.clone());
            },
        }
        map.world.set_dirty();
    }
}

fn mouse_down_handler(
    mut commands: Commands,
    wnds: Query<&Window, With<PrimaryWindow>>,
    q_layer_camera: Query<(&Camera, &GlobalTransform), With<LayerCamera>>,
    q_main_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut map: ResMut<MapWrapper>,
    buttons: Res<ButtonInput<MouseButton>>,
    current_tile: Res<SelectedTile>,
    mut tile_entity_query: Query<(Entity, &mut Transform, &mut TextureAtlas), With<WorldTile>>,
    atlas: Res<TilesetManager>,
    layer_query: Query<&RenderLayers, With<LayerCamera>>,
    config: Res<GameConfig>,
    mut state: ResMut<NextState<AppState>>,
) {
    let (layer_camera, layer_camera_transform) = q_layer_camera
        .get_single()
        .expect("There were multiple layer cameras spawned");
    let (main_camera, main_camera_transform) = q_main_camera
        .get_single()
        .expect("There were multiple main cameras spawned");
    if buttons.just_released(MouseButton::Left) {
        if let Some((world_x, world_y)) = compute_cursor_position_in_world(
            &wnds,
            main_camera,
            main_camera_transform,
            layer_camera,
            layer_camera_transform,
            config.texture_size(),
        ) {
            replace_world_tile_at(
                Vec2::new(world_x as f32, world_y as f32),
                &current_tile.tile,
                &mut commands,
                &mut map,
                &mut tile_entity_query,
                &atlas,
                layer_query,
            );
            if let Some(Tile::MESSAGE { message_id }) = map.world.get(world_x, world_y) {
                // The new tile is a message with the given ID. Open the Editor and allow the user to set or edit the message. Setting an appropriate ID has been taken care of by the replace_world_tile_at function.
                commands.insert_resource(DialogResource {
                    ui_dialog: Box::new(EditMessageDialog::new(
                        *message_id,
                        map.world.get_message(*message_id).unwrap_or("").to_string(),
                    )),
                });
                state.set(AppState::MapEditorDialog);
            }
        }
    } else if buttons.just_released(MouseButton::Right) {
        // On Right Click, replace the current tile with air
        if let Some((world_x, world_y)) = compute_cursor_position_in_world(
            &wnds,
            main_camera,
            main_camera_transform,
            layer_camera,
            layer_camera_transform,
            config.texture_size(),
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
    buttons: Res<ButtonInput<MouseButton>>,
    current_tile: Res<SelectedTile>,
    mut player_spawn_query: Query<&mut Transform, With<PlayerSpawnComponent>>,
    config: Res<GameConfig>,
) {
    if current_tile.tile == Tile::PLAYERSPAWN && buttons.just_pressed(MouseButton::Left) {
        let (layer_camera, layer_camera_transform) = q_layer_camera.single();
        let (main_camera, main_camera_transform) = q_main_camera.single();
        if let Some((world_x, world_y)) = compute_cursor_position_in_world(
            &wnds,
            main_camera,
            main_camera_transform,
            layer_camera,
            layer_camera_transform,
            config.texture_size(),
        ) {
            if world_x > 0
                && world_x < _map.world.width() as i32
                && world_y > 0
                && world_y < _map.world.height() as i32
            {
                let translation: &mut Vec3 = &mut player_spawn_query.single_mut().translation;
                translation.x = world_x as f32 * config.texture_size();
                translation.y = world_y as f32 * config.texture_size();
            }
        }
    }
}
