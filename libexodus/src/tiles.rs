use std::fmt;
use std::fmt::Formatter;
use strum_macros::EnumIter;
use serde::{Serialize, Deserialize};
use crate::directions::FromDirection;
use crate::directions::FromDirection::{FROMEAST, FROMNORTH, FROMSOUTH, FROMWEST};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TileKind {
    ///
    /// A tile that cannot interact with the player in any way
    AIR,
    ///
    /// A solid tile
    SOLID,
    ///
    /// A tile that kills the player on impact
    DEADLY { from: Vec<FromDirection> },
    ///
    /// A special tile that the player can interact with
    SPECIAL,
    ///
    ///
    PLAYERSPAWN,
    ///
    /// A collectible coin
    COIN,
    ///
    /// A ladder
    LADDER,
}

pub type AtlasIndex = usize;

// Tiles Definitions
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, EnumIter)]
pub enum Tile {
    /// An air tile without a texture
    AIR,
    /// A tile of Wall, a solid block that cannot be destroyed
    WALL,
    /// The position where the player spawns
    PLAYERSPAWN,
    /// A collectible coin
    COIN,
    /// A ladder
    LADDER,
    /// Spikes that sit on the ground and point up
    SPIKES,
    /// Spikes that sit on the ground and point up, alternative texture
    SPIKESALT,
    /// Spikes that sit on a slope and point up, only deadly if touched from above.
    SPIKESSLOPED,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESL,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESR,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESLR,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESB,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESLB,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESRB,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESTB,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESRLTB,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESRTB,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESLTB,
}

impl Tile {
    /// Get the tile kind of the given tile
    pub fn kind(&self) -> TileKind {
        match self {
            Tile::AIR => TileKind::AIR,
            Tile::WALL => TileKind::SOLID,
            Tile::PLAYERSPAWN => TileKind::PLAYERSPAWN,
            Tile::COIN => TileKind::COIN,
            Tile::LADDER => TileKind::LADDER,
            Tile::SPIKES => TileKind::DEADLY { from: vec![FROMNORTH, FROMSOUTH, FROMEAST, FROMWEST] },
            Tile::SPIKESALT => TileKind::DEADLY { from: vec![FROMNORTH, FROMSOUTH, FROMEAST, FROMWEST] },
            Tile::SPIKESSLOPED => TileKind::DEADLY { from: vec![FROMNORTH] },
            Tile::WALLSPIKESL => TileKind::DEADLY { from: vec![FROMWEST] },
            Tile::WALLSPIKESR => TileKind::DEADLY { from: vec![FROMEAST] },
            Tile::WALLSPIKESLR => TileKind::DEADLY { from: vec![FROMEAST, FROMWEST] },
            Tile::WALLSPIKESB => TileKind::DEADLY { from: vec![FROMSOUTH] },
            Tile::WALLSPIKESLB => TileKind::DEADLY { from: vec![FROMWEST, FROMSOUTH] },
            Tile::WALLSPIKESRB => TileKind::DEADLY { from: vec![FROMEAST, FROMSOUTH] },
            Tile::WALLSPIKESTB => TileKind::DEADLY { from: vec![FROMNORTH, FROMSOUTH] },
            Tile::WALLSPIKESRLTB => TileKind::DEADLY { from: vec![FROMNORTH, FROMSOUTH, FROMEAST, FROMWEST] },
            Tile::WALLSPIKESRTB => TileKind::DEADLY { from: vec![FROMNORTH, FROMSOUTH, FROMEAST] },
            Tile::WALLSPIKESLTB => TileKind::DEADLY { from: vec![FROMNORTH, FROMSOUTH, FROMWEST] },
        }
    }
    pub fn atlas_index(&self) -> Option<AtlasIndex> {
        return match self {
            Tile::AIR => None,
            Tile::WALL => Some(58),
            Tile::PLAYERSPAWN => None,
            Tile::COIN => Some(217),
            Tile::LADDER => Some(220),
            Tile::SPIKES => Some(228),
            Tile::SPIKESALT => Some(227),
            Tile::SPIKESSLOPED => Some(250),
            Tile::WALLSPIKESL => Some(244),
            Tile::WALLSPIKESR => Some(242),
            Tile::WALLSPIKESLR => Some(243),
            Tile::WALLSPIKESB => Some(233),
            Tile::WALLSPIKESLB => Some(232),
            Tile::WALLSPIKESRB => Some(234),
            Tile::WALLSPIKESTB => Some(238),
            Tile::WALLSPIKESRLTB => Some(254),
            Tile::WALLSPIKESLTB => Some(237),
            Tile::WALLSPIKESRTB => Some(239),
        };
    }
    pub fn can_collide_from(&self, from_direction: &FromDirection) -> bool {
        match self.kind() {
            TileKind::AIR => { false }
            TileKind::SOLID => { true }
            TileKind::DEADLY { from } => { !from.iter().any(|fromdir| *fromdir == *from_direction) }
            TileKind::SPECIAL => { false }
            TileKind::PLAYERSPAWN => { false }
            TileKind::COIN => { false }
            TileKind::LADDER => { false }
        }
    }
    pub fn is_deadly_from(&self, from_direction: &FromDirection) -> bool {
        match self.kind() {
            TileKind::AIR => { false }
            TileKind::SOLID => { false }
            TileKind::DEADLY { from } => { from.iter().any(|fromdir| *fromdir == *from_direction) }
            TileKind::SPECIAL => { false }
            TileKind::PLAYERSPAWN => { false }
            TileKind::COIN => { false }
            TileKind::LADDER => { false }
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Tile::AIR => "Air",
            Tile::WALL => "Wall",
            Tile::PLAYERSPAWN => "Player Spawn",
            Tile::COIN => "Coin",
            Tile::LADDER => "Ladder",
            Tile::SPIKES => "Spikes",
            Tile::SPIKESALT => "Spikes (Alt Texture)",
            Tile::SPIKESSLOPED => "Sloped Spikes",
            Tile::WALLSPIKESL => "L Wall Spikes",
            Tile::WALLSPIKESR => "R Wall Spikes",
            Tile::WALLSPIKESLR => "LR Wall Spikes",
            Tile::WALLSPIKESB => "B Wall Spikes",
            Tile::WALLSPIKESLB => "LB Wall Spikes",
            Tile::WALLSPIKESRB => "RB Wall Spikes",
            Tile::WALLSPIKESTB => "TB Wall Spikes",
            Tile::WALLSPIKESRLTB => "LTB Wall Spikes",
            Tile::WALLSPIKESRTB => "RTB Wall Spikes",
            Tile::WALLSPIKESLTB => "LTB Wall Spikes",
        })
    }
}