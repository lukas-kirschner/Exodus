use bevy::prelude::*;
use bevy::render::camera::{DepthCalculation, ScalingMode};
use bevy::window::WindowMode;
use libexodus::movement::Movement;
use libexodus::player::Player;
use libexodus::world::GameWorld;

/// Number of columns in the game board
static COLUMNS: usize = 24;
/// Number of rows in the game board
static ROWS: usize = 10;
/// Number of pixels between game board tiles
static MARGINS: f32 = 0.00;
/// (1000 / rows) - margins; // Size of a tile, all tiles are square
static TILE_SIZE: f32 = 1.0;
/// Texture size in pixels
static TEXTURE_SIZE: f32 = 32.0;
/// The speed of the player movement
static PLAYER_SPEED: f32 = 0.1;

#[derive(Component)]
struct PlayerComponent {
    pub player: Player,
}

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

fn player_movement(
    mut player_positions: Query<(&mut PlayerComponent, &mut Transform)>,
) {
    for (mut _player, mut transform) in player_positions.iter_mut() {
        // Peek the player's movement queue
        let player: &mut Player = &mut _player.player;
        // let mut transform: Transform = _transform;
        match player.peek_movement_queue() {
            Some(movement) => {
                if movement.velocity.0 > 0. {
                    if transform.translation.x < movement.target.0 {
                        transform.translation.x += movement.velocity.0;
                    } else {
                        transform.translation.x = movement.target.0;
                    }
                } else {
                    if transform.translation.x > movement.target.0 {
                        transform.translation.x += movement.velocity.0;
                    } else {
                        transform.translation.x = movement.target.0;
                    }
                }
                if movement.velocity.1 > 0. {
                    if transform.translation.y < movement.target.1 {
                        transform.translation.y += movement.velocity.1;
                    } else {
                        transform.translation.y = movement.target.1;
                    }
                } else {
                    if transform.translation.y > movement.target.1 {
                        transform.translation.y += movement.velocity.1;
                    } else {
                        transform.translation.y = movement.target.1;
                    }
                }
                if transform.translation.x == movement.target.0 && transform.translation.y == movement.target.1 {
                    // The player has reached the target of the movement, pop from the queue!
                    player.pop_movement_queue();
                }
            }
            None => { continue; }
        }
    }
}

fn setup_player(
    mut commands: Commands,
    rpg_sprite_handles: Res<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut texture_atlas_player = TextureAtlas::from_grid(
        asset_server.load("textures/Tiny_Platform_Quest_Characters.png"),
        Vec2::splat(TEXTURE_SIZE),
        16,
        16,
    );
    let atlas_handle_player = texture_atlases.add(texture_atlas_player);
    let mut player: PlayerComponent = PlayerComponent { player: Player::new() };
    let map_player_position_x = 2.;
    let map_player_position_y = 2.;
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(player.player.atlas_index()),
            texture_atlas: atlas_handle_player.clone(),
            transform: Transform {
                translation: Vec3::new(map_player_position_x, map_player_position_y, 0.),
                scale: Vec3::splat((TILE_SIZE - MARGINS) as f32 / TEXTURE_SIZE),
                ..default()
            },
            ..Default::default()
        })
        .insert(player);
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
            match world.get(col, row).expect(format!("Coordinate {},{} not accessible in world of size {},{}", col, row, world.width(), world.height()).as_str()).atlas_index {
                None => {}
                Some(index) => {
                    commands
                        .spawn_bundle(SpriteSheetBundle {
                            sprite: TextureAtlasSprite::new(index),
                            texture_atlas: atlas_handle.clone(),
                            transform: Transform {
                                translation: tile_position,
                                scale: Vec3::splat((TILE_SIZE - MARGINS) as f32 / TEXTURE_SIZE),
                                ..default()
                            },
                            ..Default::default()
                        });
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
        .init_resource::<RpgSpriteHandles>()
        .add_startup_system(setup_camera)
        .add_startup_system(setup_game_world)
        .add_startup_system(setup_player)
        .add_system(player_movement)
        .add_plugins(DefaultPlugins)
        .run();
}
