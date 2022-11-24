use bevy::prelude::*;
use crate::AppState;
use crate::game::constants::{COIN_PICKUP_DISTANCE, PICKUP_ITEM_ASCEND_SPEED, PICKUP_ITEM_DECAY_SPEED, PICKUP_ITEM_ZOOM_SPEED};
use crate::game::player::PlayerComponent;
use crate::game::scoreboard::Scoreboard;
use crate::util::dist_2d;


#[derive(Component)]
pub struct PickupItem;

/// Handler that takes care of animating and despawning the picked up item.
pub fn pickup_item_animation(
    mut commands: Commands,
    mut dead_players: Query<(&mut TextureAtlasSprite, &mut Transform, Entity), With<PickupItem>>,
    time: Res<Time>,
) {
    for (mut sprite, mut transform, entity) in dead_players.iter_mut() {
        let new_a: f32 = sprite.color.a() - (PICKUP_ITEM_DECAY_SPEED * time.delta_seconds());
        if new_a <= 0.0 {
            // The player has fully decayed and can be despawned
            commands.entity(entity).despawn_recursive();
            return;
        }
        sprite.color.set_a(new_a);
        transform.translation.y += PICKUP_ITEM_ASCEND_SPEED * time.delta_seconds();
        transform.scale += Vec3::splat(PICKUP_ITEM_ZOOM_SPEED * time.delta_seconds());
    }
}

pub struct PickupItemPlugin;

impl Plugin for PickupItemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(AppState::Playing)
                .with_system(coin_collision).after("player_movement")
            )
            .add_system_set(SystemSet::on_update(AppState::Playing)
                .with_system(pickup_item_animation).after("player_movement")
            )
        ;
    }
}

/// A wrapper for coins
#[derive(Component)]
pub struct CoinWrapper<> {
    /// The value of this coin, i.e. the score a player gets for collecting the coin
    pub coin_value: i32,
}

pub fn coin_collision(
    mut commands: Commands,
    mut coin_query: Query<(Entity, &Transform, &mut CoinWrapper)>,
    players: Query<(&PlayerComponent, &Transform)>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    for (_player, player_trans) in players.iter() {
        let player_pos: Vec3 = player_trans.translation;
        for (coin_entity, coin_trans, coin) in coin_query.iter_mut() {
            let coin_pos: Vec3 = coin_trans.translation;
            let dist = dist_2d(&player_pos, &coin_pos);
            if dist <= COIN_PICKUP_DISTANCE {
                // The player picks up the coin
                scoreboard.coins += coin.coin_value;
                commands.entity(coin_entity).remove::<CoinWrapper>().insert(PickupItem);
            }
        }
    }
}
