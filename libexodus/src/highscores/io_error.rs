use crate::highscores::highscores_database::MAGICBYTES;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
#[repr(u8)]
/// An error that might be thrown in a Highscores Parser
pub enum HighscoreParseError {
    InvalidMagicBytes {
        expected: [u8; MAGICBYTES.len()],
        actual: [u8; MAGICBYTES.len()],
    },
    NotImplemented,
    InvalidVersion {
        invalid_version: u8,
    },
    IOError {
        io_error: std::io::Error,
    },
    BincodeError {
        bincode_error: Box<bincode::ErrorKind>,
    },
    UnexpectedEndOfData {
        position: usize,
        io_error: std::io::Error,
    },
    DuplicateHighscoreEntry,
    DuplicateDatabaseEntry,
    DuplicatePlayerEntry,
    HashMismatch {
        expected: [u8; 32],
        actual: [u8; 32],
    },
}

impl Display for HighscoreParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HighscoreParseError::InvalidMagicBytes { expected, actual } => write!(
                f,
                "Invalid Magic Bytes in Map File! Expected: {:02x?} Got: {:02x?}",
                expected, actual
            ),
            HighscoreParseError::NotImplemented => write!(f, "Not Implemented"),
            HighscoreParseError::InvalidVersion { invalid_version } => {
                write!(f, "Invalid Map Version: 0x{:02x}", invalid_version)
            },
            HighscoreParseError::IOError { io_error } => std::fmt::Display::fmt(&io_error, f),
            HighscoreParseError::BincodeError { bincode_error } => {
                std::fmt::Display::fmt(&bincode_error, f)
            },
            HighscoreParseError::UnexpectedEndOfData { position, io_error } => write!(
                f,
                "Unexpected end of Tile Data at position {}! {}",
                position, io_error
            ),
            HighscoreParseError::DuplicateHighscoreEntry => {
                write!(f, "Unexpected duplicate highscore entry found!")
            },
            HighscoreParseError::DuplicateDatabaseEntry => {
                write!(f, "Unexpected duplicate entry found in database file!")
            },
            HighscoreParseError::DuplicatePlayerEntry => write!(
                f,
                "Unexpected duplicate player entry found in highscore database file!"
            ),
            HighscoreParseError::HashMismatch { expected, actual } => write!(
                f,
                "Invalid Map Hash in Map File! Expected: {:02x?} Got: {:02x?}",
                expected, actual
            ),
        }
    }
}

impl Error for HighscoreParseError {}

impl From<std::io::Error> for HighscoreParseError {
    fn from(io_error: std::io::Error) -> Self {
        HighscoreParseError::IOError { io_error }
    }
}

impl From<Box<bincode::ErrorKind>> for HighscoreParseError {
    fn from(bincode_error: Box<bincode::ErrorKind>) -> Self {
        HighscoreParseError::BincodeError { bincode_error }
    }
}

impl HighscoreParseError {
    /// Get the numeric error to compare the error kind. Discards all data that is carried by this error
    pub fn numeric_error(&self) -> u8 {
        match self {
            HighscoreParseError::InvalidMagicBytes { .. } => 0,
            HighscoreParseError::NotImplemented => 1,
            HighscoreParseError::InvalidVersion { .. } => 2,
            HighscoreParseError::IOError { .. } => 3,
            HighscoreParseError::BincodeError { .. } => 4,
            HighscoreParseError::UnexpectedEndOfData { .. } => 7,
            HighscoreParseError::DuplicateHighscoreEntry => 8,
            HighscoreParseError::DuplicateDatabaseEntry => 9,
            HighscoreParseError::DuplicatePlayerEntry => 10,
            HighscoreParseError::HashMismatch { .. } => 11,
        }
    }
}
