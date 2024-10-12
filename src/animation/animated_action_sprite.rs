use crate::game::player::respawn_player;
use crate::game::scoreboard::{GameOverEvent, GameOverState};
use crate::textures::tileset_manager::TilesetManager;
use crate::{AppLabels, AppState, GameConfig};
use bevy::prelude::*;

pub struct AnimatedActionSpritePlugin;
impl Plugin for AnimatedActionSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            animated_action_sprite_handler
                .run_if(resource_exists::<GameConfig>)
                .run_if(resource_exists::<TilesetManager>)
                .in_set(AppLabels::PlayerMovement),
        )
        .add_systems(
            OnEnter(AppState::Playing),
            despawn_and_discard_all_animated_action_tiles,
        );
    }
}

/// A enum of possible changes that may be triggered after a AnimatedActionSprite has decayed
#[derive(Clone)]
pub enum AnimatedSpriteAction {
    /// Change the application state to the given state
    StateChange {
        state: AppState,
    },
    /// Trigger a Game Over Event
    GameOverTrigger {
        state: GameOverState,
    },
    /// Teleport the player to the given location
    Teleport {
        location: (usize, usize),
    },
    /// Respawn this animation at the given location once it has finished
    RespawnAnimation {
        animation: Box<AnimatedActionSprite>,
        location: (f32, f32),
    },
    // TODO State for arbitrary scoreboard change: Pass a FnOnce which will be executed when animated sprite decays?
    None,
}
#[derive(Component, Clone)]
pub struct AnimatedActionSprite {
    /// The amount of transparency that is added per second. Must be <0
    delta_alpha: f32,
    /// The amount of translation that is added to the translation vector per second
    delta_translation: Vec3,
    /// The amount of scale that is added to the scale vector per second
    delta_scale: Vec3,
    /// The amount of rotation that is added to the rotation vector per second
    delta_rotation: Quat,
    /// Action that should be executed after the animated sprite has decayed
    action_after_despawn: AnimatedSpriteAction,
}

impl AnimatedActionSprite {
    pub fn alpha(&self) -> f32 {
        self.delta_alpha
    }
    pub fn translation(&self) -> Vec3 {
        self.delta_translation
    }
    pub fn scale(&self) -> Vec3 {
        self.delta_scale
    }
    pub fn rotation(&self) -> Quat {
        self.delta_rotation
    }
    pub fn action(&self) -> &AnimatedSpriteAction {
        &self.action_after_despawn
    }
    /// Set this action to repeat itself before executing the previously set action
    pub fn set_repeat(&mut self, times: usize, location: (f32, f32)) {
        for _ in 0..times {
            let new_action = AnimatedSpriteAction::RespawnAnimation {
                animation: Box::new(self.clone()),
                location,
            };
            self.action_after_despawn = new_action;
        }
    }
    pub fn new(
        delta_alpha: f32,
        delta_translation: Vec3,
        delta_scale: Vec3,
        delta_rotation: Quat,
        action_after_despawn: AnimatedSpriteAction,
    ) -> Self {
        assert!(delta_alpha < 0., "Since an AnimatedActionSprite decays when alpha reaches zero, delta_alpha must have a value less than zero!");
        Self {
            delta_alpha,
            delta_translation,
            delta_scale,
            delta_rotation,
            action_after_despawn,
        }
    }
    pub fn from_translation_scale(
        delta_alpha: f32,
        delta_translation: Vec3,
        delta_scale: Vec3,
        action_after_despawn: AnimatedSpriteAction,
    ) -> Self {
        AnimatedActionSprite::new(
            delta_alpha,
            delta_translation,
            delta_scale,
            Quat::from_array([0.0; 4]),
            action_after_despawn,
        )
    }
    // pub fn from_translation(
    //     delta_alpha: f32,
    //     delta_translation: Vec3,
    //     action_after_despawn: AnimatedSpriteAction,
    // ) -> Self {
    //     AnimatedActionSprite::from_translation_scale(
    //         delta_alpha,
    //         delta_translation,
    //         (0.0, 0.0, 0.0).into(),
    //         action_after_despawn,
    //     )
    // }
    pub fn from_ascend_and_zoom(
        delta_alpha: f32,
        ascend_speed: f32,
        zoom_speed: f32,
        action_after_despawn: AnimatedSpriteAction,
    ) -> Self {
        AnimatedActionSprite::from_ascend_angle_and_zoom(
            delta_alpha,
            ascend_speed,
            0.0,
            zoom_speed,
            action_after_despawn,
        )
    }
    pub fn from_ascend_angle_and_zoom(
        delta_alpha: f32,
        ascend_speed: f32,
        angle: f32,
        zoom_speed: f32,
        action_after_despawn: AnimatedSpriteAction,
    ) -> Self {
        AnimatedActionSprite::from_translation_scale(
            delta_alpha,
            (
                -angle.to_radians().sin() * ascend_speed,
                angle.to_radians().cos() * ascend_speed,
                0.0,
            )
                .into(),
            (zoom_speed, zoom_speed, 0.0).into(),
            action_after_despawn,
        )
    }
}

fn animated_action_sprite_handler(
    mut commands: Commands,
    mut animated_sprites: Query<(&mut Sprite, &mut Transform, Entity, &AnimatedActionSprite)>,
    config: Res<GameConfig>,
    time: Res<Time>,
    mut app_state: ResMut<NextState<AppState>>,
    mut event_writer: EventWriter<GameOverEvent>,
    tileset_manager: Res<TilesetManager>,
) {
    let texture_size = config.texture_size();
    for (mut sprite, mut transform, entity, animated_sprite) in animated_sprites.iter_mut() {
        let new_a: f32 = sprite.color.alpha() + (animated_sprite.alpha() * time.delta_seconds());
        if new_a <= 0.0 {
            // The player has fully decayed and can be despawned.
            match animated_sprite.action() {
                AnimatedSpriteAction::RespawnAnimation {
                    animation,
                    location,
                } => {
                    // Change the location, alpha and metadata of the animated sprite
                    commands
                        .entity(entity)
                        .remove::<AnimatedActionSprite>()
                        .insert((**animation).clone());
                    transform.translation.x = location.0;
                    transform.translation.y = location.1;
                    sprite.color.set_alpha(1.0);
                    return;
                },
                _ => {
                    // Despawn the sprite completely
                    sprite.color.set_alpha(0.0);
                    commands.entity(entity).despawn_recursive();
                },
            }
            match animated_sprite.action() {
                AnimatedSpriteAction::StateChange { state } => {
                    debug!(
                        "Entering state {:?}, triggered by AnimatedActionSprite",
                        state
                    );
                    app_state.set(*state);
                },
                AnimatedSpriteAction::None => {},
                AnimatedSpriteAction::RespawnAnimation { .. } => {},
                AnimatedSpriteAction::GameOverTrigger { state } => {
                    debug!(
                        "Sending GameOverEvent {:?}, triggered by AnimatedActionSprite",
                        state
                    );
                    event_writer.send(GameOverEvent {
                        state: state.clone(),
                    });
                },
                AnimatedSpriteAction::Teleport { location } => {
                    debug!(
                        "Teleporting Player to ({},{}), triggered by AnimatedActionSprite",
                        location.0, location.1
                    );
                    respawn_player(
                        &mut commands,
                        &tileset_manager,
                        (location.0 as f32, location.1 as f32 + 0.75),
                    )
                },
            }
            return;
        }
        sprite.color.set_alpha(new_a);
        transform.translation +=
            animated_sprite.translation() * texture_size * time.delta_seconds();
        transform.scale += animated_sprite.scale() * texture_size * time.delta_seconds();
        transform.rotation.x += animated_sprite.rotation().x * texture_size * time.delta_seconds();
        transform.rotation.y += animated_sprite.rotation().y * texture_size * time.delta_seconds();
        transform.rotation.z += animated_sprite.rotation().z * texture_size * time.delta_seconds();
        transform.rotation.w += animated_sprite.rotation().w * texture_size * time.delta_seconds();
    }
}

fn despawn_and_discard_all_animated_action_tiles(
    mut commands: Commands,
    animated_sprites: Query<Entity, With<AnimatedActionSprite>>,
) {
    for entity in animated_sprites.iter() {
        debug!("Despawning leftover animated entity {:?}", &entity);
        commands.entity(entity).despawn_recursive();
    }
}
