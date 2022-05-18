#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TileKind {
    ///
    /// A tile that cannot interact with the player in any way
    AIR,
    ///
    /// A solid tile
    SOLID,
    ///
    /// A tile that kills the player on impact
    DEADLY,
    ///
    /// A special tile that the player can interact with
    SPECIAL,
    ///
    ///
    PLAYERSPAWN,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tile {
    ///
    /// The texture index in the tile atlas that is used as sprite for this tile
    pub atlas_index: Option<usize>,
    ///
    /// The kind of the tile
    pub kind: TileKind,
}

// Tiles Definitions

///
/// An air tile without a texture
pub static AIR: Tile = Tile {
    atlas_index: None,
    kind: TileKind::AIR,
};

///
/// A tile of Wall, a solid block that cannot be destroyed
pub static WALL: Tile = Tile {
    atlas_index: Some(58),
    kind: TileKind::SOLID,
};

///
/// Spikes that sit on the ground and point up
pub static SPIKES: Tile = Tile {
    atlas_index: Some(228),
    kind: TileKind::DEADLY,
};

///
/// Spikes that sit on a slope and point up, only deadly if touched from above.
pub static SLOPED_SPIKES: Tile = Tile {
    atlas_index: Some(250),
    kind: TileKind::DEADLY,
};

///
/// Spikes that sit on a slope and point up, only deadly if touched from above.
pub static PLAYER_SPAWN: Tile = Tile {
    atlas_index: None,
    kind: TileKind::PLAYERSPAWN,
};