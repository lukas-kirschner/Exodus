use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use libexodus::tiles::{Tile, TileKind};
use libexodus::world::GameWorld;
use crate::{AppState, CurrentMapTextureAtlasHandle};
use crate::game::camera::{destroy_camera, setup_camera};
use crate::game::constants::{TEXTURE_SIZE, TILE_SIZE, WORLD_Z};
use crate::game::pickup_item::insert_wrappers;
use crate::game::tilewrapper::MapWrapper;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Playing)
                .with_system(reset_world).label("world")
            )
            .add_system_set(SystemSet::on_enter(AppState::Playing)
                .with_system(setup_camera).after("world").label("camera")
            )
            .add_system_set(SystemSet::on_exit(AppState::Playing)
                .with_system(destroy_camera)
            )
            // Map Editor needs a world as well:
            .add_system_set(SystemSet::on_enter(AppState::MapEditor)
                .with_system(reset_world).label("world")
            )
            .add_system_set(SystemSet::on_enter(AppState::MapEditor)
                .with_system(setup_camera).after("world").label("camera")
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
    map_texture_atlas: &CurrentMapTextureAtlasHandle,
    atlas_index: usize,
    tile_position: &Vec2,
    tile: &Tile,
) {
    let mut bundle: EntityCommands = commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(atlas_index),
            texture_atlas: map_texture_atlas.handle.clone(),
            transform: Transform {
                translation: tile_position.extend(WORLD_Z),
                scale: Vec3::splat(TILE_SIZE as f32 / TEXTURE_SIZE as f32),
                ..default()
            },
            ..Default::default()
        });
    bundle.insert(WorldTile); // WorldTiles are attached to each world tile, while TileWrappers are only attached to non-interactive world tiles.
    insert_wrappers(tile, &mut bundle);
    insert_door_wrappers(tile, &mut bundle);
}

pub fn setup_game_world(
    mut commands: Commands,
    mut worldwrapper: ResMut<MapWrapper>,
    the_atlas_handle: Res<CurrentMapTextureAtlasHandle>,
) {
    let world: &mut GameWorld = &mut worldwrapper.world;

    for row in 0..world.height() {
        let y = row as f32 * (TILE_SIZE);
        for col in 0..world.width() {
            let x = col as f32 * (TILE_SIZE);
            let tile_position = Vec2::new(
                x as f32,
                y as f32,
            );
            let tile = world.get(col as i32, row as i32).expect(format!("Coordinate {},{} not accessible in world of size {},{}", col, row, world.width(), world.height()).as_str());
            if let Some(index) = tile.atlas_index() {
                spawn_tile(&mut commands, &*the_atlas_handle, index, &tile_position, tile);
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
    the_atlas_handle: Res<CurrentMapTextureAtlasHandle>,
) {
    for entity in tiles_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    worldwrapper.world.reset_game_state();
    setup_game_world(commands, worldwrapper, the_atlas_handle);
}