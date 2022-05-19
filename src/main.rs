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
use crate::ui::{scoreboard_ui_system, setup_game_ui};

mod scoreboard;

mod ui;

// We use https://opengameart.org/content/8x8-resource-pack and https://opengameart.org/content/tiny-platform-quest-sprites free textures
// TODO !!! Textures are CC-BY-SA 3.0
// TODO There is a bug in Bevy that causes adjacent textures from the atlas to leak through due to precision errors: https://github.com/bevyengine/bevy/issues/1949

fn setup_camera(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    map: Res<MapWrapper>,
) {
    // Scale the camera, such that the world exactly fits into the viewport. At the top and bottom,
    // we leave at least one world tile of space free for UI elements, which we also scale
    // exactly to the height of one tile.
    let map_width_pixels: usize = TEXTURE_SIZE * map.world.width();
    let map_height_pixels_plus_ui: usize = TEXTURE_SIZE * (map.world.height() + 2); // 2 tiles for UI elements
    let window_height_pixels: usize = window.height as usize;
    let window_width_pixels: usize = window.width as usize;
    let window_ratio: f32 = window_width_pixels as f32 / window_height_pixels as f32;
    let map_ratio: f32 = map_width_pixels as f32 / map_height_pixels_plus_ui as f32;
    let camera_scale = if window_ratio < map_ratio {
        window_width_pixels as f32 / map_width_pixels as f32
    } else {
        window_height_pixels as f32 / map_height_pixels_plus_ui as f32
    };

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection = OrthographicProjection {
        far: 1000.0,
        depth_calculation: DepthCalculation::ZDifference,
        scaling_mode: ScalingMode::WindowSize,
        ..default()
    };
    camera.transform.scale = Vec3::splat(1. / (camera_scale * TEXTURE_SIZE as f32));
    // We need to subtract 0.5 to account for the fact that tiles are placed in the middle of each coordinate
    camera.transform.translation = Vec3::new((map.world.width() as f32 / 2.) - 0.5, (map.world.height() as f32 / 2.) - 0.5, 0.);
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
                            scale: Vec3::splat(TILE_SIZE as f32 / TEXTURE_SIZE as f32),
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
        .add_startup_system(setup_game_ui.after("world").after("player").after("camera").label("ui"))
        .add_system(player_movement.label("player_movement"))
        .add_system(keyboard_controls)
        .add_system(coin_collision.after("player_movement"))
        .add_system(scoreboard_ui_system)
        .run();
}
