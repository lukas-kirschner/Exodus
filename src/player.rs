use bevy::prelude::*;
use libexodus::movement::Movement;
use libexodus::player::Player;
use crate::constants::*;

#[derive(Component)]
pub struct PlayerComponent {
    pub player: Player,
}

#[derive(Default)]
pub struct RpgSpriteHandles {
    handles: Vec<HandleUntyped>,
}

pub fn player_movement(
    mut player_positions: Query<(&mut PlayerComponent, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut _player, mut transform) in player_positions.iter_mut() {
        // Peek the player's movement queue
        let player: &mut Player = &mut _player.player;
        // let mut transform: Transform = _transform;
        match player.peek_movement_queue() {
            Some(movement) => {
                if movement.velocity.0 > 0. {
                    if transform.translation.x < movement.target.0 {
                        transform.translation.x += movement.velocity.0 * time.delta_seconds();
                    }
                    if transform.translation.x >= movement.target.0 {
                        transform.translation.x = movement.target.0;
                    }
                } else {
                    if transform.translation.x > movement.target.0 {
                        transform.translation.x += movement.velocity.0 * time.delta_seconds();
                    }
                    if transform.translation.x <= movement.target.0 {
                        transform.translation.x = movement.target.0;
                    }
                }
                if movement.velocity.1 > 0. {
                    if transform.translation.y < movement.target.1 {
                        transform.translation.y += movement.velocity.1 * time.delta_seconds();
                    }
                    if transform.translation.y >= movement.target.1 {
                        transform.translation.y = movement.target.1;
                    }
                } else {
                    if transform.translation.y > movement.target.1 {
                        transform.translation.y += movement.velocity.1 * time.delta_seconds();
                    }
                    if transform.translation.y <= movement.target.1 {
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

pub fn setup_player(
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

pub fn keyboard_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut PlayerComponent, &Transform)>,
) {
    for (mut _player, transform) in players.iter_mut() {
        let player: &mut Player = &mut _player.player;
        match player.peek_movement_queue() {
            None => {
                let vx = PLAYER_SPEED;
                let vy = PLAYER_SPEED;
                // Register the key press
                if keyboard_input.pressed(KeyCode::Left) {
                    player.push_movement_queue(Movement {
                        velocity: (-vx, 0.),
                        target: (transform.translation.x - 1., transform.translation.y),
                    });
                } else if keyboard_input.pressed(KeyCode::Up) {
                    player.push_movement_queue(Movement {
                        velocity: (0., vy),
                        target: (transform.translation.x, transform.translation.y + 1.),
                    });
                } else if keyboard_input.pressed(KeyCode::Right) {
                    player.push_movement_queue(Movement {
                        velocity: (vx, 0.),
                        target: (transform.translation.x + 1., transform.translation.y),
                    });
                } else if keyboard_input.pressed(KeyCode::Down) {
                    player.push_movement_queue(Movement {
                        velocity: (0., -vy),
                        target: (transform.translation.x, transform.translation.y - 1.),
                    });
                }
            }
            Some(_) => {
                // Do not change anything if there is a pending movement!
                continue;
            }
        }
    }
}