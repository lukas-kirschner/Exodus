use crate::tilesets::Tileset;
use std::fmt::{Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, BufWriter, ErrorKind, Read, Write};
use std::path::Path;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

pub struct Config {
    pub game_language: Language,
    pub tile_set: Tileset,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            game_language: Language::default(),
            tile_set: Tileset::TinyPlatformQuestTiles,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, EnumIter, EnumCountMacro, Default)]
pub enum Language {
    #[default]
    ENGLISH,
    GERMAN,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::ENGLISH => write!(f, "English (US)"),
            Language::GERMAN => write!(f, "German"),
        }?;
        Ok(())
    }
}

impl Language {
    pub const fn to_bytes(&self) -> u8 {
        match self {
            Language::ENGLISH => 0x00,
            Language::GERMAN => 0x01,
        }
    }
    pub const fn from_bytes(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Language::ENGLISH),
            0x01 => Some(Language::GERMAN),
            _ => None,
        }
    }
    pub const fn locale(&self) -> &str {
        match self {
            Language::ENGLISH => "en_US",
            Language::GERMAN => "de_DE",
        }
    }
}

impl Tileset {
    pub const fn to_bytes(&self) -> u8 {
        match self {
            Tileset::TinyPlatformQuestTiles => 0x00,
            Tileset::Classic => 0x01,
        }
    }
    pub const fn from_bytes(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Tileset::TinyPlatformQuestTiles),
            0x01 => Some(Tileset::Classic),
            _ => None,
        }
    }
}

// Serialization/Deserialization
impl Config {
    pub fn save_to_file(&self, path: &Path) -> std::io::Result<()> {
        let file: File = OpenOptions::new().create(true).write(true).open(path)?;
        let mut buf = BufWriter::new(file);
        self.serialize(&mut buf)?;
        Ok(())
    }
    pub fn load_from_file(path: &Path) -> std::io::Result<Self> {
        let file = OpenOptions::new().read(true).open(path)?;
        let mut buf = BufReader::new(file);
        let mut ret = Config::default();
        ret.parse(&mut buf)?;
        Ok(ret)
    }

    fn serialize<T: Write>(&self, file: &mut T) -> std::io::Result<()> {
        let language_b = self.game_language.to_bytes();
        file.write_all(&[language_b])?;
        let tileset_b = self.tile_set.to_bytes();
        file.write_all(&[tileset_b])?;
        Ok(())
    }
    fn parse<T: Read>(&mut self, file: &mut T) -> std::io::Result<()> {
        // Read Language
        let mut lang_buf = [0u8; 1];
        file.read_exact(&mut lang_buf)?;
        self.game_language = Language::from_bytes(lang_buf[0]).ok_or_else(|| {
            io::Error::new(
                ErrorKind::Other,
                format!("Invalid Language 0x{:02X}", lang_buf[0]),
            )
        })?;
        // Read Tile set
        let mut tileset_buf = [0u8; 1];
        file.read_exact(&mut tileset_buf)?;
        self.tile_set = Tileset::from_bytes(tileset_buf[0]).ok_or_else(|| {
            io::Error::new(
                ErrorKind::Other,
                format!("Invalid Tileset 0x{:02X}", tileset_buf[0]),
            )
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytebuffer::ByteBuffer;
    use strum::IntoEnumIterator;

    #[test]
    fn test_bidirectional_serialization_for_language() {
        for lang in Language::iter() {
            let reference = &lang;
            let actual = Language::from_bytes(reference.to_bytes());
            assert!(actual.is_some(), "Deserializing Language {} (0x{:02X}) resulted in an error: Language not found in {}", reference.to_string(), reference.to_bytes(), stringify!(Language::from_bytes()));
            let actual = actual.unwrap();
            assert_eq!(
                *reference,
                actual,
                "The Language {} (0x{:02X}) deserialized into Language {} (0x{:02X}) !",
                reference.to_string(),
                reference.to_bytes(),
                actual.to_string(),
                actual.to_bytes(),
            );
        }
    }

    #[test]
    fn test_bidirectional_serialization_for_tileset() {
        for lang in Tileset::iter() {
            let reference = &lang;
            let actual = Tileset::from_bytes(reference.to_bytes());
            assert!(actual.is_some(), "Deserializing Tile Set {} (0x{:02X}) resulted in an error: Tile Set not found in {}", reference.to_string(), reference.to_bytes(), stringify!(Tileset::from_bytes()));
            let actual = actual.unwrap();
            assert_eq!(
                *reference,
                actual,
                "The Tile Set {} (0x{:02X}) deserialized into Tile Set {} (0x{:02X}) !",
                reference.to_string(),
                reference.to_bytes(),
                actual.to_string(),
                actual.to_bytes(),
            );
        }
    }

    fn test_write_and_read_config(config: &mut Config) {
        let mut buf = ByteBuffer::new();
        let result = config.serialize(&mut buf);
        assert!(
            result.is_ok(),
            "Config failed to serialize with error: {}",
            result.unwrap_err().to_string()
        );
        let mut result_config = Config::default();
        buf.set_rpos(0);
        let result = result_config.parse(&mut buf);
        assert!(
            result.is_ok(),
            "Config failed to parse with error: {}",
            result.unwrap_err().to_string()
        );
        assert_eq!(config.game_language, result_config.game_language);
        assert_eq!(config.tile_set, result_config.tile_set);
    }

    #[test]
    fn test_write_and_read_default_config() {
        let mut config = Config::default();
        test_write_and_read_config(&mut config);
    }

    #[test]
    fn test_write_and_read_german_config() {
        let mut config = Config::default();
        config.game_language = Language::GERMAN;
        test_write_and_read_config(&mut config);
    }

    #[test]
    fn test_write_and_read_german_config_classic() {
        let mut config = Config::default();
        config.game_language = Language::GERMAN;
        config.tile_set = Tileset::Classic;
        test_write_and_read_config(&mut config);
    }
}
