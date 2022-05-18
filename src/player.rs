use bevy::prelude::*;
use libexodus::movement::Movement;
use libexodus::player::Player;
use libexodus::world::GameWorld;
use crate::constants::*;
use crate::{MapWrapper, Wall};

#[derive(Component)]
pub struct PlayerComponent {
    pub player: Player,
}

pub fn player_movement(
    mut player_positions: Query<(&mut PlayerComponent, &mut Transform)>,
    mut worldwrapper: ResMut<MapWrapper>,
    time: Res<Time>,
) {
    for (mut _player, mut transform) in player_positions.iter_mut() {
        // Peek the player's movement queue
        let player: &mut Player = &mut _player.player;
        // let mut transform: Transform = _transform;

        while let Some(movement) = player.peek_movement_queue() {
            // Check if the player collides with anything, and remove the movement if that is the case
            let mut collision = false;
            collision = collision || *worldwrapper.world.is_solid(movement.target.0, movement.target.1).unwrap_or(&false);
            if collision {
                println!("Dropped movement to {},{} because it is a solid block", movement.target.0, movement.target.1);
                player.clear_movement_queue(); // On collision, clear all to make sure the player does not move through blocks if more movements are enqueued
            } else {
                break;
            }
        }

        if let Some(movement) = player.peek_movement_queue() {
            let target_x: f32 = movement.target.0 as f32;
            let target_y: f32 = movement.target.1 as f32;
            if movement.velocity.0 > 0. {
                if transform.translation.x < target_x as f32 {
                    transform.translation.x += movement.velocity.0 * time.delta_seconds();
                }
                if transform.translation.x >= target_x {
                    transform.translation.x = movement.target.0 as f32;
                }
            } else {
                if transform.translation.x > target_x {
                    transform.translation.x += movement.velocity.0 * time.delta_seconds();
                }
                if transform.translation.x <= target_x {
                    transform.translation.x = target_x;
                }
            }
            if movement.velocity.1 > 0. {
                if transform.translation.y < target_y {
                    transform.translation.y += movement.velocity.1 * time.delta_seconds();
                }
                if transform.translation.y >= target_y {
                    transform.translation.y = target_y;
                }
            } else {
                if transform.translation.y > target_y {
                    transform.translation.y += movement.velocity.1 * time.delta_seconds();
                }
                if transform.translation.y <= target_y {
                    transform.translation.y = target_y;
                }
            }
            if transform.translation.x == target_x && transform.translation.y == target_y {
                // The player has reached the target of the movement, pop from the queue!
                player.pop_movement_queue();
            }
        }
    }
}


pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    mut worldwrapper: ResMut<MapWrapper>,
) {
    let mut texture_atlas_player = TextureAtlas::from_grid(
        asset_server.load("textures/Tiny_Platform_Quest_Characters.png"),
        Vec2::splat(TEXTURE_SIZE as f32),
        16,
        16,
    );
    let atlas_handle_player = texture_atlases.add(texture_atlas_player);
    let mut player: PlayerComponent = PlayerComponent { player: Player::new() };
    let mut map_player_position_x: usize = 1;
    let mut map_player_position_y: usize = 1;
    (map_player_position_x, map_player_position_y) = worldwrapper.world.player_spawn();
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(player.player.atlas_index()),
            texture_atlas: atlas_handle_player.clone(),
            transform: Transform {
                translation: Vec3::new(map_player_position_x as f32, map_player_position_y as f32, 0.),
                scale: Vec3::splat((TILE_SIZE - MARGINS) as f32 / TEXTURE_SIZE as f32),
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
                let cur_x: i32 = transform.translation.x as i32;
                let cur_y: i32 = transform.translation.y as i32;
                if keyboard_input.just_pressed(KeyCode::Left) {
                    player.push_movement_queue(Movement {
                        velocity: (-vx, 0.),
                        target: (cur_x - 1, cur_y),
                    });
                } else if keyboard_input.just_pressed(KeyCode::Up) {
                    player.push_movement_queue(Movement {
                        velocity: (0., vy),
                        target: (cur_x, cur_y + 1),
                    });
                } else if keyboard_input.just_pressed(KeyCode::Right) {
                    player.push_movement_queue(Movement {
                        velocity: (vx, 0.),
                        target: (cur_x + 1, cur_y),
                    });
                } else if keyboard_input.just_pressed(KeyCode::Down) {
                    player.push_movement_queue(Movement {
                        velocity: (0., -vy),
                        target: (cur_x, cur_y - 1),
                    });
                } else if keyboard_input.just_pressed(KeyCode::Q) {
                    player.push_movement_queue(Movement {
                        velocity: (0., vy),
                        target: (cur_x, cur_y + 1),
                    });
                    player.push_movement_queue(Movement {
                        velocity: (0., vy),
                        target: (cur_x, cur_y + 2),
                    });
                    player.push_movement_queue(Movement {
                        velocity: (-vx, 0.),
                        target: (cur_x - 1, cur_y + 2),
                    });
                } else if keyboard_input.just_pressed(KeyCode::W) {
                    player.push_movement_queue(Movement {
                        velocity: (0., vy),
                        target: (cur_x, cur_y + 1),
                    });
                    player.push_movement_queue(Movement {
                        velocity: (0., vy),
                        target: (cur_x, cur_y + 2),
                    });
                    player.push_movement_queue(Movement {
                        velocity: (vx, 0.),
                        target: (cur_x + 1, cur_y + 2),
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