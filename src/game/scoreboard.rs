use crate::World;
use bevy::prelude::*;
use libexodus::highscores::highscore::Highscore;

#[derive(Resource, Clone)]
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
    pub fn new(coins: i32, moves: usize, keys: usize) -> Self {
        Scoreboard { coins, moves, keys }
    }
}

impl From<&Highscore> for Scoreboard {
    fn from(value: &Highscore) -> Self {
        Scoreboard::new(value.coins() as i32, value.moves() as usize, 0usize)
    }
}
