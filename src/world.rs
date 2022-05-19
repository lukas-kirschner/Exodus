use bevy::prelude::*;
use libexodus::tiles::TileKind;
use libexodus::world::GameWorld;
use crate::{CoinWrapper, MapWrapper, TEXTURE_SIZE, TILE_SIZE, TileWrapper};

pub fn setup_game_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut worldwrapper: ResMut<MapWrapper>,
) {

    // Load Texture Atlas
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("textures/Tiny_Platform_Quest_Tiles.png"),
        Vec2::splat(TEXTURE_SIZE as f32),
        16,
        16,
    );
    let atlas_handle = texture_atlases.add(texture_atlas);

    // Load the world
    worldwrapper.set_world(GameWorld::exampleworld());
    let world: &mut GameWorld = &mut worldwrapper.world;

    for row in 0..world.height() {
        let y = row as f32 * (TILE_SIZE);
        for col in 0..world.width() {
            let x = col as f32 * (TILE_SIZE);
            let tile_position = Vec3::new(
                x as f32,
                y as f32,
                0.0,
            );
            let tile = world.get(col as i32, row as i32).expect(format!("Coordinate {},{} not accessible in world of size {},{}", col, row, world.width(), world.height()).as_str());
            if let Some(index) = tile.atlas_index {
                let mut bundle = commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(index),
                        texture_atlas: atlas_handle.clone(),
                        transform: Transform {
                            translation: tile_position,
                            scale: Vec3::splat(TILE_SIZE as f32 / TEXTURE_SIZE as f32),
                            ..default()
                        },
                        ..Default::default()
                    });
                // Insert wrappers so we can despawn the whole map later
                match tile.kind {
                    TileKind::COIN => {
                        bundle.insert(CoinWrapper { coin_value: 1 });
                    }
                    _ => {
                        bundle.insert(TileWrapper {});
                    }
                }
            }
        }
    }
}

///
/// Delete everything world-related and respawn the world, including coins
pub fn reset_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    worldwrapper: ResMut<MapWrapper>,
    coin_query: Query<Entity, With<CoinWrapper>>,
    tiles_query: Query<Entity, With<TileWrapper>>,
) {
    for entity in coin_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in tiles_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    setup_game_world(commands, asset_server, texture_atlases, worldwrapper);
}