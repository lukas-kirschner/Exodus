use crate::tiles::{Tile, TileKind};
use std::path::{Path, PathBuf};

pub mod exampleworlds;
pub mod hash;
pub mod io;
pub mod io_error;
pub mod presets;

#[derive(Clone)]
pub struct GameWorld {
    /// A human-readable name of this world
    name: String,
    /// A human-readable name of this world
    author: String,
    /// A globally unique identifier (hash) of the map
    hash: [u8; 32],
    /// Data that contains all tiles in this world
    data: Vec<Vec<Tile>>,
    /// The coordinates of the player spawn
    playerspawn: (usize, usize),
    /// The file name of this world, if any file name has been set
    filename: Option<PathBuf>,
    /// If true, this map is clean, i.e., there are no changes that are unsaved
    clean: bool,
    /// All messages that are stored in this map
    messages: Vec<String>,
}

impl Default for GameWorld {
    fn default() -> Self {
        GameWorld {
            name: "".to_string(),
            author: "".to_string(),
            hash: [0u8; 32],
            data: vec![],
            playerspawn: (0, 0),
            filename: None,
            clean: true,
            messages: vec![],
        }
    }
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
            hash: [0u8; 32], // Generate a zeroed default hash
            filename: None,
            clean: true,
            messages: vec![], // No messages
        }
    }
    /// Get the unique ID of this map as hex-string representation
    pub fn hash_str(&self) -> String {
        let mut ret = String::new();
        for b in &self.hash {
            ret.push_str(format!("{:02X}", *b).as_str());
        }
        ret
    }
    /// Get the unique ID of this map as byte slice
    pub fn hash(&self) -> &[u8; 32] {
        &self.hash
    }
    /// Get the name of this world
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    /// Get the last set file name for this map, or None if it has been created new
    pub fn get_filename(&self) -> Option<&Path> {
        match &self.filename {
            None => None,
            Some(filename) => Some(filename.as_path()),
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
            TileKind::AIR => {},
            TileKind::SOLID => {},
            TileKind::DEADLY { .. } => {
                //TODO
            },
            TileKind::SPECIAL { interaction: _ } => {
                //TODO
            },
            TileKind::PLAYERSPAWN => {
                self.playerspawn = (x, y);
            },
            TileKind::COIN => {},
            TileKind::LADDER => {},
            TileKind::KEY => {},
            TileKind::DOOR => {},
            TileKind::COLLECTIBLE => {},
            TileKind::EXIT => {},
        }
        self
    }
    ///
    /// Set the tile at the given coordinate to a new message tile showing the given message.
    /// ```rust
    /// use libexodus::tiles::Tile;
    /// use libexodus::world::GameWorld;
    /// let mut world = GameWorld::new(2,2);
    /// assert!(world.get_message(0).is_none());
    /// world.set_message_tile(1,1,"Hello World".to_string());
    /// assert_eq!(world.get_message(0).unwrap(), "Hello World");
    /// assert!(matches!(world.get(1,1).unwrap(), Tile::MESSAGE{message_id: 0}));
    /// ```
    pub fn set_message_tile(&mut self, x: usize, y: usize, message: String) -> &mut Self {
        self.set(
            x,
            y,
            Tile::MESSAGE {
                message_id: self.messages.len(),
            },
        );
        self.messages.push(message);
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
    /// Get the message with the given message ID or None, if it does not exist.
    pub fn get_message(&self, message_id: usize) -> Option<&str> {
        if message_id > self.messages.len() {
            None
        } else {
            self.messages.get(message_id).map(|msg| msg.as_str())
        }
    }

    /// Set the message with the given message ID to a new value.
    /// ```rust
    /// use libexodus::tiles::Tile;
    /// use libexodus::world::GameWorld;
    /// let mut world = GameWorld::new(2,2);
    /// assert!(world.get_message(0).is_none());
    /// world.set_message_tile(1,1,"Hello World".to_string());
    /// assert_eq!(world.get_message(0).unwrap(), "Hello World");
    /// world.set_message(0,"Goodbye World".to_string()).unwrap();
    /// assert_eq!(world.get_message(0).unwrap(), "Goodbye World");
    /// ```
    pub fn set_message(&mut self, message_id: usize, message: String) -> Result<(), ()> {
        if 0 <= message_id && message_id < self.messages.len() {
            self.messages[message_id] = message;
            Ok(())
        } else {
            Err(())
        }
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
        if self.width() > 0 {
            self.data[0].len()
        } else {
            0
        }
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
                if self.get(x as i32, y as i32).unwrap() == &Tile::OPENDOOR {
                    self.set(x, y, Tile::DOOR);
                }
            }
        }
    }
}
