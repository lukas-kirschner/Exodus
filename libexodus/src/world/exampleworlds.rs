use world::GameWorld;
use crate::tiles::{SLOPED_SPIKES, SPIKES, WALL};
use crate::world;

impl GameWorld {
    ///
    /// An example world that is meant to showcase all available tiles in the game for debugging.
    pub fn exampleworld() -> Self {
        let columns = 24; // Number of columns in the game board
        let rows = 10; // Number of rows in the game board
        let mut world: GameWorld = GameWorld::new(columns, rows);
        world
            .set(0, 0, &WALL)
            .set(1, 0, &WALL)
            .set(2, 0, &WALL)
            .set(2, 1, &WALL)
            .set(2, 2, &SPIKES)
            .set(1, 1, &SPIKES)
            .set(2, 3, &WALL)
            .set(3, 3, &SLOPED_SPIKES)
            .set(4, 3, &WALL)
            .set(4, 2, &WALL)
            .set(4, 1, &WALL)
            .set(4, 0, &WALL)
        ;
        world
    }
}