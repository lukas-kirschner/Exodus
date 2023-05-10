use crate::game::scoreboard::Scoreboard;
use bevy::prelude::*;
use libexodus::world::presets::map_with_border;
use libexodus::world::GameWorld;

#[derive(Resource, Clone)]
pub enum GameOverState {
    /// The game was lost, i.e., the player died losing all lives without reaching the exit
    Lost,
    /// The player won the game with the given scoreboard
    Won { score: Scoreboard },
    // /// The player won the game as part of the campaign
    // WON_CAMPAIGN { score: Scoreboard },
}

///
/// A wrapper around a GameWorld
#[derive(Resource)]
pub struct MapWrapper {
    pub world: GameWorld,
    pub previous_best: Option<Scoreboard>,
}

impl FromWorld for MapWrapper {
    fn from_world(world: &mut World) -> Self {
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
