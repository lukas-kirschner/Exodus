use crate::animation::animated_action_sprite::{AnimatedActionSprite, AnimatedSpriteAction};
use crate::game::constants::*;
use crate::game::scoreboard::{GameOverEvent, GameOverState, Scoreboard};
use crate::game::tilewrapper::MapWrapper;
use crate::game::vending_machine::VendingMachineTriggered;
use crate::game::world::DoorWrapper;
use crate::{AppLabels, AppState, GameConfig, TilesetManager, LAYER_ID};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use libexodus::directions::Directions::*;
use libexodus::directions::FromDirection;
use libexodus::movement::Movement;
use libexodus::player::Player;
use libexodus::tiles::{InteractionKind, Tile, TileKind, ANGEL_SPRITE, EXITING_PLAYER_SPRITE};
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

fn set_player_direction(player: &mut Player, sprite: &mut Sprite, right: bool) {
    if right && !player.is_facing_right() {
        player.set_face_right(true);
        sprite
            .texture_atlas
            .as_mut()
            .map(|a| a.index = player.atlas_index());
    }
    if !right && player.is_facing_right() {
        player.set_face_right(false);
        sprite
            .texture_atlas
            .as_mut()
            .map(|a| a.index = player.atlas_index());
    }
}

/// Resource that determines what screen to return to when a GameOver event is triggered
/// or ESC is pressed
#[derive(Resource)]
pub struct ReturnTo(pub AppState);

/// Handle events that occur when the player would normally collide with the target tile.
/// This happens e.g. with Doors and Vending Machines.
/// For vending machines, the vending machine will be triggered if the player approached it from the right direction.
/// For doors, open the door at the new player position and return true if the door has been opened.
/// Fail, if the player does not have enough keys.
/// If true is returned, the player does not collide with the tile, e.g., because the door has been opened.
fn handle_collision_interaction(
    doors: &mut Query<(Entity, &Transform, &mut Sprite), With<DoorWrapper>>,
    commands: &mut Commands,
    target_x_coord: i32,
    target_y_coord: i32,
    config: &GameConfig,
    world: &mut GameWorld,
    scoreboard: &mut Scoreboard,
    atlas_handle: &TilesetManager,
    movement: &Movement,
    vending_machine_trigger: &mut EventWriter<VendingMachineTriggered>,
) -> bool {
    let (target_x_px, target_y_px) = (
        target_x_coord * config.texture_size() as i32,
        target_y_coord * config.texture_size() as i32,
    );
    let tile = world.get(target_x_coord, target_y_coord);
    if tile.is_none() {
        return false;
    }
    match tile.unwrap().kind() {
        TileKind::AIR => false,
        TileKind::SOLID => false,
        TileKind::SOLIDINTERACTABLE { from, kind } => {
            let from_direction = FromDirection::from(movement.direction());
            if !from.iter().any(|fromdir| *fromdir == from_direction) {
                // debug!(
                //     "A Collision with an interactable solid from {:?} was detected",
                //     FromDirection::from(movement.direction())
                // );
                return false;
            }
            // debug!("A vending machine might be triggered, if the movement was manual");
            if movement.is_manual {
                match kind {
                    InteractionKind::LaunchMap { .. } => {},
                    InteractionKind::TeleportTo { .. } => {},
                    InteractionKind::VendingMachine => {
                        // Trigger Vending Machine
                        vending_machine_trigger.send(VendingMachineTriggered);
                    },
                }
            }
            false
        },
        TileKind::DEADLY { .. } => false,
        TileKind::SPECIAL { .. } => false,
        TileKind::PLAYERSPAWN => false,
        TileKind::COLLECTIBLE { .. } => false,
        TileKind::DOOR => {
            if scoreboard.keys > 0 {
                for (entity, transform, mut sprite) in doors.iter_mut() {
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
                        sprite
                            .texture_atlas
                            .as_mut()
                            .map(|a| a.index = Tile::OPENDOOR.atlas_index().unwrap());
                        scoreboard.keys -= 1;
                        // Spawn a "Key Used" Animation:
                        commands.spawn((
                            Sprite::from_atlas_image(
                                sprite.image.clone(),
                                TextureAtlas {
                                    layout: atlas_handle.current_atlas_handle(),
                                    index: Tile::KEY.atlas_index().unwrap(),
                                },
                            ),
                            Transform::from_translation(Vec3::new(
                                target_x_px as f32,
                                target_y_px as f32,
                                PLAYER_Z - 0.1,
                            )),
                            AnimatedActionSprite::from_ascend_and_zoom(
                                KEY_OPEN_ANIMATION_DECAY_SPEED,
                                KEY_OPEN_ANIMATION_ASCEND_SPEED,
                                KEY_OPEN_ANIMATION_ZOOM_SPEED,
                                AnimatedSpriteAction::None,
                            ), // WorldTiles are attached to each world tile, while TileWrappers are additionally attached to non-interactive world tiles.
                            RenderLayers::layer(LAYER_ID),
                        ));
                        return true;
                    }
                }
                panic!(
                    "There was no DoorWrapper spawned for the door at {},{}",
                    target_x_coord, target_y_coord
                );
            }
            false
        },
        TileKind::LADDER => false,
        TileKind::EXIT => false,
    }
}

pub fn player_movement(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut player_positions: Query<
        (&mut PlayerComponent, &mut Sprite, Entity, &mut Transform),
        Without<DoorWrapper>,
    >,
    mut doors: Query<(Entity, &Transform, &mut Sprite), With<DoorWrapper>>,
    mut worldwrapper: ResMut<MapWrapper>,
    config: Res<GameConfig>,
    time: Res<Time>,
    atlas_handle: Res<TilesetManager>,
    mut vending_machine_trigger: EventWriter<VendingMachineTriggered>,
) {
    for (mut _player, mut sprite, player_entity, mut transform) in player_positions.iter_mut() {
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
                    if !handle_collision_interaction(
                        &mut doors,
                        &mut commands,
                        target_x_coord,
                        target_y_coord,
                        &config,
                        &mut worldwrapper.world,
                        &mut scoreboard,
                        atlas_handle.as_ref(),
                        movement,
                        &mut vending_machine_trigger,
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
                set_player_direction(player, sprite.as_mut(), true);

                if transform.translation.x < target_x_px {
                    transform.translation.x += velocity_x * time.delta_secs();
                }
                if transform.translation.x >= target_x_px {
                    transform.translation.x = target_x_px;
                }
            } else {
                if velocity_x < 0. {
                    // Do not change direction if no x acceleration is happening
                    // Check player's x direction and change texture accordingly
                    set_player_direction(player, sprite.as_mut(), false);
                }

                if transform.translation.x > target_x_px {
                    transform.translation.x += velocity_x * time.delta_secs();
                }
                if transform.translation.x <= target_x_px {
                    transform.translation.x = target_x_px;
                }
            }
            if direction == NORTH {
                if transform.translation.y < target_y_px {
                    transform.translation.y += velocity_y * time.delta_secs();
                }
                if transform.translation.y >= target_y_px {
                    transform.translation.y = target_y_px;
                }
            } else {
                if transform.translation.y > target_y_px {
                    transform.translation.y += velocity_y * time.delta_secs();
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
                        TileKind::SOLIDINTERACTABLE { .. } => {},
                        TileKind::DEADLY { .. } => {
                            if block.is_deadly_from(&FromDirection::from(direction)) {
                                commands.entity(player_entity).despawn_recursive();
                                sprite
                                    .texture_atlas
                                    .as_mut()
                                    .map(|a| a.index = ANGEL_SPRITE);
                                let layer = RenderLayers::layer(LAYER_ID);
                                commands.spawn((
                                    Sprite::from_atlas_image(
                                        sprite.image.clone(),
                                        sprite.texture_atlas.clone().unwrap(),
                                    ),
                                    Transform {
                                        translation: transform.translation,
                                        scale: transform.scale * Vec3::splat(1.2),
                                        ..default()
                                    },
                                    AnimatedActionSprite::from_ascend_and_zoom(
                                        DEAD_PLAYER_DECAY_SPEED,
                                        DEAD_PLAYER_ASCEND_SPEED,
                                        DEAD_PLAYER_ZOOM_SPEED,
                                        AnimatedSpriteAction::GameOverTrigger {
                                            state: GameOverState::Lost,
                                        },
                                    ),
                                    layer,
                                ));
                            }
                        },
                        TileKind::SPECIAL { interaction } => {
                            match interaction {
                                InteractionKind::LaunchMap { .. } => { // Only applicable in Campaign Trail
                                },
                                InteractionKind::VendingMachine { .. } => { // Handled already in Collision Detection
                                },
                                InteractionKind::TeleportTo { teleport_id } => {
                                    // Teleport the player to the given location
                                    commands.entity(player_entity).despawn_recursive();
                                    sprite
                                        .texture_atlas
                                        .as_mut()
                                        .map(|a| a.index = EXITING_PLAYER_SPRITE);
                                    let layer = RenderLayers::layer(LAYER_ID);
                                    if let Some(location) =
                                        worldwrapper.world.get_teleport_location(teleport_id)
                                    {
                                        commands.spawn((
                                            Sprite::from_atlas_image(
                                                sprite.image.clone(),
                                                sprite.texture_atlas.clone().unwrap(),
                                            ),
                                            *transform,
                                            AnimatedActionSprite::from_ascend_and_zoom(
                                                EXITED_PLAYER_DECAY_SPEED,
                                                EXITED_PLAYER_ASCEND_SPEED,
                                                EXITED_PLAYER_ZOOM_SPEED,
                                                AnimatedSpriteAction::Teleport {
                                                    location: *location,
                                                },
                                            ),
                                            layer,
                                        ));
                                    }
                                },
                            }
                        },
                        TileKind::PLAYERSPAWN => {},
                        TileKind::LADDER => {
                            // On a ladder, the movement queue is cleared after every step!
                            // This way, the player is unable to jump on a double ladder and ascends instead of jumping.
                            // For players with empty movement queues, this case is handled below as well.
                            player.clear_movement_queue();
                        },
                        TileKind::DOOR => {},
                        TileKind::COLLECTIBLE { .. } => {},
                        TileKind::EXIT => {
                            commands.entity(player_entity).despawn_recursive();
                            sprite
                                .texture_atlas
                                .as_mut()
                                .map(|a| a.index = EXITING_PLAYER_SPRITE);
                            let layer = RenderLayers::layer(LAYER_ID);
                            commands.spawn((
                                Sprite::from_atlas_image(
                                    sprite.image.clone(),
                                    sprite.texture_atlas.clone().unwrap(),
                                ),
                                *transform,
                                AnimatedActionSprite::from_ascend_and_zoom(
                                    EXITED_PLAYER_DECAY_SPEED,
                                    EXITED_PLAYER_ASCEND_SPEED,
                                    EXITED_PLAYER_ZOOM_SPEED,
                                    AnimatedSpriteAction::GameOverTrigger {
                                        state: GameOverState::Won {
                                            score: (*scoreboard).clone(),
                                        },
                                    },
                                ),
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

/// Respawn the player. The position must be given in world coordinates
pub fn respawn_player(
    commands: &mut Commands,
    atlas_handle_player: &TilesetManager,
    position: (f32, f32),
) {
    let mut player_inner = Player::new();
    player_inner.push_movement_queue(Movement {
        velocity: (
            0.0,
            -PLAYER_SPEED_ * (atlas_handle_player.current_tileset.texture_size() as f32),
        ),
        target: (position.0.floor() as i32, position.1.floor() as i32),
        is_manual: false,
    });
    let player: PlayerComponent = PlayerComponent {
        player: player_inner,
    };
    let layer = RenderLayers::layer(LAYER_ID);
    commands.spawn((
        Sprite::from_atlas_image(
            atlas_handle_player.current_texture_handle(),
            TextureAtlas {
                layout: atlas_handle_player.current_atlas_handle(),
                index: player.player.atlas_index(),
            },
        ),
        Transform::from_translation(Vec3::new(
            position.0 * atlas_handle_player.current_tileset().texture_size() as f32,
            position.1 * atlas_handle_player.current_tileset().texture_size() as f32,
            PLAYER_Z,
        )),
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
    respawn_player(
        &mut commands,
        &current_texture_atlas,
        (
            world.world.player_spawn().0 as f32,
            world.world.player_spawn().1 as f32,
        ),
    );
}

pub fn keyboard_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&mut PlayerComponent, &mut Sprite, &Transform)>,
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
                if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
                    set_player_direction(player, &mut sprite, false);
                    player.push_movement_queue(Movement {
                        velocity: (-vx, 0.),
                        target: (cur_x - 1, cur_y),
                        is_manual: true,
                    });
                } else if keyboard_input.just_pressed(KeyCode::ArrowUp) {
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
                } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
                    set_player_direction(player, sprite.as_mut(), true);
                    player.push_movement_queue(Movement {
                        velocity: (vx, 0.),
                        target: (cur_x + 1, cur_y),
                        is_manual: true,
                    });
                } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
                    player.push_movement_queue(Movement {
                        velocity: (0., -vy),
                        target: (cur_x, cur_y - 1),
                        is_manual: true,
                    });
                } else if keyboard_input.just_pressed(KeyCode::KeyQ) {
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
                } else if keyboard_input.just_pressed(KeyCode::KeyW) {
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
                    KeyCode::ArrowUp,
                    KeyCode::ArrowRight,
                    KeyCode::ArrowDown,
                    KeyCode::ArrowLeft,
                    KeyCode::KeyQ,
                    KeyCode::KeyW,
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
    if let Some(event) = reader.read().next() {
        commands.insert_resource(event.state.clone());
        state.set(AppState::GameOverScreen);
    }
}
