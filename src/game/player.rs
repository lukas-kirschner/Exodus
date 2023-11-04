use crate::game::constants::*;
use crate::game::scoreboard::Scoreboard;
use crate::game::tilewrapper::{GameOverState, MapWrapper};
use crate::game::world::DoorWrapper;
use crate::{AppLabels, AppState, GameConfig, TilesetManager, LAYER_ID};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use libexodus::directions::Directions::*;
use libexodus::directions::FromDirection;
use libexodus::movement::Movement;
use libexodus::player::Player;
use libexodus::tiles::{Tile, TileKind};
use libexodus::world::GameWorld;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Playing),
            setup_player
                .after(AppLabels::World)
                .after(AppLabels::ResetScore)
                .in_set(AppLabels::Player),
        )
        .add_systems(
            Update,
            keyboard_controls.run_if(in_state(AppState::Playing)),
        )
        .add_systems(
            Update,
            player_movement
                .run_if(in_state(AppState::Playing))
                .in_set(AppLabels::PlayerMovement),
        )
        .add_systems(
            Update,
            player_gravity
                .in_set(AppLabels::Gravity)
                .run_if(in_state(AppState::Playing))
                .after(AppLabels::PlayerMovement),
        )
        .add_systems(
            Update,
            despawn_dead_player
                .run_if(in_state(AppState::Playing))
                .in_set(AppLabels::GameOverTrigger),
        )
        .add_systems(
            Update,
            despawn_exited_player
                .run_if(in_state(AppState::Playing))
                .in_set(AppLabels::GameOverTrigger),
        )
        .add_systems(OnExit(AppState::Playing), despawn_players)
        .add_systems(
            Update,
            game_over_event_listener
                .run_if(in_state(AppState::Playing))
                .in_set(AppLabels::GameOverTrigger),
        )
        .add_event::<GameOverEvent>();
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

#[derive(Component)]
pub struct ExitingPlayerComponent {
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
fn despawn_dead_player(
    mut commands: Commands,
    mut dead_players: Query<
        (&mut TextureAtlasSprite, &mut Transform, Entity),
        With<DeadPlayerComponent>,
    >,
    config: Res<GameConfig>,
    time: Res<Time>,
    mut event_writer: EventWriter<GameOverEvent>,
) {
    let texture_size = config.texture_size();
    for (mut sprite, mut transform, entity) in dead_players.iter_mut() {
        let new_a: f32 = sprite.color.a() - (DEAD_PLAYER_DECAY_SPEED * time.delta_seconds());
        if new_a <= 0.0 {
            // The player has fully decayed and can be despawned.
            commands.entity(entity).despawn_recursive();
            event_writer.send(GameOverEvent {
                state: GameOverState::Lost,
            });
            return;
        }
        sprite.color.set_a(new_a);
        transform.translation.y += DEAD_PLAYER_ASCEND_SPEED * texture_size * time.delta_seconds();
        transform.scale +=
            Vec3::splat(DEAD_PLAYER_ZOOM_SPEED * texture_size * time.delta_seconds());
    }
}

/// Event that is triggered when a game is won or lost
#[derive(Event)]
pub struct GameOverEvent {
    pub state: GameOverState,
}

///
/// Handler that takes care of despawning the player after he exited the game through an exit
pub fn despawn_exited_player(
    mut commands: Commands,
    mut exited_players: Query<
        (&mut TextureAtlasSprite, &mut Transform, Entity),
        With<ExitingPlayerComponent>,
    >,
    config: Res<GameConfig>,
    time: Res<Time>,
    mut event_writer: EventWriter<GameOverEvent>,
    scoreboard: Res<Scoreboard>,
) {
    let texture_size = config.texture_size();
    for (mut sprite, mut transform, entity) in exited_players.iter_mut() {
        let new_a: f32 = sprite.color.a() - (EXITED_PLAYER_DECAY_SPEED * time.delta_seconds());
        if new_a <= 0.0 {
            // The player has fully decayed and can be despawned.
            commands.entity(entity).despawn_recursive();
            event_writer.send(GameOverEvent {
                state: GameOverState::Won {
                    score: (*scoreboard).clone(),
                },
            });
            return;
        }
        sprite.color.set_a(new_a);
        transform.translation.y += EXITED_PLAYER_ASCEND_SPEED * texture_size * time.delta_seconds();
        transform.scale +=
            Vec3::splat(EXITED_PLAYER_ZOOM_SPEED * texture_size * time.delta_seconds());
    }
}

/// Open the door at the new player position and return true if the door has been opened.
/// Fail, if the player does not have enough keys
fn door_opened(
    doors: &mut Query<(Entity, &Transform, &mut TextureAtlasSprite), With<DoorWrapper>>,
    commands: &mut Commands,
    target_x_coord: i32,
    target_y_coord: i32,
    config: &GameConfig,
    world: &mut GameWorld,
    scoreboard: &mut Scoreboard,
) -> bool {
    let (target_x_px, target_y_px) = (
        target_x_coord * config.texture_size() as i32,
        target_y_coord * config.texture_size() as i32,
    );
    if !world
        .get(target_x_coord, target_y_coord)
        .map(|t| t.kind() == TileKind::DOOR)
        .unwrap_or(false)
    {
        return false;
    }
    if scoreboard.keys > 0 {
        for (entity, transform, mut texture_atlas_sprite) in doors.iter_mut() {
            if transform.translation.x == target_x_px as f32
                && transform.translation.y == target_y_px as f32
            {
                // Found the door. Despawn it and change its texture to an open door
                commands.entity(entity).remove::<DoorWrapper>();
                world.set(
                    target_x_coord as usize,
                    target_y_coord as usize,
                    Tile::OPENDOOR,
                );
                texture_atlas_sprite.index = Tile::OPENDOOR.atlas_index().unwrap();
                scoreboard.keys -= 1;
                return true;
            }
        }
        panic!(
            "There was no DoorWrapper spawned for the door at {},{}",
            target_x_coord, target_y_coord
        );
    }
    false
}

pub fn player_movement(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut player_positions: Query<
        (
            &mut PlayerComponent,
            &mut TextureAtlasSprite,
            Entity,
            &mut Transform,
            &Handle<TextureAtlas>,
        ),
        Without<DoorWrapper>,
    >,
    mut doors: Query<(Entity, &Transform, &mut TextureAtlasSprite), With<DoorWrapper>>,
    mut worldwrapper: ResMut<MapWrapper>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    for (mut _player, mut sprite, entity, mut transform, handle) in player_positions.iter_mut() {
        // Peek the player's movement queue
        let player: &mut Player = &mut _player.player;
        // let mut transform: Transform = _transform;

        while let Some(movement) = player.peek_movement_queue() {
            // Check if the player collides with anything, and remove the movement if that is the case.
            // For Player movements, only the directions from the movements are used -- The target is discarded and calculated from the direction.
            let (target_x_coord, target_y_coord) = movement.int_target_from_direction(
                transform.translation.x / (config.texture_size()),
                transform.translation.y / (config.texture_size()),
            );
            // Check if the player collides with map boundaries
            if target_x_coord < 0
                || target_y_coord < 0
                || target_x_coord >= (worldwrapper.world.width()) as i32
                || target_y_coord >= (worldwrapper.world.height()) as i32
            {
                debug!("Dropped Movement {:?} to {},{}, because its target lies outside of the map boundaries!", movement.direction(), movement.target.0, movement.target.1);
                player.pop_movement_queue();
                continue;
            }
            if let Some(block) = worldwrapper.world.get(target_x_coord, target_y_coord) {
                let collision = block.can_collide_from(&FromDirection::from(movement.direction()));
                if collision {
                    if !door_opened(
                        &mut doors,
                        &mut commands,
                        target_x_coord,
                        target_y_coord,
                        &config,
                        &mut worldwrapper.world,
                        &mut scoreboard,
                    ) {
                        debug!(
                            "Dropped movement {:?} to {},{} because a collision was detected.",
                            movement.direction(),
                            movement.target.0,
                            movement.target.1
                        );
                        player.pop_movement_queue(); // On collision, clear the latest movement
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        if let Some(movement) = player.peek_movement_queue() {
            let (target_x_coord, target_y_coord) = movement.int_target_from_direction(
                transform.translation.x / (config.texture_size()),
                transform.translation.y / (config.texture_size()),
            );
            let (target_x_px, target_y_px) = (
                (target_x_coord * config.texture_size() as i32) as f32,
                (target_y_coord * config.texture_size() as i32) as f32,
            );
            let velocity_x = movement.velocity.0;
            let velocity_y = movement.velocity.1;
            let direction = movement.direction();
            if direction == EAST {
                // Check player's x direction and change texture accordingly
                set_player_direction(player, &mut sprite, true);

                if transform.translation.x < target_x_px {
                    transform.translation.x += velocity_x * time.delta_seconds();
                }
                if transform.translation.x >= target_x_px {
                    transform.translation.x = target_x_px;
                }
            } else {
                if velocity_x < 0. {
                    // Do not change direction if no x acceleration is happening
                    // Check player's x direction and change texture accordingly
                    set_player_direction(player, &mut sprite, false);
                }

                if transform.translation.x > target_x_px {
                    transform.translation.x += velocity_x * time.delta_seconds();
                }
                if transform.translation.x <= target_x_px {
                    transform.translation.x = target_x_px;
                }
            }
            if direction == NORTH {
                if transform.translation.y < target_y_px {
                    transform.translation.y += velocity_y * time.delta_seconds();
                }
                if transform.translation.y >= target_y_px {
                    transform.translation.y = target_y_px;
                }
            } else {
                if transform.translation.y > target_y_px {
                    transform.translation.y += velocity_y * time.delta_seconds();
                }
                if transform.translation.y <= target_y_px {
                    transform.translation.y = target_y_px;
                }
            }
            if transform.translation.x == target_x_px && transform.translation.y == target_y_px {
                // Check for events that occur when the player is already on the same tile as the block
                if let Some(block) = worldwrapper.world.get(target_x_coord, target_y_coord) {
                    match block.kind() {
                        // Handle special collision events here
                        TileKind::AIR => {},
                        TileKind::SOLID => {},
                        TileKind::DEADLY { .. } => {
                            if block.is_deadly_from(&FromDirection::from(direction)) {
                                commands.entity(entity).despawn_recursive();
                                sprite.index = 222; // Angel texture
                                let layer = RenderLayers::layer(LAYER_ID);
                                commands.spawn((
                                    SpriteSheetBundle {
                                        sprite: sprite.clone(),
                                        texture_atlas: handle.clone(),
                                        transform: Transform {
                                            translation: transform.translation,
                                            scale: transform.scale * Vec3::splat(1.2),
                                            ..default()
                                        },
                                        ..Default::default()
                                    },
                                    DeadPlayerComponent {
                                        player: player.clone(),
                                    },
                                    layer,
                                ));
                            }
                        },
                        TileKind::SPECIAL { interaction: _ } => {},
                        TileKind::PLAYERSPAWN => {},
                        TileKind::COIN => {},
                        TileKind::LADDER => {
                            // On a ladder, the movement queue is cleared after every step!
                            // This way, the player is unable to jump on a double ladder and ascends instead of jumping.
                            // For players with empty movement queues, this case is handled below as well.
                            player.clear_movement_queue();
                        },
                        TileKind::KEY => {},
                        TileKind::DOOR => {},
                        TileKind::COLLECTIBLE => {},
                        TileKind::EXIT => {
                            commands.entity(entity).despawn_recursive();
                            sprite.index = 247; // Player turning their back to the camera
                            let layer = RenderLayers::layer(LAYER_ID);
                            commands.spawn((
                                SpriteSheetBundle {
                                    sprite: sprite.clone(),
                                    texture_atlas: handle.clone(),
                                    transform: *transform,
                                    ..default()
                                },
                                ExitingPlayerComponent {
                                    player: player.clone(),
                                },
                                layer,
                            ));
                        },
                    }
                }
                player.pop_movement_queue();
            }
        }
    }
}

fn player_gravity(
    mut player_positions: Query<(&mut PlayerComponent, &Transform), Without<DoorWrapper>>,
    worldwrapper: ResMut<MapWrapper>,
    config: Res<GameConfig>,
) {
    for (mut _player, transform) in player_positions.iter_mut() {
        // Peek the player's movement queue
        let player: &mut Player = &mut _player.player;
        // Gravity: If Queue is empty and the tile below the player is non-solid and the block the player stands on is not a ladder, add downward movement
        if player.movement_queue_is_empty() {
            let current_x_coord = (transform.translation.x / config.texture_size()) as i32;
            let current_y_coord = (transform.translation.y / config.texture_size()) as i32;
            if let Some(block) = worldwrapper.world.get(current_x_coord, current_y_coord - 1) {
                if !block.can_collide_from(&FromDirection::FROMNORTH) {
                    player.push_movement_queue(Movement {
                        velocity: (0., -(PLAYER_SPEED_ * (config.texture_size()))),
                        target: (current_x_coord, current_y_coord - 1),
                        is_manual: false,
                    });
                }
            }
            if let Some(block) = worldwrapper.world.get(current_x_coord, current_y_coord) {
                if let TileKind::LADDER = block.kind() {
                    player.clear_movement_queue(); // We don't want any gravity pulling the player off a ladder
                }
            }
            if let Some(block) = worldwrapper.world.get(current_x_coord, current_y_coord - 1) {
                if let TileKind::LADDER = block.kind() {
                    player.clear_movement_queue(); // Players shall be able to stand on ladders
                }
            }
        }
    }
}

fn respawn_player(
    commands: &mut Commands,
    atlas_handle_player: &TilesetManager,
    world: &GameWorld,
) {
    let player: PlayerComponent = PlayerComponent {
        player: Player::new(),
    };
    let (map_player_position_x, map_player_position_y) = world.player_spawn();
    let layer = RenderLayers::layer(LAYER_ID);
    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(player.player.atlas_index()),
            texture_atlas: atlas_handle_player.current_handle(),
            transform: Transform {
                translation: Vec3::new(
                    (map_player_position_x * atlas_handle_player.current_tileset().texture_size())
                        as f32,
                    (map_player_position_y * atlas_handle_player.current_tileset().texture_size())
                        as f32,
                    PLAYER_Z,
                ),
                ..default()
            },
            ..Default::default()
        },
        player,
        layer,
    ));
}

pub fn despawn_players(mut commands: Commands, players: Query<Entity, With<PlayerComponent>>) {
    for entity in players.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn setup_player(
    mut commands: Commands,
    current_texture_atlas: Res<TilesetManager>,
    world: ResMut<MapWrapper>,
) {
    respawn_player(&mut commands, &current_texture_atlas, &world.world);
}

pub fn keyboard_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut PlayerComponent, &mut TextureAtlasSprite, &Transform)>,
    mut scoreboard: ResMut<Scoreboard>,
    config: Res<GameConfig>,
    map: Res<MapWrapper>,
) {
    for (mut _player, mut sprite, transform) in players.iter_mut() {
        let player: &mut Player = &mut _player.player;
        match player.peek_movement_queue() {
            None => {
                let vx = PLAYER_SPEED_ * (config.texture_size());
                let vy = PLAYER_SPEED_ * (config.texture_size());
                // Register the key press
                let cur_x: i32 = (transform.translation.x / (config.texture_size())) as i32;
                let cur_y: i32 = (transform.translation.y / (config.texture_size())) as i32;
                if keyboard_input.just_pressed(KeyCode::Left) {
                    set_player_direction(player, &mut sprite, false);
                    player.push_movement_queue(Movement {
                        velocity: (-vx, 0.),
                        target: (cur_x - 1, cur_y),
                        is_manual: true,
                    });
                } else if keyboard_input.just_pressed(KeyCode::Up) {
                    let tile = map.world.get(cur_x, cur_y).unwrap();
                    player.push_movement_queue(Movement {
                        velocity: (0., vy),
                        target: (cur_x, cur_y + 1),
                        is_manual: true,
                    });
                    // We need to differentiate between ladder and NOT ladder here, this allows us to jump only 1 high on the end of a ladder
                    if tile.kind() != TileKind::LADDER {
                        // Jump 3 high.
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
                    }
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
                if keyboard_input.any_just_pressed(vec![
                    KeyCode::Up,
                    KeyCode::Right,
                    KeyCode::Down,
                    KeyCode::Left,
                    KeyCode::Q,
                    KeyCode::W,
                ]) {
                    scoreboard.moves += 1;
                }
            },
            Some(_) => {
                // Do not change anything if there is a pending movement!
                continue;
            },
        }
    }
}

fn game_over_event_listener(
    mut reader: EventReader<GameOverEvent>,
    mut state: ResMut<NextState<AppState>>,
    mut commands: Commands,
) {
    if let Some(event) = reader.iter().next() {
        commands.insert_resource(event.state.clone());
        state.set(AppState::GameOverScreen);
    }
}
