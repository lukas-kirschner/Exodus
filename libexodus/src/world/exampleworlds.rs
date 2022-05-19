use world::GameWorld;
use crate::tiles::*;
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
            .set(2, 1, wall())
            .set(1, 1, wall())
            .set(1, 2, player_spawn())
            .set(3, 1, spikes())
            .set(2, 2, wall())
            .set(3, 3, sloped_spikes())
            .set(4, 3, wall())
            .set(4, 2, wall())
            .set(4, 1, wall())
            .set(4, 0, wall())
            .set(5, 1, spikes())
            .set(6, 1, spikes_alternative_a())
            .set(8, 2, spikes_platform())
            .set(1, 4, wall())
            .set(2, 6, wall())
            .set(3, 7, wall())
            .set(4, 7, wall())
            .set(6, 3, wall())
            .set(9, 1, coin())
            .set(9, 2, coin())
            .set(9, 3, coin())
            .set(12, 1, spikes_platform())
        ;
        world
    }
}