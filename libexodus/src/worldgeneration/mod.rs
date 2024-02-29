use crate::tiles::Tile;
use strum_macros::EnumIter;

#[derive(Eq, PartialEq, Default)]
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
