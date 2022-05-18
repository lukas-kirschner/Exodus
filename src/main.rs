use bevy::prelude::*;
use bevy::render::camera::{DepthCalculation, ScalingMode};
use bevy::window::WindowMode;
use libexodus::movement::Movement;
use libexodus::tiles::TileKind;
use libexodus::world::GameWorld;

mod constants;

use crate::constants::*;

mod player;

use crate::player::*;

mod tilewrapper;

use crate::tilewrapper::*;

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

#[derive(Component)]
struct WorldWrapper {
    world: GameWorld,
}

fn setup_game_world(
    mut commands: Commands,
    rpg_sprite_handles: Res<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {

    // Load Texture Atlas
    let mut texture_atlas = TextureAtlas::from_grid(
        asset_server.load("textures/Tiny_Platform_Quest_Tiles.png"),
        Vec2::splat(TEXTURE_SIZE),
        16,
        16,
    );
    let atlas_handle = texture_atlases.add(texture_atlas);

    let mut world: GameWorld = GameWorld::exampleworld();

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
            match tile.atlas_index {
                None => {}
                Some(index) => {
                    let entity = commands
                        .spawn_bundle(SpriteSheetBundle {
                            sprite: TextureAtlasSprite::new(index),
                            texture_atlas: atlas_handle.clone(),
                            transform: Transform {
                                translation: tile_position,
                                scale: Vec3::splat((TILE_SIZE - MARGINS) as f32 / TEXTURE_SIZE),
                                ..default()
                            },
                            ..Default::default()
                        }).id();
                }
            }
        }
    }
    commands.spawn().insert(
        MapWrapper { world },
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
        .add_startup_system(setup_player)
        .add_system(player_movement)
        .add_system(keyboard_controls)
        .add_plugins(DefaultPlugins)
        .run();
}
