use bevy::prelude::*;
use libexodus::directions::{FromDirection};
use libexodus::directions::Directions::*;
use libexodus::movement::Movement;
use libexodus::player::Player;
use libexodus::tiles::TileKind;
use crate::{AppState, CurrentMapTextureAtlasHandle, CurrentPlayerTextureAtlasHandle};
use crate::game::constants::*;
use crate::game::scoreboard::Scoreboard;
use crate::game::tilewrapper::{CoinWrapper, MapWrapper, TileWrapper};
use crate::game::world::reset_world;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Playing)
                .with_system(setup_player).after("world").after("reset_score").label("player")
            )
            .add_system_set(SystemSet::on_update(AppState::Playing)
                .with_system(keyboard_controls)
            )
            .add_system_set(SystemSet::on_update(AppState::Playing)
                .with_system(player_movement).label("player_movement")
            )
            .add_system_set(SystemSet::on_update(AppState::Playing)
                .with_system(despawn_dead_player)
            )
        ;
    }
}

#[derive(Component)]
pub struct PlayerComponent {
    pub player: Player,
}

#[derive(Component)]
pub struct DeadPlayerComponent {
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

///
/// Handler that takes care of despawning the dead player and respawning the game world, resetting all counters and objects.
pub fn despawn_dead_player(
    mut commands: Commands,
    mut dead_players: Query<(&mut DeadPlayerComponent, &mut TextureAtlasSprite, &mut Transform, Entity, &Handle<TextureAtlas>)>,
    time: Res<Time>,
    worldwrapper: ResMut<MapWrapper>,
    mut scoreboard: ResMut<Scoreboard>,
    currentMapTextureHandle: Res<CurrentMapTextureAtlasHandle>,
    coin_query: Query<Entity, With<CoinWrapper>>,
    tiles_query: Query<Entity, With<TileWrapper>>,
) {
    for (mut _dead_player, mut sprite, mut transform, entity, texture_atlas_player) in dead_players.iter_mut() {
        let new_a: f32 = sprite.color.a() - (DEAD_PLAYER_DECAY_SPEED * time.delta_seconds());
        if new_a <= 0.0 {
            // The player has fully decayed and can be despawned
            commands.entity(entity).despawn_recursive();
            // Spawn new player and reset scores
            respawn_player(&mut commands, texture_atlas_player.clone(), &worldwrapper);
            scoreboard.reset();
            reset_world(commands, worldwrapper, coin_query, tiles_query, currentMapTextureHandle);
            return;
        }
        sprite.color.set_a(new_a);
        transform.translation.y += DEAD_PLAYER_ASCEND_SPEED * time.delta_seconds();
        transform.scale += Vec3::splat(DEAD_PLAYER_ZOOM_SPEED * time.delta_seconds());
    }
}

pub fn player_movement(
    mut commands: Commands,
    mut player_positions: Query<(&mut PlayerComponent, &mut TextureAtlasSprite, Entity, &mut Transform, &Handle<TextureAtlas>)>,
    worldwrapper: Res<MapWrapper>,
    time: Res<Time>,
) {
    for (mut _player, mut sprite, entity, mut transform, handle) in player_positions.iter_mut() {
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
                    match block.kind() { // Handle special collision events here
                        TileKind::AIR => {}
                        TileKind::SOLID => {}
                        TileKind::DEADLY { .. } => {
                            if block.is_deadly_from(&FromDirection::from(direction)) {
                                commands.entity(entity).despawn_recursive();
                                sprite.index = 80; // Angel texture
                                //TODO trigger GameOverEvent?
                                commands.spawn_bundle(SpriteSheetBundle {
                                    sprite: sprite.clone(),
                                    texture_atlas: handle.clone(),
                                    transform: Transform {
                                        translation: transform.translation,
                                        scale: transform.scale * Vec3::splat(1.2), // Make the angel slightly bigger
                                        ..default()
                                    },
                                    ..Default::default()
                                })
                                    .insert(DeadPlayerComponent { player: player.clone() });
                                // println!("The player should be dead now, after having a deadly encounter with {:?} at {:?}", block, (target_x, target_y));
                            }
                        }
                        TileKind::SPECIAL => {}
                        TileKind::PLAYERSPAWN => {}
                        TileKind::COIN => {}
                        TileKind::LADDER => {
                            // On a ladder, the movement queue is cleared after every step!
                            // This way, the player is unable to jump on a double ladder and ascends instead of jumping.
                            // This case is handled redundantly below in the gravity handler, but we include it here anyways
                            player.clear_movement_queue();
                        }
                    }
                }
                player.pop_movement_queue();
            }
        }

        // Gravity: If Queue is empty and the tile below the player is non-solid and the block the player stands on is not a ladder, add downward movement
        if player.movement_queue_is_empty() {
            if let Some(block) = worldwrapper.world.get(transform.translation.x as i32, transform.translation.y as i32 - 1) {
                if !block.can_collide_from(&FromDirection::FROMNORTH) {
                    player.push_movement_queue(Movement {
                        velocity: (0., -PLAYER_SPEED),
                        target: (transform.translation.x as i32, transform.translation.y as i32 - 1),
                        is_manual: false,
                    });
                }
            }
            if let Some(block) = worldwrapper.world.get(transform.translation.x as i32, transform.translation.y as i32) {
                if let TileKind::LADDER = block.kind() {
                    player.clear_movement_queue(); // We don't want any gravity pulling the player off a ladder
                }
            }
        }
    }
}

fn respawn_player(
    commands: &mut Commands,
    atlas_handle_player: Handle<TextureAtlas>,
    worldwrapper: &ResMut<MapWrapper>,
) {
    let player: PlayerComponent = PlayerComponent { player: Player::new() };
    let (map_player_position_x, map_player_position_y) = worldwrapper.world.player_spawn();
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(player.player.atlas_index()),
            texture_atlas: atlas_handle_player.clone(),
            transform: Transform {
                translation: Vec3::new(map_player_position_x as f32, map_player_position_y as f32, 0.),
                scale: Vec3::splat(TILE_SIZE as f32 / TEXTURE_SIZE as f32),
                ..default()
            },
            ..Default::default()
        })
        .insert(player);
}


pub fn setup_player(
    mut commands: Commands,
    current_texture_atlas: Res<CurrentPlayerTextureAtlasHandle>,
    worldwrapper: ResMut<MapWrapper>,
) {
    respawn_player(&mut commands, (*current_texture_atlas).handle.clone(), &worldwrapper);
}

pub fn keyboard_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut PlayerComponent, &mut TextureAtlasSprite, &Transform)>,
    mut scoreboard: ResMut<Scoreboard>,
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
                        is_manual: true,
                    });
                } else if keyboard_input.just_pressed(KeyCode::Up) {
                    // Jump 3 high
                    player.push_movement_queue(Movement {
                        velocity: (0., vy),
                        target: (cur_x, cur_y + 1),
                        is_manual: true,
                    });
                    player.push_movement_queue(Movement {
                        velocity: (0., vy),
                        target: (cur_x, cur_y + 2),
                        is_manual: true,
                    });
                    player.push_movement_queue(Movement {
                        velocity: (0., vy),
                        target: (cur_x, cur_y + 3),
                        is_manual: true,
                    });
                } else if keyboard_input.just_pressed(KeyCode::Right) {
                    set_player_direction(player, &mut sprite, true);
                    player.push_movement_queue(Movement {
                        velocity: (vx, 0.),
                        target: (cur_x + 1, cur_y),
                        is_manual: true,
                    });
                } else if keyboard_input.just_pressed(KeyCode::Down) {
                    player.push_movement_queue(Movement {
                        velocity: (0., -vy),
                        target: (cur_x, cur_y - 1),
                        is_manual: true,
                    });
                } else if keyboard_input.just_pressed(KeyCode::Q) {
                    set_player_direction(player, &mut sprite, false);
                    player.push_movement_queue(Movement {
                        velocity: (0., vy),
                        target: (cur_x, cur_y + 1),
                        is_manual: true,
                    });
                    player.push_movement_queue(Movement {
                        velocity: (0., vy),
                        target: (cur_x, cur_y + 2),
                        is_manual: true,
                    });
                    player.push_movement_queue(Movement {
                        velocity: (-vx, 0.),
                        target: (cur_x - 1, cur_y + 2),
                        is_manual: true,
                    });
                    player.push_movement_queue(Movement {
                        velocity: (-vx, 0.),
                        target: (cur_x - 2, cur_y + 2),
                        is_manual: true,
                    });
                } else if keyboard_input.just_pressed(KeyCode::W) {
                    set_player_direction(player, &mut sprite, true);
                    player.push_movement_queue(Movement {
                        velocity: (0., vy),
                        target: (cur_x, cur_y + 1),
                        is_manual: true,
                    });
                    player.push_movement_queue(Movement {
                        velocity: (0., vy),
                        target: (cur_x, cur_y + 2),
                        is_manual: true,
                    });
                    player.push_movement_queue(Movement {
                        velocity: (vx, 0.),
                        target: (cur_x + 1, cur_y + 2),
                        is_manual: true,
                    });
                    player.push_movement_queue(Movement {
                        velocity: (vx, 0.),
                        target: (cur_x + 2, cur_y + 2),
                        is_manual: true,
                    });
                }
                if keyboard_input.any_just_pressed(vec![KeyCode::Up, KeyCode::Right, KeyCode::Down, KeyCode::Left, KeyCode::Q, KeyCode::W]) {
                    scoreboard.moves += 1;
                }
            }
            Some(_) => {
                // Do not change anything if there is a pending movement!
                continue;
            }
        }
    }
}