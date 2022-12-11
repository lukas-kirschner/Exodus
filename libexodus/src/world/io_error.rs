use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::world::io::MAGICBYTES;

#[derive(Debug)]
#[repr(u8)]
/// An error that might be thrown in a Game World Parser
pub enum GameWorldParseError {
    InvalidMagicBytes { expected: [u8; MAGICBYTES.len()], actual: [u8; MAGICBYTES.len()] },
    NotImplemented,
    InvalidVersion { invalid_version: u8 },
    IOError { io_error: std::io::Error },
    BincodeError { bincode_error: Box<bincode::ErrorKind> },
    InvalidMapWidth { max_width: usize, actual_width: usize },
    InvalidMapHeight { max_height: usize, actual_height: usize },
    UnexpectedEndOfTileData { position: usize, io_error: std::io::Error },
    InvalidTile { tile_bytes: u8 },
    HashMismatch { expected: [u8; 32], actual: [u8; 32] },
}

impl Display for GameWorldParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameWorldParseError::InvalidMagicBytes { expected, actual } =>
                write!(f, "Invalid Magic Bytes in Map File! Expected: {:02x?} Got: {:02x?}", expected, actual),
            GameWorldParseError::NotImplemented => write!(f, "Not Implemented"),
            GameWorldParseError::InvalidVersion { invalid_version } => write!(f, "Invalid Map Version: 0x{:02x}", invalid_version),
            GameWorldParseError::IOError { io_error } => std::fmt::Display::fmt(&io_error, f),
            GameWorldParseError::BincodeError { bincode_error } => std::fmt::Display::fmt(&bincode_error, f),
            GameWorldParseError::InvalidMapWidth { max_width, actual_width } => write!(f, "Invalid Map Width: {} (Max allowed width: {})", actual_width, max_width),
            GameWorldParseError::InvalidMapHeight { max_height, actual_height } => write!(f, "Invalid Map Height: {} (Max allowed height: {})", actual_height, max_height),
            GameWorldParseError::UnexpectedEndOfTileData { position, io_error } => write!(f, "Unexpected end of Tile Data at position {}! {}", position, io_error.to_string()),
            GameWorldParseError::InvalidTile { tile_bytes } => write!(f, "Tile Byte not recognized as valid tile: 0x{:02x}", tile_bytes),
            GameWorldParseError::HashMismatch { expected, actual } => write!(f, "Hash Mismatch - your map file might be corrupted!\nExpected: {:02x?}\nActual: {:02x?}", expected, actual),
        }
    }
}

impl Error for GameWorldParseError {}

impl From<std::io::Error> for GameWorldParseError {
    fn from(io_error: std::io::Error) -> Self {
        GameWorldParseError::IOError { io_error }
    }
}

impl From<Box<bincode::ErrorKind>> for GameWorldParseError {
    fn from(bincode_error: Box<bincode::ErrorKind>) -> Self {
        GameWorldParseError::BincodeError { bincode_error }
    }
}

impl GameWorldParseError {
    /// Get the numeric error to compare the error kind. Discards all data that is carried by this error
    pub fn numeric_error(&self) -> u8 {
        match self {
            GameWorldParseError::InvalidMagicBytes { .. } => 0,
            GameWorldParseError::NotImplemented => 1,
            GameWorldParseError::InvalidVersion { .. } => 2,
            GameWorldParseError::IOError { .. } => 3,
            GameWorldParseError::BincodeError { .. } => 4,
            GameWorldParseError::InvalidMapWidth { .. } => 5,
            GameWorldParseError::InvalidMapHeight { .. } => 6,
            GameWorldParseError::UnexpectedEndOfTileData { .. } => 7,
            GameWorldParseError::InvalidTile { .. } => 8,
            GameWorldParseError::HashMismatch { .. } => 9,
        }
    }
}

