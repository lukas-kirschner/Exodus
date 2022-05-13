#[derive(Copy, Clone)]
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
}

#[derive(Copy, Clone)]
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