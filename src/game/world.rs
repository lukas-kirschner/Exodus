use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use libexodus::tiles::{Tile, TileKind};
use libexodus::world::GameWorld;
use crate::{AppState, LAYER_ID};
use crate::game::camera::{destroy_camera, handle_ui_resize, setup_camera};
use crate::game::constants::WORLD_Z;
use crate::game::pickup_item::insert_wrappers;
use crate::game::tilewrapper::MapWrapper;
use crate::tileset_manager::TilesetManager;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Playing)
                .with_system(reset_world.label("world"))
            )
            .add_system_set(SystemSet::on_enter(AppState::Playing)
                .with_system(setup_camera.after("world").label("camera"))
            )
            .add_system_set(SystemSet::on_update(AppState::Playing)
                .with_system(handle_ui_resize.after("gameui"))
            )
            .add_system_set(SystemSet::on_exit(AppState::Playing)
                .with_system(destroy_camera)
            )
            // Map Editor needs a world as well:
            .add_system_set(SystemSet::on_enter(AppState::MapEditor)
                .with_system(reset_world.label("world"))
            )
            .add_system_set(SystemSet::on_enter(AppState::MapEditor)
                .with_system(setup_camera.after("world").label("camera"))
            )
            .add_system_set(SystemSet::on_update(AppState::MapEditor)
                .with_system(handle_ui_resize.after("gameui"))
            )
            .add_system_set(SystemSet::on_exit(AppState::MapEditor)
                .with_system(destroy_camera)
            )
        ;
    }
}

#[derive(Component)]
pub struct WorldTile;

#[derive(Component)]
pub struct DoorWrapper;

pub fn insert_door_wrappers(
    tile: &Tile,
    bundle: &mut EntityCommands,
) {
    match tile.kind() {
        TileKind::DOOR => {
            bundle.insert(DoorWrapper);
        }
        _ => {}
    }
}

/// Spawn a single tile at the given position
pub fn spawn_tile(
    commands: &mut Commands,
    map_texture_atlas: &TilesetManager,
    atlas_index: usize,
    tile_position: &Vec2,
    tile: &Tile,
    layer: &RenderLayers,
) {
    let mut bundle: EntityCommands = commands
        .spawn((SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(atlas_index),
            texture_atlas: map_texture_atlas.current_handle(),
            transform: Transform {
                translation: tile_position.extend(WORLD_Z),
                scale: Vec3::splat(1.0 / map_texture_atlas.current_tileset().texture_size() as f32),
                ..default()
            },
            ..Default::default()
        },
                WorldTile,// WorldTiles are attached to each world tile, while TileWrappers are only attached to non-interactive world tiles.
                layer.clone()));
    insert_wrappers(tile, &mut bundle);
    insert_door_wrappers(tile, &mut bundle);
}

pub fn setup_game_world(
    mut commands: Commands,
    mut worldwrapper: ResMut<MapWrapper>,
    the_atlas_handle: Res<TilesetManager>,
) {
    let layer: RenderLayers = RenderLayers::layer(LAYER_ID);
    // Set Background Color
    let bg_color = the_atlas_handle.current_tileset.background_color();
    commands.insert_resource(ClearColor(Color::rgb_u8(bg_color.r, bg_color.g, bg_color.b)));
    // Load the world
    let world: &mut GameWorld = &mut worldwrapper.world;

    for row in 0..world.height() {
        for col in 0..world.width() {
            let tile_position = Vec2::new(
                col as f32,
                row as f32,
            );
            let tile = world.get(col as i32, row as i32)
                .expect(format!("Coordinate {},{} not accessible in world of size {},{}",
                                col, row, world.width(), world.height())
                    .as_str());
            if let Some(index) = tile.atlas_index() {
                spawn_tile(&mut commands, &*the_atlas_handle, index, &tile_position, tile, &layer);
            }
        }
    }
}

///
/// Delete everything world-related and respawn the world, including coins
pub fn reset_world(
    mut commands: Commands,
    mut worldwrapper: ResMut<MapWrapper>,
    tiles_query: Query<Entity, With<WorldTile>>,
    the_atlas_handle: Res<TilesetManager>,
) {
    for entity in tiles_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    worldwrapper.world.reset_game_state();
    setup_game_world(commands, worldwrapper, the_atlas_handle);
}