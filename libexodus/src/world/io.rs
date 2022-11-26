use std::{fs, io};
use bincode;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use crate::tiles::Tile;
use crate::world::GameWorld;
use strum::IntoEnumIterator;

pub const CURRENT_MAP_VERSION: u8 = 0x01;

///
/// This file contains code used to manipulate physical data representing game worlds.
///
/// Definition of the binary Map File Format:
///
/// 1. Magic Bytes 0x 45 78 6f 64 75 73 4d 61 70
///
/// 2. Map Format Version (current version: 0x01)
///
/// 3. Length of Name, Name, encoded with bincode crate
///
/// 4. Length of Author, Author, encoded with bincode crate
///
/// 5. Cached UUID as 16-byte octet
///
/// 6. Map Width, Map Height
///
/// 7. Map Tiles, each row is appended from bottom to top, i.e. starting at (0,0),(1,0),(2,0),...
///
/// The cached UUID is used for checksum validation, and will be re-calculated on map load.
/// If it does not match, the load will fail.

impl GameWorld {
    /// Load a map from the given file.
    pub fn load_from_file(path: &Path) -> std::io::Result<Self> {
        let mut file = OpenOptions::new().read(true).open(path)?;
        let mut buf = BufReader::new(file);
        let mut ret: GameWorld = GameWorld {
            name: "".to_string(),
            author: "".to_string(),
            uuid: Default::default(),
            data: vec![],
            playerspawn: (0, 0),
            filename: Some(path.to_path_buf()),
            clean: true,
        };
        ret.parse(&mut buf)?;
        Ok(ret)
    }
    /// Save the map to the given file.
    pub fn save_to_file(&self, path: &Path) -> std::io::Result<()> {
        let mut file: File = OpenOptions::new().create(true).write(true).open(path)?;
        let mut buf = BufWriter::new(file);
        self.serialize(&mut buf)?;
        println!("Successfully saved map file {}", path.to_str().unwrap_or("<NONE>"));
        Ok(())
    }
}

/// Implementation for Serializer
impl GameWorld {
    fn serialize<T: Write>(&self, file: &mut T) -> std::io::Result<()> {
        // Write magic bytes
        file.write(&[0x45, 0x78, 0x6f, 0x64, 0x75, 0x73, 0x4d, 0x61, 0x70])?;

        // Write Map Version
        file.write(&[CURRENT_MAP_VERSION])?;

        // Write Map Name
        let name_b = bincode::serialize(&self.name).map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;
        file.write(&name_b)?;

        // Write Map Author
        let author_b = bincode::serialize(&self.author).map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;
        file.write(&author_b)?;

        // Write cached UUID
        file.write(self.uuid.as_bytes())?;

        // Write Map Width and Height
        let width_b = bincode::serialize(&self.width()).map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;
        file.write(&width_b)?;
        let height_b = bincode::serialize(&self.height()).map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;
        file.write(&height_b)?;

        // Write Map Tiles
        for x in 0..self.width() {
            for y in 0..self.height() {
                file.write(&[self.get(x as i32, y as i32).unwrap().to_bytes()])?;
            }
        }

        Ok(())
    }
}

/// Implementation for Parser
impl GameWorld {
    fn parse<T: Read>(&mut self, file: &mut T) -> std::io::Result<()> {
        Err(io::Error::new(ErrorKind::Other, "Not Implemented"))
    }
}

impl Tile {
    /// Serialize a map tile into a byte.
    ///
    /// Since this is a fixed value for each tile defined in the map format, we explicitly define it here.
    pub const fn to_bytes(&self) -> u8 {
        match *self {
            Tile::AIR => 0x00,
            Tile::WALL => 0x01,
            Tile::PLAYERSPAWN => 0x10,
            Tile::DOOR => 0x20,
            Tile::OPENDOOR => 0x21,
            Tile::COIN => 0x30,
            Tile::KEY => 0x31,
            Tile::LADDER => 0x22,
            Tile::SPIKES => 0x40,
            Tile::SPIKESALT => 0x41,
            Tile::SPIKESSLOPED => 0x42,
            Tile::WALLSPIKESL => 0x50,
            Tile::WALLSPIKESR => 0x51,
            Tile::WALLSPIKESLR => 0x52,
            Tile::WALLSPIKESB => 0x53,
            Tile::WALLSPIKESLB => 0x54,
            Tile::WALLSPIKESRB => 0x55,
            Tile::WALLSPIKESTB => 0x56,
            Tile::WALLSPIKESRLTB => 0x57,
            Tile::WALLSPIKESRTB => 0x58,
            Tile::WALLSPIKESLTB => 0x59,
            Tile::ARROWRIGHT => 0x32,
            Tile::ARROWLEFT => 0x33,
            Tile::ARROWUP => 0x34,
            Tile::ARROWDOWN => 0x35,
        }
    }

    pub const fn from_bytes(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Tile::AIR),
            0x01 => Some(Tile::WALL),
            0x10 => Some(Tile::PLAYERSPAWN),
            0x20 => Some(Tile::DOOR),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bidirectional_serialization_for_tiles() {
        for tile in Tile::iter() {
            let reference: &Tile = &tile;
            let actual = Tile::from_bytes(reference.to_bytes());
            assert!(actual.is_some(), "Deserializing Tile {} (0x{:02X}) resulted in an error!", reference.to_string(), reference.to_bytes());
            let actual = actual.unwrap();
            assert_eq!(*reference, actual, "The Tile {} (0x{:02X}) deserialized into tile {} (0x{:02X}) !",
                       reference.to_string(), reference.to_bytes(), actual.to_string(), actual.to_bytes(), );
        }
    }
}