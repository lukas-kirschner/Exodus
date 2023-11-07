use crate::animation::animated_action_sprite::AnimatedActionSpritePlugin;
use bevy::prelude::*;

pub mod animated_action_sprite;

pub struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AnimatedActionSpritePlugin);
    }
}
