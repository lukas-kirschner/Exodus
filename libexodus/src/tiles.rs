use crate::directions::FromDirection;
use crate::directions::FromDirection::{FROMEAST, FROMNORTH, FROMSOUTH, FROMWEST};

#[derive(Clone, Debug, Eq, PartialEq)]
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tile {
    ///
    /// The texture index in the tile atlas that is used as sprite for this tile
    pub atlas_index: Option<usize>,
    ///
    /// The kind of the tile
    pub kind: TileKind,
}

impl Tile {
    pub fn can_collide_from(&self, from_direction: &FromDirection) -> bool {
        match &self.kind {
            TileKind::AIR => { false }
            TileKind::SOLID => { true }
            TileKind::DEADLY { from } => { !from.iter().any(|fromdir| *fromdir == *from_direction) }
            TileKind::SPECIAL => { false }
            TileKind::PLAYERSPAWN => { false }
            TileKind::COIN => { false }
        }
    }
    pub fn is_deadly_from(&self, from_direction: &FromDirection) -> bool {
        match &self.kind {
            TileKind::AIR => { false }
            TileKind::SOLID => { false }
            TileKind::DEADLY { from } => { from.iter().any(|fromdir| *fromdir == *from_direction) }
            TileKind::SPECIAL => { false }
            TileKind::PLAYERSPAWN => { false }
            TileKind::COIN => { false }
        }
    }
}

// Tiles Definitions

///
/// An air tile without a texture
pub fn AIR() -> Tile {
    Tile {
        atlas_index: None,
        kind: TileKind::AIR,
    }
}

///
/// A tile of Wall, a solid block that cannot be destroyed
pub fn WALL() -> Tile {
    Tile {
        atlas_index: Some(58),
        kind: TileKind::SOLID,
    }
}

///
/// Spikes that sit on the ground and point up
pub fn SPIKES() -> Tile {
    Tile {
        atlas_index: Some(228),
        kind: TileKind::DEADLY { from: vec![FROMNORTH, FROMSOUTH, FROMEAST, FROMWEST] },
    }
}

///
/// Spikes that sit on the ground and point up, alternative texture
pub fn SPIKES_ALTERNATIVE_A() -> Tile {
    Tile {
        atlas_index: Some(227),
        kind: TileKind::DEADLY { from: vec![FROMNORTH, FROMSOUTH, FROMEAST, FROMWEST] },
    }
}

///
/// Spikes that sit on a floating platform and are deadly from all sides
pub fn SPIKES_PLATFORM() -> Tile {
    Tile {
        atlas_index: Some(224),
        kind: TileKind::DEADLY { from: vec![FROMNORTH, FROMSOUTH, FROMEAST, FROMWEST] },
    }
}

///
/// Spikes that sit on a slope and point up, only deadly if touched from above.
pub fn SLOPED_SPIKES() -> Tile {
    Tile {
        atlas_index: Some(250),
        kind: TileKind::DEADLY { from: vec![FROMNORTH] },
    }
}

///
/// Spikes that sit on a slope and point up, only deadly if touched from above.
pub fn PLAYER_SPAWN() -> Tile {
    Tile {
        atlas_index: None,
        kind: TileKind::PLAYERSPAWN,
    }
}

///
/// A collectible coin
pub fn COIN() -> Tile {
    Tile {
        atlas_index: Some(217),
        kind: TileKind::COIN,
    }
}