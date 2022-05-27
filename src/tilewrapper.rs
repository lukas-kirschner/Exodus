use bevy::prelude::*;
use libexodus::world::GameWorld;
use libexodus::world::presets;
use crate::{AppState, COIN_PICKUP_DISTANCE, PlayerComponent};
use crate::scoreboard::Scoreboard;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(AppState::Playing)
                .with_system(coin_collision).after("player_movement")
            )
        ;
    }
}

///
/// A wrapper around a GameWorld
pub struct MapWrapper<> {
    pub world: GameWorld,
}

impl FromWorld for MapWrapper {
    fn from_world(_: &mut World) -> Self {
        MapWrapper {
            world: presets::map_with_border(24, 10),
        }
    }
}

impl MapWrapper {
    pub fn set_world(&mut self, world: GameWorld) {
        self.world = world;
    }
}

/// A wrapper for coins
#[derive(Component)]
pub struct CoinWrapper<> {
    /// The value of this coin, i.e. the score a player gets for collecting the coin
    pub coin_value: i32,
}

/// A wrapper for any other map tile
#[derive(Component)]
pub struct TileWrapper<> {}

pub fn coin_collision(
    mut commands: Commands,
    mut coin_query: Query<(Entity, &Transform, &mut CoinWrapper)>,
    players: Query<(&PlayerComponent, &Transform)>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    for (_player, player_trans) in players.iter() {
        let player_pos = player_trans.translation;
        for (coin_entity, coin_trans, coin) in coin_query.iter_mut() {
            let coin_pos = coin_trans.translation;
            if player_pos.distance(coin_pos) <= COIN_PICKUP_DISTANCE {
                // The player picks up the coin
                scoreboard.scores += coin.coin_value;
                commands.entity(coin_entity).despawn_recursive();
            }
        }
    }
}