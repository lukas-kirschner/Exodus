use crate::game::camera::{destroy_camera, handle_ui_resize, setup_camera};
use crate::game::constants::WORLD_Z;
use crate::game::pickup_item::insert_wrappers;
use crate::game::tilewrapper::MapWrapper;
use crate::textures::tileset_manager::TilesetManager;
use crate::{AppLabels, AppState, LAYER_ID};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use libexodus::tiles::{Tile, TileKind};
use libexodus::world::GameWorld;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(reset_world.in_schedule(OnEnter(AppState::Playing)).in_set(AppLabels::World))
            .add_system(setup_camera.in_schedule(OnEnter(AppState::Playing)).after(AppLabels::World).in_set(AppLabels::Camera))
            .add_system(handle_ui_resize.in_set(OnUpdate(AppState::Playing)).after(AppLabels::GameUI))
            .add_system(destroy_camera.in_schedule(OnExit(AppState::Playing)))
        // Map Editor needs a world as well:
            .add_system(reset_world.in_schedule(OnEnter(AppState::MapEditor)).in_set(AppLabels::World))
            .add_system(setup_camera.in_schedule(OnEnter(AppState::MapEditor)).after(AppLabels::World).in_set(AppLabels::Camera))
            .add_system(handle_ui_resize.in_set(OnUpdate(AppState::MapEditor)).after(AppLabels::GameUI))
            .add_system(destroy_camera.in_schedule(OnExit(AppState::MapEditor)));
    }
}

#[derive(Component)]
pub struct WorldTile;

#[derive(Component)]
pub struct DoorWrapper;

pub fn insert_door_wrappers(tile: &Tile, bundle: &mut EntityCommands) {
    if tile.kind() == TileKind::DOOR {
        bundle.insert(DoorWrapper);
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
    let mut bundle: EntityCommands = commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(atlas_index),
            texture_atlas: map_texture_atlas.current_handle(),
            transform: Transform {
                // Multiply the position by the texture size
                translation: (*tile_position
                    * (map_texture_atlas.current_tileset().texture_size() as f32))
                    .extend(WORLD_Z),
                ..default()
            },
            ..Default::default()
        },
        WorldTile, // WorldTiles are attached to each world tile, while TileWrappers are only attached to non-interactive world tiles.
        *layer,
    ));
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
    commands.insert_resource(ClearColor(Color::rgb_u8(
        bg_color.r, bg_color.g, bg_color.b,
    )));
    // Load the world
    let world: &mut GameWorld = &mut worldwrapper.world;

    for row in 0..world.height() {
        for col in 0..world.width() {
            let tile_position = Vec2::new(col as f32, row as f32);
            let tile = world.get(col as i32, row as i32).unwrap_or_else(|| {
                panic!(
                    "Coordinate {},{} not accessible in world of size {},{}",
                    col,
                    row,
                    world.width(),
                    world.height()
                )
            });
            if let Some(index) = tile.atlas_index() {
                spawn_tile(
                    &mut commands,
                    &the_atlas_handle,
                    index,
                    &tile_position,
                    tile,
                    &layer,
                );
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
