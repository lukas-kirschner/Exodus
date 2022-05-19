use bevy::prelude::*;
use libexodus::directions::{FromDirection};
use libexodus::directions::Directions::*;
use libexodus::movement::Movement;
use libexodus::player::Player;
use crate::constants::*;
use crate::{MapWrapper};

#[derive(Component)]
pub struct PlayerComponent {
    pub player: Player,
}

fn set_player_direction(player: &mut Player, sprite: &mut TextureAtlasSprite, right: bool) {
    if right && !player.is_facing_right() {
        player.set_face_right(true);
        sprite.index = player.atlas_index();
    }
    if !right && player.is_facing_right() {
        player.set_face_right(false);
        sprite.index = player.atlas_index();
    }
}

pub fn player_movement(
    mut player_positions: Query<(&mut PlayerComponent, &mut TextureAtlasSprite, &mut Transform)>,
    worldwrapper: Res<MapWrapper>,
    time: Res<Time>,
) {
    for (mut _player, mut sprite, mut transform) in player_positions.iter_mut() {
        // Peek the player's movement queue
        let player: &mut Player = &mut _player.player;
        // let mut transform: Transform = _transform;

        while let Some(movement) = player.peek_movement_queue() {
            // Check if the player collides with anything, and remove the movement if that is the case.
            // For Player movements, only the directions from the movements are used -- The target is discarded and calculated from the direction.
            let (target_x, target_y) = movement.int_target_from_direction(transform.translation.x, transform.translation.y);
            if let Some(block) = worldwrapper.world.get(target_x, target_y) {
                let collision = block.can_collide_from(&FromDirection::from(movement.direction()));
                if collision {
                    println!("Dropped movement {:?} to {},{} because a collision was detected.", movement.direction(), movement.target.0, movement.target.1);
                    player.pop_movement_queue(); // On collision, clear the latest movement
                } else {
                    break;
                }
            }
        }

        if let Some(movement) = player.peek_movement_queue() {
            let (target_x, target_y) = movement.int_target_from_direction(transform.translation.x, transform.translation.y);
            let (target_x, target_y) = (target_x as f32, target_y as f32);
            let velocity_x = movement.velocity.0;
            let velocity_y = movement.velocity.1;
            let direction = movement.direction();
            if direction == EAST {
                // Check player's x direction and change texture accordingly
                set_player_direction(player, &mut sprite, true);

                if transform.translation.x < target_x as f32 {
                    transform.translation.x += velocity_x * time.delta_seconds();
                }
                if transform.translation.x >= target_x {
                    transform.translation.x = target_x as f32;
                }
            } else {
                if velocity_x < 0. { // Do not change direction if no x acceleration is happening
                    // Check player's x direction and change texture accordingly
                    set_player_direction(player, &mut sprite, false);
                }

                if transform.translation.x > target_x {
                    transform.translation.x += velocity_x * time.delta_seconds();
                }
                if transform.translation.x <= target_x {
                    transform.translation.x = target_x;
                }
            }
            if direction == NORTH {
                if transform.translation.y < target_y {
                    transform.translation.y += velocity_y * time.delta_seconds();
                }
                if transform.translation.y >= target_y {
                    transform.translation.y = target_y;
                }
            } else {
                if transform.translation.y > target_y {
                    transform.translation.y += velocity_y * time.delta_seconds();
                }
                if transform.translation.y <= target_y {
                    transform.translation.y = target_y;
                }
            }
            if transform.translation.x == target_x && transform.translation.y == target_y {
                // Check for deadly collision and kill the player, if one has occurred
                if let Some(block) = worldwrapper.world.get(target_x as i32, target_y as i32) {
                    if block.is_deadly_from(&FromDirection::from(direction)) {
                        println!("The player should be dead now, after having a deadly encounter with {:?} at {:?}", block, (target_x, target_y));
                        //TODO
                    }
                }
                // The player has reached the target of the movement, pop from the queue!
                player.pop_movement_queue();
            }
        }

        // Gravity: If Queue is empty and the tile below the player is non-solid, add downward movement
        if player.movement_queue_is_empty() {
            if let Some(block) = worldwrapper.world.get(transform.translation.x as i32, transform.translation.y as i32 - 1) {
                if !block.can_collide_from(&FromDirection::FROMNORTH) {
                    player.push_movement_queue(Movement {
                        velocity: (0., -PLAYER_SPEED),
                        target: (transform.translation.x as i32, transform.translation.y as i32 - 1),
                    });
                }
            }
        }
    }
}


pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    worldwrapper: Res<MapWrapper>,
) {
    let texture_atlas_player = TextureAtlas::from_grid(
        asset_server.load("textures/Tiny_Platform_Quest_Characters.png"),
        Vec2::splat(TEXTURE_SIZE as f32),
        16,
        16,
    );
    let atlas_handle_player = texture_atlases.add(texture_atlas_player);
    let player: PlayerComponent = PlayerComponent { player: Player::new() };
    let (map_player_position_x, map_player_position_y) = worldwrapper.world.player_spawn();
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
    mut players: Query<(&mut PlayerComponent, &mut TextureAtlasSprite, &Transform)>,
) {
    for (mut _player, mut sprite, transform) in players.iter_mut() {
        let player: &mut Player = &mut _player.player;
        match player.peek_movement_queue() {
            None => {
                let vx = PLAYER_SPEED;
                let vy = PLAYER_SPEED;
                // Register the key press
                let cur_x: i32 = transform.translation.x as i32;
                let cur_y: i32 = transform.translation.y as i32;
                if keyboard_input.just_pressed(KeyCode::Left) {
                    set_player_direction(player, &mut sprite, false);
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
                    set_player_direction(player, &mut sprite, true);
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
                    set_player_direction(player, &mut sprite, false);
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
                    set_player_direction(player, &mut sprite, true);
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