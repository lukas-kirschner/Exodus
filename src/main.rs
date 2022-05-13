use std::borrow::Borrow;
use bevy::prelude::*;
use bevy::render::camera::{DepthCalculation, ScalingMode};
use bevy::render::render_resource::{Texture, TextureDescriptor};
use bevy::render::renderer::RenderDevice;
use bevy::window::WindowMode;
use libexodus::tiles;
use libexodus::tiles::{SPIKES, WALL};
use libexodus::world;
use libexodus::world::GameWorld;

// We use https://opengameart.org/content/8x8-resource-pack and https://opengameart.org/content/tiny-platform-quest-sprites free textures
// TODO !!! Textures are CC-BY-SA 3.0

#[derive(Default)]
struct RpgSpriteHandles {
    handles: Vec<HandleUntyped>,
}

#[derive(Component)]
struct TileSolid;

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
    rpg_sprite_handles: Res<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let columns = 24; // Number of columns in the game board
    let rows = 10; // Number of rows in the game board
    let margins = 0.00; // Number of pixels between game board tiles
    let tile_size = 1.0;//(1000 / rows) - margins; // Size of a tile, all tiles are square
    let texture_size = 32.0; // Texture size in pixels

    // Load Texture Atlas
    let mut texture_atlas = TextureAtlas::from_grid(
        asset_server.load("textures/Tiny_Platform_Quest_Tiles.png"),
        Vec2::splat(texture_size),
        16,
        16,
    );
    let solid_index: usize = 58;
    let atlas_handle = texture_atlases.add(texture_atlas);

    let mut world: GameWorld = GameWorld::new(columns, rows);
    world
        .set(0, 0, &WALL)
        .set(1, 0, &WALL)
        .set(2, 0, &WALL)
        .set(2, 1, &WALL)
        .set(2, 2, &SPIKES)
    ;

    for row in 0..world.height() {
        let y = row as f32 * (tile_size);
        for col in 0..world.width() {
            let x = col as f32 * (tile_size);
            let tile_position = Vec3::new(
                x as f32,
                y as f32,
                0.0,
            );
            match world.get(col, row).expect(format!("Coordinate {},{} not accessible in world of size {},{}", col, row, world.width(), world.height()).as_str()).atlas_index {
                None => {}
                Some(index) => {
                    commands
                        .spawn_bundle(SpriteSheetBundle {
                            sprite: TextureAtlasSprite::new(index),
                            texture_atlas: atlas_handle.clone(),
                            transform: Transform {
                                translation: tile_position,
                                scale: Vec3::splat((tile_size - margins) as f32 / texture_size),
                                ..default()
                            },
                            ..Default::default()
                        });
                }
            }
        }
    }
    commands
        .spawn_bundle(
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(1.0)),
                    ..default()
                },
                ..default()
            }
        );
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
        .init_resource::<RpgSpriteHandles>()
        .add_startup_system(setup_camera)
        .add_startup_system(setup_game_world)
        .add_plugins(DefaultPlugins)
        .run();
}
