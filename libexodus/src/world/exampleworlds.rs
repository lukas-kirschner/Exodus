use world::GameWorld;
use crate::tiles::{PLAYER_SPAWN, SLOPED_SPIKES, SPIKES, WALL};
use crate::world;
use crate::world::presets;

impl GameWorld {
    ///
    /// An example world that is meant to showcase all available tiles in the game for debugging.
    pub fn exampleworld() -> Self {
        let columns = 24; // Number of columns in the game board
        let rows = 10; // Number of rows in the game board
        let mut world: GameWorld = presets::map_with_border(columns, rows);
        world
            .set(2, 1, &WALL)
            .set(1, 1, &WALL)
            .set(1, 2, &PLAYER_SPAWN)
            .set(3, 1, &SPIKES)
            .set(2, 2, &WALL)
            .set(3, 3, &SLOPED_SPIKES)
            .set(4, 3, &WALL)
            .set(4, 2, &WALL)
            .set(4, 1, &WALL)
            .set(4, 0, &WALL)
        ;
        world
    }
}