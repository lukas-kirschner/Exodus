use crate::game::scoreboard::{GameOverEvent, GameOverState};
use crate::{AppLabels, AppState, GameConfig};
use bevy::prelude::*;

pub struct AnimatedActionSpritePlugin;
impl Plugin for AnimatedActionSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            animated_action_sprite_handler
                .run_if(resource_exists::<GameConfig>())
                .in_set(AppLabels::PlayerMovement),
        );
    }
}

/// A enum of possible changes that may be triggered after a AnimatedActionSprite has decayed
pub enum AnimatedSpriteAction {
    StateChange { state: AppState },
    GameOverTrigger { state: GameOverState },
    // TODO State for arbitrary scoreboard change: Pass a FnOnce which will be executed when animated sprite decays?
    None,
}
#[derive(Component)]
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
    pub fn from_translation(
        delta_alpha: f32,
        delta_translation: Vec3,
        action_after_despawn: AnimatedSpriteAction,
    ) -> Self {
        AnimatedActionSprite::from_translation_scale(
            delta_alpha,
            delta_translation,
            (0.0, 0.0, 0.0).into(),
            action_after_despawn,
        )
    }
    pub fn from_ascend_and_zoom(
        delta_alpha: f32,
        ascend_speed: f32,
        zoom_speed: f32,
        action_after_despawn: AnimatedSpriteAction,
    ) -> Self {
        AnimatedActionSprite::from_translation_scale(
            delta_alpha,
            (0.0, ascend_speed, 0.0).into(),
            (zoom_speed, zoom_speed, 0.0).into(),
            action_after_despawn,
        )
    }
}

fn animated_action_sprite_handler(
    mut commands: Commands,
    mut animated_sprites: Query<(
        &mut TextureAtlasSprite,
        &mut Transform,
        Entity,
        &AnimatedActionSprite,
    )>,
    config: Res<GameConfig>,
    time: Res<Time>,
    mut app_state: ResMut<NextState<AppState>>,
    mut event_writer: EventWriter<GameOverEvent>,
) {
    let texture_size = config.texture_size();
    for (mut sprite, mut transform, entity, animated_sprite) in animated_sprites.iter_mut() {
        let new_a: f32 = sprite.color.a() + (animated_sprite.alpha() * time.delta_seconds());
        if new_a <= 0.0 {
            // The player has fully decayed and can be despawned.
            sprite.color.set_a(0.0);
            commands.entity(entity).despawn_recursive();
            match &animated_sprite.action_after_despawn {
                AnimatedSpriteAction::StateChange { state } => {
                    debug!(
                        "Entering state {:?}, triggered by AnimatedActionSprite",
                        state
                    );
                    app_state.set(*state);
                },
                AnimatedSpriteAction::None => {},
                AnimatedSpriteAction::GameOverTrigger { state } => {
                    debug!(
                        "Sending GameOverEvent {:?}, triggered by AnimatedActionSprite",
                        state
                    );
                    event_writer.send(GameOverEvent {
                        state: state.clone(),
                    });
                },
            }
            return;
        }
        sprite.color.set_a(new_a);
        transform.translation +=
            animated_sprite.translation() * texture_size * time.delta_seconds();
        transform.scale += animated_sprite.scale() * texture_size * time.delta_seconds();
        transform.rotation.x += animated_sprite.rotation().x * texture_size * time.delta_seconds();
        transform.rotation.y += animated_sprite.rotation().y * texture_size * time.delta_seconds();
        transform.rotation.z += animated_sprite.rotation().z * texture_size * time.delta_seconds();
        transform.rotation.w += animated_sprite.rotation().w * texture_size * time.delta_seconds();
    }
}
