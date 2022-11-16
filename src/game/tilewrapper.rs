use bevy::prelude::*;
use libexodus::world::GameWorld;
use libexodus::world::presets;
use crate::AppState;
use crate::game::constants::COIN_PICKUP_DISTANCE;
use crate::game::player::PlayerComponent;
use crate::game::scoreboard::Scoreboard;
use crate::util::dist_2d;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Playing)
                .with_system(reset_score).label("reset_score"))
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

impl MapWrapper {
    /// Get the name of the world
    pub fn name(&self) -> &str {
        self.world.get_name()
    }
    /// Get the author name of the world
    pub fn author(&self) -> &str {
        self.world.get_author()
    }
    /// Get the UUID of this world
    pub fn uuid(&self) -> String {
        self.world.uuid()
    }
}

impl FromWorld for MapWrapper {
    fn from_world(_: &mut World) -> Self {
        MapWrapper {
            world: presets::map_with_border(24, 10),
        }
    }
}

impl MapWrapper {
    pub fn _set_world(&mut self, world: GameWorld) {
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
pub struct TileWrapper;

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
                commands.entity(coin_entity).despawn_recursive();
            }
        }
    }
}

fn reset_score(
    mut scoreboard: ResMut<Scoreboard>
) {
    scoreboard.coins = 0;
    scoreboard.moves = 0;
}