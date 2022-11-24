use bevy::prelude::*;
use crate::World;

#[derive(Resource)]
pub struct Scoreboard {
    pub coins: i32,
    // This might be changed to a HashMap later to support multiplayer
    pub moves: usize, // see above
}

impl FromWorld for Scoreboard {
    fn from_world(_: &mut World) -> Self {
        Scoreboard {
            coins: 0,
            moves: 0,
        }
    }
}

impl Scoreboard {
    pub fn reset(&mut self) {
        self.coins = 0;
        self.moves = 0;
    }
}