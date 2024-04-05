mod border;
mod empty;
mod filled;

use crate::tiles::Tile;
use crate::world::GameWorld;
use crate::worldgeneration::border::Border;
use crate::worldgeneration::empty::Empty;
use crate::worldgeneration::filled::Filled;
use std::num::TryFromIntError;
use strum_macros::EnumIter;

#[derive(Eq, PartialEq, Default, EnumIter, Clone)]
pub enum WorldGenerationKind {
    #[default]
    Empty,
    Border {
        width: u32,
        color: Tile,
    },
    Filled {
        color: Tile,
    },
    Labyrinth {
        color: Tile,
    },
}

#[derive(Eq, PartialEq, Default, EnumIter, Copy, Clone)]
pub enum WorldSize {
    #[default]
    Classic5mx,
    Small,
    Medium,
    Large,
    Huge,
    Custom {
        /// The map width
        width: u32,
        /// The map height
        height: u32,
    },
}

const SIZE_5MX: (u32, u32) = (35, 15);
const SIZE_SMALL: (u32, u32) = (22, 10);
const SIZE_MEDIUM: (u32, u32) = (30, 14);
const SIZE_LARGE: (u32, u32) = (38, 16);
const SIZE_HUGE: (u32, u32) = (50, 20);

impl WorldSize {
    pub fn width(&self) -> u32 {
        match self {
            WorldSize::Classic5mx => SIZE_5MX.0,
            WorldSize::Small => SIZE_SMALL.0,
            WorldSize::Medium => SIZE_MEDIUM.0,
            WorldSize::Large => SIZE_LARGE.0,
            WorldSize::Huge => SIZE_HUGE.0,
            WorldSize::Custom { width, height: _ } => *width,
        }
    }
    pub fn height(&self) -> u32 {
        match self {
            WorldSize::Classic5mx => SIZE_5MX.1,
            WorldSize::Small => SIZE_SMALL.1,
            WorldSize::Medium => SIZE_MEDIUM.1,
            WorldSize::Large => SIZE_LARGE.1,
            WorldSize::Huge => SIZE_HUGE.1,
            WorldSize::Custom { width: _, height } => *height,
        }
    }
}

impl WorldGenerationKind {
    /// Whether randomness, i.e., a seed is involved when generating a map using the given MapGenerationKind
    pub fn is_seeded(&self) -> bool {
        match self {
            WorldGenerationKind::Empty => false,
            WorldGenerationKind::Border { .. } => false,
            WorldGenerationKind::Filled { .. } => false,
            WorldGenerationKind::Labyrinth { .. } => true,
        }
    }
}

/// An error that might occur during a map generation
#[derive(Debug)]
pub enum WorldGenerationError {
    InvalidWidth { width: u32 },
    InvalidHeight { height: u32 },
    WidthOutOfRange { e: TryFromIntError },
    HeightOutOfRange { e: TryFromIntError },
    InvalidBorderWidth { border_width: u32 },
}

pub trait WorldGenerationAlgorithm: Send {
    /// Generate the world using this algorithm.
    /// This may take a long time, depending on the algorithm.
    fn generate(&self) -> Result<GameWorld, WorldGenerationError>;
    /// Get the width of the generated map
    fn width(&self) -> &u32;
    /// Get the height of the generated map
    fn height(&self) -> &u32;
}

/// Build the generator matching the given parameters
pub fn build_generator(
    parameters: WorldGenerationKind,
    width: u32,
    height: u32,
) -> Box<dyn WorldGenerationAlgorithm> {
    match parameters {
        WorldGenerationKind::Empty => Box::new(Empty { width, height }),
        WorldGenerationKind::Border {
            width: border_width,
            color,
        } => Box::new(Border {
            width,
            height,
            color,
            border_width,
        }),
        WorldGenerationKind::Filled { color } => Box::new(Filled {
            color,
            width,
            height,
        }),
        WorldGenerationKind::Labyrinth { .. } => Box::new(Empty { width, height }),
    }
}

#[cfg(test)]
mod tests {
    use crate::tiles::{TeleportId, Tile};
    use crate::world::GameWorld;
    use crate::worldgeneration::{build_generator, WorldGenerationKind};

    /// Assert that the given map's content matches the given string.
    /// The string must be given as 2D-Map, with "W" being a Wall tile, "N" being a
    /// Nature Wall Tile and " " being Air.
    pub fn assert_map_content_matches(expected: &str, map: &GameWorld) {
        print_map(map);
        let lines: Vec<&str> = expected.split("\n").filter(|l| l.len() > 0).collect();
        let expected_h = lines.len();
        let expected_w = lines[0].len();
        assert_eq!(expected_h, map.height());
        assert_eq!(expected_w, map.width());
        for (y, line) in lines.iter().enumerate() {
            assert_eq!(expected_w, line.len());
            for (x, tile) in line.chars().enumerate() {
                let my = (map.height() - y - 1) as i32;
                assert_eq!(
                    &match tile {
                        'W' => Tile::WALL,
                        'N' => Tile::WALLNATURE,
                        ' ' => Tile::AIR,
                        e => panic!("Unrecognized tile: {}", e),
                    },
                    map.get(x as i32, my).unwrap(),
                    "Expected {} tile at position {},{} - got: {}",
                    tile,
                    x,
                    my,
                    map.get(x as i32, my).unwrap()
                );
            }
        }
    }
    /// Debug-Print the given map's content to stdout
    pub fn print_map(map: &GameWorld) {
        for y in map.height() - 1..=0 {
            for x in 0..map.width() {
                print!(
                    "{}",
                    match map.get(x as i32, y as i32) {
                        Some(Tile::WALL) => 'W',
                        Some(Tile::WALLNATURE) => 'N',
                        Some(Tile::AIR) => ' ',
                        Some(t) => panic!("Unsupported tile found in map: {} at {},{}", t, x, y),
                        None => panic!("Illegal Coordinate {},{}", x, y),
                    }
                );
            }
            print!("\n");
        }
    }
}
