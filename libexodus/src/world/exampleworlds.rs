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
            .set_name("Example World")
            .set_author("Debugger")
            .set_uuid("badeaffe-e4fe-47af-8ff6-0000c0febabe")
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
            .set(8, 2, wall_spikes_rltb())
            .set(1, 4, wall())
            .set(1, 8, coin())
            .set(2, 6, wall())
            .set(3, 7, wall())
            .set(4, 7, wall())
            .set(6, 3, wall())
            .set(5, 5, ladder())
            .set(9, 1, coin())
            .set(9, 2, coin())
            .set(9, 3, coin())
            .set(12, 1, wall_spikes_l())
            .set(14, 1, wall_spikes_r())
            .set(14, 4, wall_spikes_b())
            .set(13, 4, wall_spikes_lb())
            .set(15, 4, wall_spikes_rb())
            .set(16, 1, wall_spikes_lr())
            .set(17, 1, ladder())
            .set(17, 2, ladder())
            .set(17, 3, ladder())
            .set(17, 4, ladder())
            .set(16, 5, wall_spikes_tb())
            .set(17, 8, wall_spikes_ltb())
            .set(18, 8, wall_spikes_tb())
            .set(19, 8, wall_spikes_rtb())
            .set(17, 7, coin())
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
            .set_author("Debugger")
            .set_uuid("fbcfbe80-6ced-4573-95df-b443b66a07c4");
        for c in 0..columns {
            world.set(c, 0, wall());
        }
        for r in 0..rows {
            world.set(19, r, wall());
        }
        world
            .set(3, 1, wall())
            .set(0, 1, player_spawn())
            .set(3, 2, spikes())
            .set(4, 1, wall())
            .set(4, 2, wall())
            .set(5, 2, wall())
            .set(5, 3, wall())
            .set(6, 3, wall())
            .set(7, 3, wall())
            .set(8, 3, wall())
            .set(9, 3, wall())
            .set(10, 3, wall())
            .set(10, 2, wall())

            .set(7, 5, wall())
            .set(10, 5, wall())
            .set(7, 6, ladder())
            .set(7, 7, ladder())
            .set(7, 8, ladder())
            .set(7, 9, ladder())
            .set(7, 10, ladder())

            .set(6, 9, wall())
            .set(5, 9, wall())
            .set(4, 9, wall())
            .set(3, 9, wall())
            .set(3, 10, wall())
            .set(3, 11, wall())
            .set(3, 12, wall())
            .set(3, 13, wall())
            .set(3, 14, wall())
            .set(4, 10, spikes())
            .set(5, 10, spikes())
            .set(6, 10, spikes())

            .set(8, 10, wall())
            .set(9, 10, wall())
            .set(10, 10, wall())
            .set(10, 11, wall())
            .set(10, 12, wall())
            .set(10, 14, wall())
            .set(11, 14, wall())
            .set(12, 14, wall())
            .set(12, 13, wall())
            .set(14, 13, wall())
            .set(15, 13, wall())
            .set(16, 13, wall())

            .set(17, 13, ladder())
            .set(17, 12, ladder())
            .set(17, 11, ladder())
            .set(17, 10, ladder())

            .set(17, 9, wall())
            .set(16, 9, wall())
            .set(15, 9, wall())
            .set(14, 9, wall())
            .set(18, 9, wall())

            .set(12, 7, wall())

            .set(16, 1, wall())
            .set(17, 1, spikes())
            .set(18, 1, spikes())

            .set(27, 1, wall())
            .set(29, 1, wall())

            .set(24, 5, wall())
            .set(25, 5, wall())
            .set(26, 5, wall())
            .set(27, 5, wall())
            .set(28, 5, wall())
            .set(29, 5, wall())
            .set(30, 5, wall())
            .set(31, 5, wall())
            .set(27, 4, wall())
            .set(28, 4, wall())

            .set(21, 8, wall())
            .set(22, 8, wall())
            .set(23, 8, wall())
            .set(24, 8, wall())
            .set(25, 8, wall())
            .set(26, 8, wall())
            .set(27, 8, wall())
            .set(28, 8, wall())
            .set(29, 8, wall())
            .set(30, 8, wall())
            .set(31, 8, wall())
            .set(32, 8, wall())
            .set(33, 8, wall())
            .set(24, 9, spikes())
            .set(26, 9, spikes())
            .set(30, 9, spikes())

            .set(23, 12, wall())
            .set(23, 13, wall())
            .set(24, 13, wall())
            .set(25, 13, wall())
            .set(25, 12, wall())
            .set(26, 13, wall())
            .set(27, 12, wall())
            .set(27, 13, wall())
            .set(28, 13, wall())
            .set(29, 12, wall())
            .set(29, 13, wall())
            .set(30, 13, wall())
            .set(31, 12, wall())
            .set(31, 13, wall())
        ;
        world
    }
}