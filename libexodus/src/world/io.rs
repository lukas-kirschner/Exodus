use crate::exodus_serializable::ExodusSerializable;
use crate::tiles::{InteractionKind, TeleportId, Tile};
use crate::tilesets::Tileset;
use crate::world::hash::RecomputeHashResult;
use crate::world::io_error::GameWorldParseError;
use crate::world::GameWorld;
use bincode;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

pub(crate) const MAGICBYTES: [u8; 9] = [0x45, 0x78, 0x6f, 0x64, 0x75, 0x73, 0x4d, 0x61, 0x70];
pub(crate) const MAX_MAP_WIDTH: usize = 1024;
pub(crate) const MAX_MAP_HEIGHT: usize = 1024;
pub(crate) const HASH_LENGTH: usize = 32;

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
/// 8. All messages in correct order
///
/// The cached UUID is used for checksum validation, and will be re-calculated on map load.
/// If it does not match, the load will fail.
/// If there is extra data at the end of a map, it will be ignored and discarded when the map
/// is loaded and saved again.

impl GameWorld {
    /// Load a map from the given file.
    pub fn load_from_file(path: &Path) -> Result<GameWorld, GameWorldParseError> {
        let file = OpenOptions::new().read(true).open(path)?;
        let mut buf = BufReader::new(file);
        let mut ret: GameWorld = GameWorld {
            name: "".to_string(),
            author: "".to_string(),
            hash: Default::default(),
            data: vec![],
            playerspawn: (0, 0),
            filename: Some(path.to_path_buf()),
            clean: true,
            messages: vec![],
            forced_tileset: None,
            teleport_metadata: Default::default(),
        };
        ret.parse(&mut buf)?;
        Ok(ret)
    }
    /// Save the map to the given file. The hash MUST be recomputed before saving the map - else, the next load will fail!
    pub fn save_to_file(&self, path: &Path) -> Result<(), GameWorldParseError> {
        let file: File = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;
        let mut buf = BufWriter::new(file);
        self.serialize(&mut buf)?;
        println!(
            "Successfully saved map file {}",
            path.to_str().unwrap_or("<NONE>")
        );
        Ok(())
    }
}

/// Implementation for Serializer
impl ExodusSerializable for GameWorld {
    const CURRENT_VERSION: u8 = 0x02;
    type ParseError = GameWorldParseError;
    fn serialize<T: Write>(&self, file: &mut T) -> Result<(), GameWorldParseError> {
        // Write magic bytes
        file.write_all(&MAGICBYTES)?;

        // Write Map Version
        file.write_all(&[Self::CURRENT_VERSION])?;

        // Write Map Name
        let name_b = bincode::serialize(&self.name)?;
        file.write_all(&name_b)?;

        // Write Map Author
        let author_b = bincode::serialize(&self.author)?;
        file.write_all(&author_b)?;

        // Write cached UUID
        file.write_all(&self.hash)?;

        self.serialize_world_content(file)?;

        // Write forced tileset. The tileset should not be considered when
        // calculating the hash of a map.
        self.serialize_tileset(file)?;
        Ok(())
    }
    fn parse<T: Read>(&mut self, file: &mut T) -> Result<(), GameWorldParseError> {
        // Parse Magic Bytes
        let mut buf: [u8; MAGICBYTES.len()] = [0; MAGICBYTES.len()];
        file.read_exact(&mut buf)?;
        if buf != MAGICBYTES {
            return Err(GameWorldParseError::InvalidMagicBytes {
                expected: MAGICBYTES,
                actual: buf,
            });
        }

        // Parse Map Format
        let mut buf: [u8; 1] = [0; 1];
        file.read_exact(&mut buf)?;
        match buf[0] {
            0x01 => self.parse_v1(file),
            Self::CURRENT_VERSION => self.parse_current_version(file),
            // Add older versions here
            _ => {
                return Err(Self::ParseError::InvalidVersion {
                    invalid_version: buf[0],
                })
            },
        }?;
        match self.recompute_hash() {
            RecomputeHashResult::SAME => Ok(()),
            RecomputeHashResult::CHANGED { old_hash } => Err(GameWorldParseError::HashMismatch {
                expected: self.hash,
                actual: old_hash,
            }),
            RecomputeHashResult::ERROR { error } => Err(error),
        }
    }

    /// Parse a map with the current version.
    /// The file read position must be already behind the version byte
    fn parse_current_version<T: Read>(&mut self, file: &mut T) -> Result<(), GameWorldParseError> {
        // Parse Map Name
        let name = self.parse_current_version_string(file)?;
        self.set_name(name.as_str());

        // Parse Map Author
        let author = self.parse_current_version_string(file)?;
        self.set_author(author.as_str());

        let hash = self.parse_current_version_uuid(file)?;
        self.hash = hash;

        // Parse Map Width and Map Height
        let map_width: usize = bincode::deserialize_from::<&mut T, usize>(file)?;
        let map_height: usize = bincode::deserialize_from::<&mut T, usize>(file)?;
        if map_width > MAX_MAP_WIDTH {
            return Err(GameWorldParseError::InvalidMapWidth {
                max_width: MAX_MAP_WIDTH,
                actual_width: map_width,
            });
        }
        if map_height > MAX_MAP_HEIGHT {
            return Err(GameWorldParseError::InvalidMapHeight {
                max_height: MAX_MAP_HEIGHT,
                actual_height: map_height,
            });
        }
        self.data = vec![vec![Tile::AIR; map_height]; map_width];
        assert_eq!(map_width, self.width());
        assert_eq!(map_height, self.height());

        // Parse actual map content
        // The message ID of the current message tile
        let mut current_message_id = 0usize;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let mut buf = [0u8; 1];
                file.read_exact(&mut buf).map_err(|e| {
                    GameWorldParseError::UnexpectedEndOfTileData {
                        io_error: e,
                        position: (y * x) + x,
                    }
                })?;
                let mut tile = Tile::from_bytes(buf[0])
                    .ok_or(GameWorldParseError::InvalidTile { tile_bytes: buf[0] })?;
                // Assign the current message ID to message tiles
                if let Tile::MESSAGE { .. } = tile {
                    tile = Tile::MESSAGE {
                        message_id: current_message_id,
                    };
                    current_message_id += 1;
                }
                self.set(x, y, tile);
            }
        }
        self.parse_messages(file, current_message_id)?;
        self.parse_tileset(file)?;

        Ok(())
    }
}
impl GameWorld {
    fn parse_messages<T: Read>(
        &mut self,
        file: &mut T,
        expected_len: usize,
    ) -> Result<(), GameWorldParseError> {
        let actual_len = bincode::deserialize_from::<&mut T, u32>(file)?;
        if actual_len != expected_len as u32 {
            return Err(GameWorldParseError::MissingMessageString {
                expected_length: expected_len as u32,
                actual_length: actual_len,
            });
        }
        for _ in 0..actual_len {
            let message = self.parse_current_version_string(file)?;
            self.messages.push(message);
        }
        Ok(())
    }
    fn parse_tileset<T: Read>(&mut self, file: &mut T) -> Result<(), GameWorldParseError> {
        let mut opt = [0u8; 1];
        file.read_exact(&mut opt)?;
        if opt[0] == 0x00 {
            self.forced_tileset = None;
        } else {
            let mut tileset_buf = [0u8; 1];
            file.read_exact(&mut tileset_buf)?;
            let Some(tileset) = Tileset::from_bytes(tileset_buf[0]) else {
                return Err(GameWorldParseError::InvalidTileset {
                    tileset_bytes: tileset_buf[0],
                });
            };
            self.forced_tileset = Some(tileset);
        }
        Ok(())
    }
}

// Code to parse a map with the current version.
impl GameWorld {
    /// Parse a string
    fn parse_current_version_string<T: Read>(
        &mut self,
        file: &mut T,
    ) -> Result<String, GameWorldParseError> {
        let string_value: String = bincode::deserialize_from(file)?;
        Ok(string_value)
    }
    /// Parse a UUID
    fn parse_current_version_uuid<T: Read>(
        &mut self,
        file: &mut T,
    ) -> Result<[u8; HASH_LENGTH], GameWorldParseError> {
        let mut buf = [0u8; HASH_LENGTH];
        file.read_exact(&mut buf)?;
        Ok(buf)
    }
    /// Serialize this GameWorld's content. Everything that is serialized here will be considered
    /// when computing the hash value of the map.
    pub(crate) fn serialize_world_content<T: Write>(
        &self,
        file: &mut T,
    ) -> Result<(), GameWorldParseError> {
        // Write Map Width and Height
        let width_b = bincode::serialize(&self.width())?;
        file.write_all(&width_b)?;
        let height_b = bincode::serialize(&self.height())?;
        file.write_all(&height_b)?;

        // Write Map Tiles
        // All message IDs in correct order
        let mut message_ids: Vec<usize> = vec![];
        for y in 0..self.height() {
            for x in 0..self.width() {
                let tile = self.get(x as i32, y as i32).unwrap();
                if let Tile::MESSAGE { message_id } = tile {
                    message_ids.push(*message_id);
                }
                file.write_all(&[tile.to_bytes()])?;
            }
        }
        self.serialize_messages(file, &message_ids)?;
        Ok(())
    }
    pub(crate) fn serialize_messages<T: Write>(
        &self,
        file: &mut T,
        message_ids: &Vec<usize>,
    ) -> Result<(), GameWorldParseError> {
        // Write length as 32-bit unsigned integer
        let num_messages = bincode::serialize(&(message_ids.len() as u32))?;
        file.write_all(&num_messages)?;

        // Write all messages
        for message_id in message_ids {
            let serialized_message_text =
                bincode::serialize(self.get_message(*message_id).unwrap_or_default())?;
            file.write_all(&serialized_message_text)?;
        }
        Ok(())
    }
    /// Serialize the forced tileset of this map
    pub(crate) fn serialize_tileset<T: Write>(
        &self,
        file: &mut T,
    ) -> Result<(), GameWorldParseError> {
        if let Some(tileset) = &self.forced_tileset {
            let buf = [0x01u8, tileset.to_bytes()];
            file.write_all(&buf)?;
        } else {
            let buf = [0u8; 1];
            file.write_all(&buf)?;
        };
        Ok(())
    }
}
/// Implementations for parsing v0x01 map files.
/// The code duplication is intentional here because we want to keep perfect
/// backwards-compatibility with older map formats while supporting frequent
/// changes of the current map format version.
impl GameWorld {
    /// Parse a map with version 0x01.
    fn parse_v1<T: Read>(&mut self, file: &mut T) -> Result<(), GameWorldParseError> {
        // Parse Map Name
        let name = self.parse_current_version_string(file)?;
        self.set_name(name.as_str());

        // Parse Map Author
        let author = self.parse_current_version_string(file)?;
        self.set_author(author.as_str());

        let hash = self.parse_current_version_uuid(file)?;
        self.hash = hash;

        // Parse Map Width and Map Height
        let map_width: usize = bincode::deserialize_from::<&mut T, usize>(file)?;
        let map_height: usize = bincode::deserialize_from::<&mut T, usize>(file)?;
        if map_width > MAX_MAP_WIDTH {
            return Err(GameWorldParseError::InvalidMapWidth {
                max_width: MAX_MAP_WIDTH,
                actual_width: map_width,
            });
        }
        if map_height > MAX_MAP_HEIGHT {
            return Err(GameWorldParseError::InvalidMapHeight {
                max_height: MAX_MAP_HEIGHT,
                actual_height: map_height,
            });
        }
        self.data = vec![vec![Tile::AIR; map_height]; map_width];
        assert_eq!(map_width, self.width());
        assert_eq!(map_height, self.height());

        // Parse actual map content
        // The message ID of the current message tile
        let mut current_message_id = 0usize;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let mut buf = [0u8; 1];
                file.read_exact(&mut buf).map_err(|e| {
                    GameWorldParseError::UnexpectedEndOfTileData {
                        io_error: e,
                        position: (y * x) + x,
                    }
                })?;
                let mut tile = Tile::from_bytes(buf[0])
                    .ok_or(GameWorldParseError::InvalidTile { tile_bytes: buf[0] })?;
                // Assign the current message ID to message tiles
                if let Tile::MESSAGE { .. } = tile {
                    tile = Tile::MESSAGE {
                        message_id: current_message_id,
                    };
                    current_message_id += 1;
                }
                self.set(x, y, tile);
            }
        }
        if current_message_id > 0 {
            self.parse_messages(file, current_message_id)?;
        }

        Ok(())
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
            Tile::WALLNATURE => 0x02,
            Tile::WALLCOBBLE => 0x03,
            Tile::WALLSMOOTH => 0x04,
            Tile::WALLCHISELED => 0x05,
            Tile::SLOPE => 0x0e,
            Tile::PILLAR => 0x0f,
            Tile::PLAYERSPAWN => 0x10,
            Tile::DOOR => 0x20,
            Tile::OPENDOOR => 0x21,
            Tile::COIN => 0x30,
            Tile::KEY => 0x31,
            Tile::STARCRYSTAL => 0x37,
            Tile::LADDER => 0x22,
            Tile::LADDERSLOPE => 0x23,
            Tile::LADDERNATURE => 0x24,
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
            Tile::WALLSPIKEST => 0x5A,
            Tile::WALLSPIKESLT => 0x5B,
            Tile::WALLSPIKESRT => 0x5C,
            Tile::WALLSPIKESRLB => 0x5D,
            Tile::WALLSPIKESRLT => 0x5E,
            Tile::ARROWRIGHT => 0x32,
            Tile::ARROWLEFT => 0x33,
            Tile::ARROWUP => 0x34,
            Tile::ARROWDOWN => 0x35,
            Tile::MESSAGE { .. } => 0x36,
            Tile::EXIT => 0x11,
            Tile::TELEPORTENTRY { teleport_id } => 0x70 + (teleport_id.const_to_u8() * 2),
            Tile::TELEPORTEXIT { teleport_id } => 0x71 + (teleport_id.const_to_u8() * 2),
            Tile::CAMPAIGNTRAILWALKWAY => 0xf0,
            Tile::CAMPAIGNTRAILMAPENTRYPOINT { .. } => 0xf1,
            Tile::CAMPAIGNTRAILBORDER => 0xf2,
            Tile::CAMPAIGNTRAILLOCKEDMAPENTRYPOINT { .. } => 0xf3,
            Tile::COBBLEROOFSLOPEL => 0x80,
            Tile::COBBLEROOFSLOPER => 0x81,
            Tile::VENDINGMACHINEL => 0x12,
            Tile::VENDINGMACHINER => 0x13,
        }
    }

    pub const fn from_bytes(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Tile::AIR),
            0x01 => Some(Tile::WALL),
            0x02 => Some(Tile::WALLNATURE),
            0x03 => Some(Tile::WALLCOBBLE),
            0x04 => Some(Tile::WALLSMOOTH),
            0x05 => Some(Tile::WALLCHISELED),
            0x0e => Some(Tile::SLOPE),
            0x0f => Some(Tile::PILLAR),
            0x10 => Some(Tile::PLAYERSPAWN),
            0x11 => Some(Tile::EXIT),
            0x12 => Some(Tile::VENDINGMACHINEL),
            0x13 => Some(Tile::VENDINGMACHINER),
            0x20 => Some(Tile::DOOR),
            0x21 => Some(Tile::OPENDOOR),
            0x30 => Some(Tile::COIN),
            0x31 => Some(Tile::KEY),
            0x37 => Some(Tile::STARCRYSTAL),
            0x22 => Some(Tile::LADDER),
            0x23 => Some(Tile::LADDERSLOPE),
            0x24 => Some(Tile::LADDERNATURE),
            0x40 => Some(Tile::SPIKES),
            0x41 => Some(Tile::SPIKESALT),
            0x42 => Some(Tile::SPIKESSLOPED),
            0x50 => Some(Tile::WALLSPIKESL),
            0x51 => Some(Tile::WALLSPIKESR),
            0x52 => Some(Tile::WALLSPIKESLR),
            0x53 => Some(Tile::WALLSPIKESB),
            0x54 => Some(Tile::WALLSPIKESLB),
            0x55 => Some(Tile::WALLSPIKESRB),
            0x56 => Some(Tile::WALLSPIKESTB),
            0x57 => Some(Tile::WALLSPIKESRLTB),
            0x58 => Some(Tile::WALLSPIKESRTB),
            0x59 => Some(Tile::WALLSPIKESLTB),
            0x5A => Some(Tile::WALLSPIKEST),
            0x5B => Some(Tile::WALLSPIKESLT),
            0x5C => Some(Tile::WALLSPIKESRT),
            0x5D => Some(Tile::WALLSPIKESRLB),
            0x5E => Some(Tile::WALLSPIKESRLT),
            0x70..=0x7e if byte % 2 == 0 => Some(Tile::TELEPORTENTRY {
                teleport_id: TeleportId::const_from_u8((byte - 0x70) / 2),
            }),
            0x71..=0x7f if byte % 2 == 1 => Some(Tile::TELEPORTEXIT {
                teleport_id: TeleportId::const_from_u8((byte - 0x71) / 2),
            }),
            0x32 => Some(Tile::ARROWRIGHT),
            0x33 => Some(Tile::ARROWLEFT),
            0x34 => Some(Tile::ARROWUP),
            0x35 => Some(Tile::ARROWDOWN),
            0x36 => Some(Tile::MESSAGE { message_id: 0 }),
            0xf0 => Some(Tile::CAMPAIGNTRAILWALKWAY),
            0xf1 => Some(Tile::CAMPAIGNTRAILMAPENTRYPOINT {
                interaction: InteractionKind::LaunchMap {
                    map_name: String::new(),
                },
            }),
            0xf2 => Some(Tile::CAMPAIGNTRAILBORDER),
            0xf3 => Some(Tile::CAMPAIGNTRAILLOCKEDMAPENTRYPOINT {
                interaction: InteractionKind::LaunchMap {
                    map_name: String::new(),
                },
            }),
            0x80 => Some(Tile::COBBLEROOFSLOPEL),
            0x81 => Some(Tile::COBBLEROOFSLOPER),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bincode::ErrorKind;
    use bytebuffer::ByteBuffer;
    use std::path::PathBuf;
    use strum::{EnumCount, IntoEnumIterator};

    fn get_test_data_folder() -> PathBuf {
        let mut ret = PathBuf::from(env!("PROJECTDIR"));
        ret.push("testdata");
        ret
    }

    #[test]
    fn test_bidirectional_serialization_for_tiles() {
        for tile in Tile::iter().chain(TeleportId::iter().flat_map(|teleport_id| {
            vec![
                Tile::TELEPORTEXIT { teleport_id },
                Tile::TELEPORTENTRY { teleport_id },
            ]
        })) {
            let reference: &Tile = &tile;
            let actual = Tile::from_bytes(reference.to_bytes());
            assert!(
                actual.is_some(),
                "Deserializing Tile {} (0x{:02X}) resulted in an error: Tile not found in {}",
                reference.to_string(),
                reference.to_bytes(),
                stringify!(Tile::from_bytes())
            );
            let actual = actual.unwrap();
            assert_eq!(
                *reference,
                actual,
                "The Tile {} (0x{:02X}) deserialized into tile {} (0x{:02X}) !",
                reference.to_string(),
                reference.to_bytes(),
                actual.to_string(),
                actual.to_bytes(),
            );
        }
    }
    fn test_write_and_read_map(map: &mut GameWorld) -> GameWorld {
        map.recompute_hash();
        let mut buf = ByteBuffer::new();
        let result = map.serialize(&mut buf);
        assert!(
            result.is_ok(),
            "Map failed to serialize with error: {}",
            result.unwrap_err().to_string()
        );
        let mut result_map = GameWorld::new(1, 1);
        buf.set_rpos(0);
        let result = result_map.parse(&mut buf);
        assert!(
            result.is_ok(),
            "Map failed to parse with error: {}",
            result.unwrap_err().to_string()
        );
        assert_eq!(map.hash, result_map.hash);
        assert_eq!(map.author, result_map.author);
        assert_eq!(map.name, result_map.name);
        assert_eq!(map.width(), result_map.width());
        assert_eq!(map.height(), result_map.height());
        assert_eq!(map.forced_tileset(), result_map.forced_tileset());
        result_map
    }

    #[test]
    /// Regression Test that tests loading a V0x01 map correctly from disk.
    fn test_read_v1_map() {
        let mut file = get_test_data_folder();
        file.push("testmap_v0x01.exm");
        println!("World file {}", &file.as_path().to_str().unwrap());
        let world = GameWorld::load_from_file(file.as_path()).unwrap();
        assert_eq!(24usize, world.width());
        assert_eq!(10usize, world.height());
        assert_eq!("Debugger", world.get_author());
        assert_eq!("Example World", world.get_name());
    }

    #[test]
    fn test_write_and_read_simple_map() {
        let mut reference_map = GameWorld::new(2, 2);
        reference_map
            .set(0, 0, Tile::WALL)
            .set(1, 0, Tile::DOOR)
            .set(1, 1, Tile::PLAYERSPAWN)
            .set(0, 1, Tile::AIR);
        test_write_and_read_map(&mut reference_map);
    }

    #[test]
    fn test_write_and_read_debug_map() {
        let mut reference_map = GameWorld::exampleworld();
        test_write_and_read_map(&mut reference_map);
    }
    #[test]
    fn test_write_and_read_default_map() {
        let mut reference_map = GameWorld::default();
        test_write_and_read_map(&mut reference_map);
    }
    #[test]
    fn test_write_and_read_showcase_map() {
        let mut reference_map = GameWorld::showcaseworld();
        test_write_and_read_map(&mut reference_map);
    }

    #[test]
    fn test_map_with_invalid_hash() {
        let mut map = GameWorld::exampleworld();
        let mut data: Vec<u8> = vec![];
        data.extend_from_slice(&MAGICBYTES);
        data.push(GameWorld::CURRENT_VERSION);
        data.extend_from_slice(&bincode::serialize(&map.name).unwrap());
        data.extend_from_slice(&bincode::serialize(&map.author).unwrap());
        map.set(0, 0, Tile::WALL)
            .set(1, 0, Tile::DOOR)
            .set(1, 1, Tile::PLAYERSPAWN)
            .set(0, 1, Tile::AIR);
        let mut buf = ByteBuffer::new();
        let result = map.serialize(&mut buf);
        assert!(result.is_ok());
        let mut result_map = GameWorld::new(1, 1);
        let hash_offset: usize = data.len() + 16;
        buf.set_rpos(hash_offset);
        let val = buf.read_u8();
        buf.set_rpos(hash_offset);
        buf.write_u8(val.unwrap() + 1);
        buf.set_rpos(0);
        let result = result_map.parse(&mut buf);
        assert!(&result.is_err(), "Map with invalid hash parsed correctly");
        assert_eq!(
            9,
            result.as_ref().unwrap_err().numeric_error(),
            "Expected Hash Mismatch, got {:?}",
            &result
        );
    }

    #[test]
    fn test_write_and_read_map_with_empty_name() {
        let mut reference_map = GameWorld::new(2, 2);
        reference_map
            .set(0, 0, Tile::WALL)
            .set(1, 0, Tile::DOOR)
            .set(1, 1, Tile::KEY)
            .set(0, 1, Tile::AIR)
            .set_name("")
            .set_author("John Doe")
            .set_forced_tileset(Some(Tileset::Classic))
            .set_clean();
        test_write_and_read_map(&mut reference_map);
    }

    #[test]
    fn test_write_and_read_map_with_empty_author() {
        let mut reference_map = GameWorld::new(2, 2);
        reference_map
            .set(0, 0, Tile::AIR)
            .set(1, 0, Tile::WALL)
            .set(1, 1, Tile::SPIKES)
            .set(0, 1, Tile::COIN)
            .set_name("Test Map")
            .set_author("")
            .set_dirty();
        test_write_and_read_map(&mut reference_map);
    }

    #[test]
    fn test_write_and_read_map_with_removed_teleports() {
        let mut reference_map = GameWorld::new(2, 2);
        reference_map
            .set(0, 0, Tile::WALL)
            .set(
                0,
                0,
                Tile::TELEPORTEXIT {
                    teleport_id: TeleportId::ONE,
                },
            )
            .set(0, 0, Tile::WALL)
            .set(
                0,
                1,
                Tile::TELEPORTEXIT {
                    teleport_id: TeleportId::ONE,
                },
            );
        assert!(matches!(reference_map.get(0, 0), Some(Tile::WALL)));
        test_write_and_read_map(&mut reference_map);
        reference_map.set(
            1,
            1,
            Tile::TELEPORTEXIT {
                teleport_id: TeleportId::ONE,
            },
        );
        assert!(matches!(reference_map.get(0, 1), Some(Tile::AIR)));
        assert!(matches!(
            reference_map.get(1, 1),
            Some(Tile::TELEPORTEXIT { .. })
        ));
        test_write_and_read_map(&mut reference_map);
    }

    #[test]
    fn test_map_hash_differs_for_different_messages() {
        let mut reference_map = GameWorld::new(2, 2);
        reference_map.set_message_tile(0, 0, "Hello World".to_string());
        reference_map.recompute_hash();
        let mut other_map = GameWorld::new(2, 2);
        other_map.set_message_tile(0, 0, "Goodbye World".to_string());
        other_map.recompute_hash();
        assert_ne!(reference_map.hash, other_map.hash);
        other_map.set_message(0, "Hello World".to_string()).unwrap();
        other_map.recompute_hash();
        assert_eq!(reference_map.hash, other_map.hash);
    }
    #[test]
    fn test_write_and_read_map_with_messages() {
        let mut reference_map = GameWorld::new(2, 2);
        reference_map
            .set_message_tile(0, 0, "Hello World".to_string())
            .set(1, 0, Tile::WALL)
            .set(0, 1, Tile::SPIKES)
            .set_message_tile(1, 1, "Goodbye World".to_string())
            .set_name("Test Map with strings")
            .set_author("Debugger");
        let new_map = test_write_and_read_map(&mut reference_map);
        assert_eq!(new_map.get_message(0).unwrap(), "Hello World");
        assert_eq!(new_map.get_message(1).unwrap(), "Goodbye World");
    }
    #[test]
    fn test_write_and_read_map_with_messages_out_of_order() {
        let mut reference_map = GameWorld::new(2, 2);
        reference_map
            .set_message_tile(1, 1, "Hello World".to_string())
            .set(1, 0, Tile::WALL)
            .set(0, 1, Tile::SPIKES)
            .set_message_tile(0, 0, "Goodbye World".to_string())
            .set_name("Test Map with strings")
            .set_author("Debugger");
        let new_map = test_write_and_read_map(&mut reference_map);
        assert_eq!(reference_map.get_message(0).unwrap(), "Hello World");
        assert_eq!(reference_map.get_message(1).unwrap(), "Goodbye World");
        assert_eq!(new_map.get_message(1).unwrap(), "Hello World");
        assert_eq!(new_map.get_message(0).unwrap(), "Goodbye World");
        assert!(matches!(
            new_map.get(0, 0).unwrap(),
            Tile::MESSAGE { message_id: 0 }
        ));
        assert!(matches!(
            new_map.get(1, 1).unwrap(),
            Tile::MESSAGE { message_id: 1 }
        ));
    }

    #[test]
    fn test_write_and_read_map_with_all_tiles() {
        let mut reference_map = GameWorld::new(Tile::COUNT, 1);
        for (i, tile) in Tile::iter().enumerate() {
            reference_map.set(i, 0, tile);
        }
        test_write_and_read_map(&mut reference_map);
    }

    #[test]
    fn test_map_with_invalid_magic_bytes() {
        let data: [u8; 100] = [0x00; 100];
        let mut buf = ByteBuffer::from_bytes(&data);
        let mut map = GameWorld::new(1, 1);
        let result = map.parse(&mut buf);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().numeric_error(),
            GameWorldParseError::InvalidMagicBytes {
                expected: MAGICBYTES,
                actual: [0u8; 9],
            }
            .numeric_error()
        );
    }

    #[test]
    fn test_map_with_invalid_version() {
        let mut data: Vec<u8> = vec![];
        data.extend_from_slice(&MAGICBYTES);
        data.push(0xff);
        data.extend_from_slice(&[0; 100]);
        let mut buf = ByteBuffer::from_bytes(&data);
        let mut map = GameWorld::new(1, 1);
        let result = map.parse(&mut buf);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().numeric_error(),
            GameWorldParseError::InvalidVersion {
                invalid_version: 0xff
            }
            .numeric_error()
        );
    }

    #[test]
    #[ignore]
    fn test_map_with_invalid_name_string() {
        let mut data: Vec<u8> = vec![];
        data.extend_from_slice(&MAGICBYTES);
        data.push(GameWorld::CURRENT_VERSION);
        data.extend_from_slice(&[0xffu8; 100]);
        let mut buf = ByteBuffer::from_bytes(&data);
        let mut map = GameWorld::new(1, 1);
        let result = map.parse(&mut buf);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().numeric_error(),
            GameWorldParseError::BincodeError {
                bincode_error: Box::new(ErrorKind::InvalidCharEncoding)
            }
            .numeric_error()
        );
    }

    #[test]
    #[ignore]
    fn test_map_with_invalid_author_string() {
        let mut data: Vec<u8> = vec![];
        data.extend_from_slice(&MAGICBYTES);
        data.push(GameWorld::CURRENT_VERSION);
        data.extend_from_slice(&bincode::serialize("Test Name").unwrap());
        data.extend_from_slice(&[0xffu8; 100]);
        let mut buf = ByteBuffer::from_bytes(&data);
        let mut map = GameWorld::new(1, 1);
        let result = map.parse(&mut buf);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().numeric_error(),
            GameWorldParseError::BincodeError {
                bincode_error: Box::new(ErrorKind::InvalidCharEncoding)
            }
            .numeric_error()
        );
    }

    #[test]
    #[ignore]
    fn test_map_with_invalid_width() {
        let mut data: Vec<u8> = vec![];
        data.extend_from_slice(&MAGICBYTES);
        data.push(GameWorld::CURRENT_VERSION);
        data.extend_from_slice(&bincode::serialize("Test Name").unwrap());
        data.extend_from_slice(&bincode::serialize("Test Author").unwrap());
        data.extend_from_slice(&[0xffu8; HASH_LENGTH]);
        data.extend_from_slice(&bincode::serialize(&((MAX_MAP_WIDTH + 1) as usize)).unwrap());
        let mut buf = ByteBuffer::from_bytes(&data);
        let mut map = GameWorld::new(1, 1);
        let result = map.parse(&mut buf);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().numeric_error(),
            GameWorldParseError::InvalidMapWidth {
                max_width: MAX_MAP_WIDTH,
                actual_width: MAX_MAP_WIDTH + 1,
            }
            .numeric_error()
        );
    }

    #[test]
    #[ignore]
    fn test_map_with_invalid_width_zero() {
        let mut data: Vec<u8> = vec![];
        data.extend_from_slice(&MAGICBYTES);
        data.push(GameWorld::CURRENT_VERSION);
        data.extend_from_slice(&bincode::serialize("Test Name").unwrap());
        data.extend_from_slice(&bincode::serialize("Test Author").unwrap());
        data.extend_from_slice(&[0xffu8; HASH_LENGTH]);
        data.extend_from_slice(&bincode::serialize(&(0 as usize)).unwrap());
        let mut buf = ByteBuffer::from_bytes(&data);
        let mut map = GameWorld::new(1, 1);
        let result = map.parse(&mut buf);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().numeric_error(),
            GameWorldParseError::InvalidMapWidth {
                max_width: MAX_MAP_WIDTH,
                actual_width: MAX_MAP_WIDTH + 1,
            }
            .numeric_error()
        );
    }
}
