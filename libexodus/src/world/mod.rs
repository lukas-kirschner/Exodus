use std::path::{Path, PathBuf};
use uuid::Uuid;
use crate::tiles::{Tile, TileKind};


pub mod exampleworlds;
pub mod presets;
pub mod io;

#[derive(Clone)]
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
    /// The file name of this world, if any file name has been set
    filename: Option<PathBuf>,
    /// If true, this map is clean, i.e., there are no changes that are unsaved
    clean: bool,
}

impl GameWorld {
    pub fn new(width: usize, height: usize) -> Self {
        assert! {width > 0};
        assert! {height > 0};
        Self {
            data: vec![vec![Tile::AIR; height]; width],
            playerspawn: (1, 1), // Default spawn point is (1,1)
            name: "New World".to_string(),
            author: "".to_string(),
            uuid: Uuid::new_v4(), // Generate a new random UUID
            filename: None,
            clean: true,
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
    /// Get the last set file name for this map, or None if it has been created new
    pub fn get_filename(&self) -> Option<&Path> {
        match &self.filename {
            None => None,
            Some(filename) => Some(filename.as_path())
        }
    }
    /// Set the last used file name of this map
    pub fn set_filename(&mut self, new_filename: PathBuf) {
        self.filename = Some(new_filename);
    }
    /// Remove the file name of this map
    pub fn remove_filename(&mut self) {
        self.filename = None;
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
        match &self.data[x][y].kind() {
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
            TileKind::KEY => {}
            TileKind::DOOR => {}
            TileKind::COLLECTIBLE => {}
        }
        self
    }
    ///
    /// Get the tile at the given location.
    /// If the location is outside of the map boundaries, return None instead.
    ///
    /// ```rust
    /// use libexodus::tiles::Tile;
    /// use libexodus::world::GameWorld;
    /// let mut world = GameWorld::new(2,2);
    /// world.set(1,1,Tile::SPIKES);
    /// world.set(1,0,Tile::WALL);
    /// assert_eq!(&Tile::WALL,world.get(1,0).unwrap());
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
    /// use libexodus::tiles::Tile;
    /// use libexodus::world::GameWorld;
    /// let mut world = GameWorld::new(2,2);
    /// world.set(1,1,Tile::SPIKES);
    /// world.fill(&Tile::WALL);
    /// assert_eq!(&Tile::WALL,world.get(1,1).unwrap());
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
    /// Get the dirty state of this map, i.e. if the map has unsaved changes
    /// ```rust
    /// use libexodus::world::GameWorld;
    /// let mut world = GameWorld::new(69,1337);
    /// world.set_dirty();
    /// assert!(world.is_dirty());
    /// ```
    ///
    /// ```rust
    /// use libexodus::world::GameWorld;
    /// let world = GameWorld::new(69,1337);
    /// assert!(!world.is_dirty());
    /// ```
    pub fn is_dirty(&self) -> bool {
        !self.clean
    }

    /// Set the dirty state of this map to dirty, i.e. the map has unsaved changes
    /// ```rust
    /// use libexodus::world::GameWorld;
    /// let mut world = GameWorld::new(69,1337);
    /// world.set_dirty();
    /// assert!(world.is_dirty());
    /// ```
    pub fn set_dirty(&mut self) -> &mut Self {
        self.clean = false;
        self
    }
    /// Set the dirty state of this map to clean, i.e. the map has no unsaved changes
    /// ```rust
    /// use libexodus::world::GameWorld;
    /// let mut world = GameWorld::new(69,1337);
    /// world.set_dirty();
    /// world.set_clean();
    /// assert!(!world.is_dirty());
    /// ```
    pub fn set_clean(&mut self) -> &mut Self {
        self.clean = true;
        self
    }
    /// Reset the game state.
    /// This will close opened doors.
    ///
    /// ```rust
    /// use libexodus::tiles::Tile;
    /// use libexodus::world::GameWorld;
    /// let mut world = GameWorld::new(2,1);
    /// world.set(1,0,Tile::OPENDOOR);
    /// world.reset_game_state();
    /// assert_eq!(Tile::DOOR,*world.get(1,0).unwrap())
    /// ```
    pub fn reset_game_state(&mut self) {
        for x in 0..self.width() {
            for y in 0..self.height() {
                match self.get(x as i32, y as i32).unwrap() {
                    Tile::OPENDOOR => {
                        self.set(x, y, Tile::DOOR);
                    }
                    _ => {}
                }
            }
        }
    }
}