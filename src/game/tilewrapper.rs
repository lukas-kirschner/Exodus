use bevy::prelude::*;
use libexodus::world::GameWorld;
use libexodus::world::presets;
use crate::game::scoreboard::Scoreboard;

///
/// A wrapper around a GameWorld
#[derive(Resource)]
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
    pub fn _set_world(&mut self, world: GameWorld) {
        self.world = world;
    }
}


pub fn reset_score(
    mut scoreboard: ResMut<Scoreboard>
) {
    scoreboard.coins = 0;
    scoreboard.moves = 0;
}