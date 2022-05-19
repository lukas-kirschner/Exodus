use bevy::prelude::*;
use bevy::render::camera::{DepthCalculation, ScalingMode};
use bevy::window::WindowMode;
use libexodus::tiles::TileKind;
use libexodus::world::GameWorld;

mod constants;

use crate::constants::*;

mod player;

use crate::player::*;
use crate::scoreboard::Scoreboard;

mod tilewrapper;

use crate::tilewrapper::*;

mod scoreboard;

// We use https://opengameart.org/content/8x8-resource-pack and https://opengameart.org/content/tiny-platform-quest-sprites free textures
// TODO !!! Textures are CC-BY-SA 3.0

fn setup_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection = OrthographicProjection {
        far: 1000.0,
        depth_calculation: DepthCalculation::ZDifference,
        scaling_mode: ScalingMode::FixedHorizontal,
        ..default()
    };
    camera.transform.scale = Vec3::splat(1000.0 / 24.0);
    camera.transform.translation = Vec3::new(12., 5., 0.);
    commands.spawn_bundle(camera);
}

fn setup_game_world(
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
                            scale: Vec3::splat((TILE_SIZE - MARGINS) as f32 / TEXTURE_SIZE as f32),
                            ..default()
                        },
                        ..Default::default()
                    });
                match tile.kind {
                    TileKind::COIN => {
                        bundle.insert(CoinWrapper { coin_value: 1 });
                    }
                    _ => {}
                }
            }
        }
    }
}


fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Exodus".to_string(),
            resizable: false,
            width: 1000.,
            height: 500.,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<MapWrapper>()
        .init_resource::<Scoreboard>()
        .add_startup_system(setup_game_world.label("world"))
        .add_startup_system(setup_camera.after("world").label("camera"))
        .add_startup_system(setup_player.after("world").label("player"))
        .add_system(player_movement.label("player_movement"))
        .add_system(keyboard_controls)
        .add_system(coin_collision.after("player_movement"))
        .run();
}
