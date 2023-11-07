use crate::game::scoreboard::Scoreboard;
use bevy::prelude::*;
use libexodus::world::presets::map_with_border;
use libexodus::world::GameWorld;

///
/// A wrapper around a GameWorld
#[derive(Resource)]
pub struct MapWrapper {
    pub world: GameWorld,
    pub previous_best: Option<Scoreboard>,
}

impl FromWorld for MapWrapper {
    fn from_world(_: &mut World) -> Self {
        MapWrapper {
            world: map_with_border(24, 10),
            previous_best: None,
        }
    }
}

pub fn reset_score(mut scoreboard: ResMut<Scoreboard>) {
    scoreboard.coins = 0;
    scoreboard.moves = 0;
}
