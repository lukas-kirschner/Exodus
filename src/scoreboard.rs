use bevy::prelude::FromWorld;
use crate::World;

pub struct Scoreboard {
    pub scores: i32, // This might be changed to a HashMap later to support multiplayer
}

impl FromWorld for Scoreboard {
    fn from_world(_: &mut World) -> Self {
        Scoreboard {
            scores: 0,
        }
    }
}