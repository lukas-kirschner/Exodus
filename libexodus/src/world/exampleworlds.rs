use world::GameWorld;
use crate::tiles::*;
use crate::tiles::Tile::*;
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
            .set_name("Example World")
            .set_author("Debugger")
            .set_uuid("badeaffe-e4fe-47af-8ff6-0000c0febabe")
            .set(2, 1, WALL)
            .set(1, 1, WALL)
            .set(1, 2, PLAYERSPAWN)
            .set(3, 1, SPIKES)
            .set(2, 2, WALL)
            .set(3, 3, SPIKESSLOPED)
            .set(4, 3, WALL)
            .set(4, 2, WALL)
            .set(4, 1, WALL)
            .set(4, 0, WALL)
            .set(5, 1, SPIKES)
            .set(6, 1, SPIKESALT)
            .set(8, 2, WALLSPIKESRLTB)
            .set(1, 4, WALL)
            .set(1, 8, COIN)
            .set(2, 6, WALL)
            .set(3, 7, WALL)
            .set(4, 7, WALL)
            .set(6, 3, WALL)
            .set(5, 5, LADDER)
            .set(9, 1, COIN)
            .set(9, 2, COIN)
            .set(9, 3, COIN)
            .set(12, 1, WALLSPIKESL)
            .set(14, 1, WALLSPIKESR)
            .set(14, 4, WALLSPIKESB)
            .set(13, 4, WALLSPIKESLB)
            .set(15, 4, WALLSPIKESRB)
            .set(16, 1, WALLSPIKESLR)
            .set(17, 1, LADDER)
            .set(17, 2, LADDER)
            .set(17, 3, LADDER)
            .set(17, 4, LADDER)
            .set(16, 5, WALLSPIKESRB)
            .set(17, 8, WALLSPIKESLTB)
            .set(18, 8, WALLSPIKESTB)
            .set(19, 8, WALLSPIKESRTB)
            .set(17, 7, COIN)
        ;
        world
    }
}