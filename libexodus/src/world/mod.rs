use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::tiles;
use crate::tiles::{Tile, TileKind};


pub mod exampleworlds;
pub mod presets;
pub mod io;

#[derive(Clone, Serialize, Deserialize)]
pub struct GameWorld {
    /// A human-readable name of this world
    name: String,
    /// A human-readable name of this world
    author: String,
    /// A globally unique identifier
    uuid: Uuid,
    /// Data that contains all tiles in this world
    data: Vec<Vec<Tile>>,
    /// The coordinates of the player spawn
    playerspawn: (usize, usize),
}

impl GameWorld {
    pub fn new(width: usize, height: usize) -> Self {
        assert! {width > 0};
        assert! {height > 0};
        Self {
            data: vec![vec![tiles::air(); height]; width],
            playerspawn: (1, 1), // Default spawn point is (1,1)
            name: "New World".to_string(),
            author: "".to_string(),
            uuid: Uuid::new_v4(), // Generate a new random UUID
        }
    }
    /// Set the UUID to the given value. If the given value is not a valid UUID, do not set anything.
    pub fn set_uuid(&mut self, new_uuid: &str) -> &mut Self {
        self.uuid = Uuid::parse_str(new_uuid).unwrap_or(self.uuid);
        self
    }
    /// Get the unique ID of this map
    pub fn uuid(&self) -> String {
        self.uuid.to_string()
    }
    /// Get the name of this world
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    /// Set the name of this world
    pub fn set_name(&mut self, new_name: &str) -> &mut Self {
        self.name = new_name.to_string();
        self
    }
    /// Get the author name of this world
    pub fn get_author(&self) -> &str {
        self.author.as_str()
    }
    /// Set the author name of this world
    pub fn set_author(&mut self, new_name: &str) -> &mut Self {
        self.author = new_name.to_string();
        self
    }
    ///
    /// Set the tile at the given coordinate to the given value.
    pub fn set(&mut self, x: usize, y: usize, tile: Tile) -> &mut Self {
        self.data[x][y] = tile;
        match &self.data[x][y].kind {
            TileKind::AIR => {}
            TileKind::SOLID => {}
            TileKind::DEADLY { .. } => {
                //TODO
            }
            TileKind::SPECIAL => {
                //TODO
            }
            TileKind::PLAYERSPAWN => {
                self.playerspawn = (x, y);
            }
            TileKind::COIN => {}
            TileKind::LADDER => {}
        }
        self
    }
    ///
    /// Get the tile at the given location.
    /// If the location is outside of the map boundaries, return None instead.
    ///
    /// ```rust
    /// use libexodus::tiles;
    /// use libexodus::world::GameWorld;
    /// let mut world = GameWorld::new(2,2);
    /// world.set(1,1,tiles::spikes());
    /// world.set(1,0,tiles::wall());
    /// assert_eq!(&tiles::wall,world.get(1,0).unwrap());
    /// assert!(world.get(2,0).is_none());
    /// assert!(world.get(0,-1).is_none());
    /// ```
    pub fn get(&self, x: i32, y: i32) -> Option<&Tile> {
        if x < 0 || y < 0 {
            return None;
        }
        self.data.get(x as usize)?.get(y as usize)
    }

    ///
    /// Fill the whole map with the given tile and delete everything else.
    ///
    /// ```rust
    /// use libexodus::tiles;
    /// use libexodus::world::GameWorld;
    /// let mut world = GameWorld::new(2,2);
    /// world.set(1,1,tiles::spikes());
    /// world.fill(&tiles::wall());
    /// assert_eq!(&tiles::wall,world.get(1,1).unwrap());
    /// ```
    pub fn fill(&mut self, tile: &Tile) -> &mut Self {
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                self.set(i, j, tile.clone());
            }
        }
        self
    }

    ///
    /// Get the width of this world.
    ///
    /// ```rust
    /// use libexodus::world::GameWorld;
    /// let world = GameWorld::new(69,1337);
    /// assert_eq!(69, world.width());
    /// ```
    pub fn width(&self) -> usize {
        self.data.len()
    }

    ///
    /// Get the height of this world.
    ///
    /// ```rust
    /// use libexodus::world::GameWorld;
    /// let world = GameWorld::new(69,1337);
    /// assert_eq!(1337, world.height());
    /// ```
    pub fn height(&self) -> usize {
        self.data[0].len()
    }

    pub fn player_spawn(&self) -> (usize, usize) {
        self.playerspawn
    }
}