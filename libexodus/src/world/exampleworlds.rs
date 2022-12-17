use world::GameWorld;
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
            .set(2, 1, WALL)
            .set(1, 1, WALL)
            .set(1, 2, PLAYERSPAWN)
            .set(3, 1, SPIKES)
            .set(2, 2, WALL)
            .set(3, 3, SPIKESSLOPED)
            .set(4, 3, WALLCOBBLE)
            .set(4, 2, WALLCHISELED)
            .set(4, 1, WALLSMOOTH)
            .set(4, 0, WALLSMOOTH)
            .set(5, 1, SPIKES)
            .set(6, 1, SPIKESALT)
            .set(8, 2, WALLSPIKESRLTB)
            .set(1, 4, WALL)
            .set(1, 8, KEY)
            .set(2, 6, WALLNATURE)
            .set(3, 7, WALLNATURE)
            .set(4, 7, WALLNATURE)
            .set(4, 8, DOOR)
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
            .set(19, 1, ARROWDOWN)
            .set(18, 1, ARROWLEFT)
            .set(17, 1, ARROWRIGHT)
            .set(16, 1, ARROWUP)
            .set(13, 5, EXIT)
        ;
        world
    }
    ///
    /// The world that is shown in the thumbnail of the original game at https://web.archive.org/web/20010609173820/http://www.davidsansome.co.uk/pages/psion/exodus/index.htm
    pub fn showcaseworld() -> Self {
        let columns = 35;
        let rows = 15;
        let mut world: GameWorld = GameWorld::new(columns, rows);
        world
            .set_name("Showcase World")
            .set_author("Debugger");
        for c in 0..columns {
            world.set(c, 0, WALL);
        }
        for r in 0..rows {
            world.set(19, r, WALL);
        }
        world
            .set(3, 1, WALL)
            .set(0, 1, PLAYERSPAWN)
            .set(3, 2, SPIKES)
            .set(4, 1, WALL)
            .set(4, 2, WALL)
            .set(5, 2, WALL)
            .set(5, 3, WALL)
            .set(6, 3, WALL)
            .set(7, 3, WALL)
            .set(8, 3, WALL)
            .set(9, 3, WALL)
            .set(10, 3, WALL)
            .set(10, 2, WALL)

            .set(7, 4, ARROWUP)
            .set(7, 1, ARROWLEFT)
            .set(7, 7, ARROWUP)
            .set(7, 5, WALL)
            .set(10, 5, WALL)
            .set(7, 6, LADDER)
            .set(7, 7, LADDER)
            .set(7, 8, LADDER)
            .set(7, 9, LADDER)
            .set(7, 10, LADDER)

            .set(6, 9, WALL)
            .set(5, 9, WALL)
            .set(4, 9, WALL)
            .set(3, 9, WALL)
            .set(3, 10, WALL)
            .set(3, 11, WALL)
            .set(3, 12, WALL)
            .set(3, 13, WALL)
            .set(3, 14, WALL)
            .set(4, 10, SPIKES)
            .set(5, 10, SPIKES)
            .set(6, 10, SPIKES)

            .set(8, 10, WALL)
            .set(8, 14, EXIT)
            .set(9, 10, WALL)
            .set(10, 10, WALL)
            .set(10, 11, WALL)
            .set(10, 12, WALL)
            .set(10, 14, WALL)
            .set(11, 14, WALL)
            .set(12, 14, WALL)
            .set(13, 13, EXIT)
            .set(12, 13, WALL)
            .set(14, 13, WALL)
            .set(15, 13, WALL)
            .set(16, 13, WALL)

            .set(17, 13, LADDER)
            .set(17, 12, LADDER)
            .set(17, 11, LADDER)
            .set(17, 10, LADDER)

            .set(17, 9, WALL)
            .set(16, 9, WALL)
            .set(15, 9, WALL)
            .set(14, 9, WALL)
            .set(18, 9, WALL)

            .set(12, 7, WALL)

            .set(16, 1, WALL)
            .set(17, 1, SPIKES)
            .set(18, 1, SPIKES)

            .set(27, 1, WALL)
            .set(28, 1, EXIT)
            .set(29, 1, WALL)

            .set(24, 5, WALL)
            .set(25, 5, WALL)
            .set(26, 5, WALL)
            .set(27, 5, WALL)
            .set(28, 5, WALL)
            .set(29, 5, WALL)
            .set(30, 5, WALL)
            .set(31, 5, WALL)
            .set(27, 4, WALL)
            .set(28, 4, WALL)

            .set(24, 4, COIN)
            .set(25, 4, COIN)
            .set(26, 4, COIN)
            .set(29, 4, COIN)
            .set(30, 4, COIN)
            .set(31, 4, COIN)

            .set(21, 8, WALL)
            .set(22, 8, WALL)
            .set(23, 8, WALL)
            .set(24, 8, WALL)
            .set(25, 8, WALL)
            .set(26, 8, WALL)
            .set(27, 8, WALL)
            .set(28, 8, WALL)
            .set(29, 8, WALL)
            .set(30, 8, WALL)
            .set(31, 8, WALL)
            .set(32, 8, WALL)
            .set(33, 8, WALL)
            .set(21, 9, ARROWLEFT)
            .set(24, 9, SPIKES)
            .set(26, 9, SPIKES)
            .set(30, 9, SPIKES)
            .set(33, 9, ARROWRIGHT)

            .set(23, 12, WALL)
            .set(23, 13, WALL)
            .set(24, 13, WALL)
            .set(25, 13, WALL)
            .set(25, 12, WALL)
            .set(26, 13, WALL)
            .set(27, 12, WALL)
            .set(27, 13, WALL)
            .set(28, 13, WALL)
            .set(29, 12, WALL)
            .set(29, 13, WALL)
            .set(30, 13, WALL)
            .set(31, 12, WALL)
            .set(31, 13, WALL)
        ;
        world
    }
}