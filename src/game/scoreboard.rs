use crate::World;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Scoreboard {
    pub coins: i32,
    pub moves: usize,
    pub keys: usize,
}

impl FromWorld for Scoreboard {
    fn from_world(_: &mut World) -> Self {
        Scoreboard {
            coins: 0,
            moves: 0,
            keys: 0,
        }
    }
}

impl Scoreboard {
    pub fn reset(&mut self) {
        self.coins = 0;
        self.moves = 0;
        self.keys = 0;
    }
}
