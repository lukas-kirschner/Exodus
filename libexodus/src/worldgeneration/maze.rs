use crate::tiles::Tile;
use crate::world::GameWorld;
use crate::worldgeneration::{WorldGenerationAlgorithm, WorldGenerationError};
use rand::Rng;
use rand::prelude::IndexedRandom;
use rand_chacha::ChaCha20Rng;
use rand_chacha::rand_core::SeedableRng;
use std::cmp::{max, min};

#[derive(Clone)]
pub(super) struct Maze {
    pub width: u32,
    pub height: u32,
    pub color: Tile,
    pub seed: u32,
}

impl Maze {
    /// Generate a new maze map using Prim's Algorithm.
    /// All Values must be already validated before calling this function.
    /// The algorithm works as follows:
    /// 1. The maze consists of 2-dimensional grid-like cells that represent either WALL or AIR.
    /// 2. Start with a grid full of WALL tiles.
    /// 3. Pick a random cell, set it to AIR and compute its frontier cells.
    ///    A frontier cell of a cell is a cell with distance 2 in state WALL.
    /// 4. While the list of frontier cells is not empty:
    ///    1. Pick a random frontier cell from the list of frontier cells.
    ///    2. Let neighbors(frontierCell) = All cells in distance 2 in state AIR.
    ///    3. Pick a random neighbor and connect the frontier cell with the neighbor
    ///       by setting the cell in-between to state Passage.
    ///    4. Compute the frontier cells of the chosen frontier cell and add them to the
    ///       frontier list.
    ///    5. Remove the chosen frontier cell from the list of frontier cells.
    ///
    /// See https://stackoverflow.com/a/29758926
    fn generate_validated(&self) -> Result<GameWorld, WorldGenerationError> {
        let mut frontier_cells: Vec<(usize, usize)> = vec![];
        let mut ret = GameWorld::new(self.width as usize, self.height as usize);
        // Start with a maze that consists only of WALL
        ret.fill(&self.color);
        let seed = [
            ((self.seed & 0xff000000) >> 24) as u8,
            ((self.seed & 0x00ff0000) >> 16) as u8,
            ((self.seed & 0x0000ff00) >> 8) as u8,
            (self.seed & 0x000000ff) as u8,
        ]
        .repeat(8);
        let mut rng = ChaCha20Rng::from_seed(seed.try_into().unwrap());

        let start_cell: (usize, usize) = (
            rng.random_range(0..self.width as usize),
            rng.random_range(0..self.height as usize),
        );
        frontier_cells.push(start_cell);
        while !frontier_cells.is_empty() {
            // Pop a random frontier cell. TODO: Choose a more efficient data structure for removing
            let remove_ind = rng.random_range(0..frontier_cells.len());
            let current_coord = frontier_cells.remove(remove_ind);

            // Check if the cell is in fact unvisited and continue if it is visited
            if let Some(Tile::AIR) = ret.get(current_coord.0 as i32, current_coord.1 as i32) {
                continue;
            }
            ret.set(current_coord.0, current_coord.1, Tile::AIR);

            // Push all possible frontier cells
            frontier_cells.extend(self.frontier_cells(&ret, current_coord));

            // Select a random wall to an already-visited cell to remove
            let connection_walls = self.connecting_walls(&ret, current_coord);
            if let Some(wall_to_connect) = connection_walls.choose(&mut rng) {
                self.connect_wall(&mut ret, wall_to_connect, current_coord);
            }
        }
        Ok(ret)
    }
    /// Compute all unvisited frontier cells of the given cell
    fn frontier_cells(&self, map: &GameWorld, cell: (usize, usize)) -> Vec<(usize, usize)> {
        self.find_neighbor_cells(map, cell, false)
    }
    fn find_neighbor_cells(
        &self,
        map: &GameWorld,
        cell: (usize, usize),
        matches_air: bool,
    ) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        for test_coord in [
            (cell.0 as i32 + 2, cell.1 as i32),
            (cell.0 as i32 - 2, cell.1 as i32),
            (cell.0 as i32, cell.1 as i32 + 2),
            (cell.0 as i32, cell.1 as i32 - 2),
        ] {
            if let Some(frontier_cell) = map.get(test_coord.0, test_coord.1) {
                if matches_air {
                    if matches!(frontier_cell, Tile::AIR) {
                        ret.push((test_coord.0 as usize, test_coord.1 as usize));
                    }
                } else if !matches!(frontier_cell, Tile::AIR) {
                    ret.push((test_coord.0 as usize, test_coord.1 as usize));
                }
            }
        }
        ret
    }
    fn connecting_walls(&self, map: &GameWorld, cell: (usize, usize)) -> Vec<(usize, usize)> {
        self.find_neighbor_cells(map, cell, true)
    }
    /// Connect two orthogonal coordinates with Air on the given map
    fn connect_wall(&self, map: &mut GameWorld, coord1: &(usize, usize), coord2: (usize, usize)) {
        if coord1.0 != coord2.0 {
            for x in min(coord1.0, coord2.0)..=max(coord1.0, coord2.0) {
                assert_eq!(coord1.1, coord2.1);
                map.set(x, coord1.1, Tile::AIR);
            }
        } else if coord1.1 != coord2.1 {
            for y in min(coord1.1, coord2.1)..=max(coord1.1, coord2.1) {
                assert_eq!(coord1.0, coord2.0);
                map.set(coord1.0, y, Tile::AIR);
            }
        } else {
            panic!();
        }
    }
}

impl WorldGenerationAlgorithm for Maze {
    fn generate(&self) -> Result<GameWorld, WorldGenerationError> {
        match (self.width, self.height) {
            (0..=1, _) => Err(WorldGenerationError::InvalidWidth { width: self.width }),
            (_, 0..=1) => Err(WorldGenerationError::InvalidHeight {
                height: self.height,
            }),
            (w_u32, h_u32) => match usize::try_from(w_u32) {
                Ok(..) => match usize::try_from(h_u32) {
                    Ok(..) => self.generate_validated(),
                    Err(e) => Err(WorldGenerationError::HeightOutOfRange { e }),
                },
                Err(e) => Err(WorldGenerationError::WidthOutOfRange { e }),
            },
        }
    }

    fn width(&self) -> &u32 {
        &self.width
    }

    fn height(&self) -> &u32 {
        &self.height
    }
}

#[cfg(test)]
mod tests {
    use crate::tiles::Tile;
    use crate::worldgeneration::{WorldGenerationKind, build_generator};

    #[test]
    fn test_generate_2x2_map() {
        let algo = build_generator(WorldGenerationKind::Maze { color: Tile::WALL }, 2, 2, 0);
        let result = algo.generate();
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_eq!(2, map.height());
        assert_eq!(2, map.width());
    }
}
