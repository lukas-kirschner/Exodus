use std::fmt::{Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use strum_macros::{EnumIter, EnumCount as EnumCountMacro};

pub struct Config {
    pub game_language: Language,
}

impl Default for Config {
    fn default() -> Self {
        return Config {
            game_language: Language::default()
        };
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, EnumIter, EnumCountMacro)]
pub enum Language {
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

impl Default for Language {
    fn default() -> Self {
        Language::ENGLISH
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
            _ => None
        }
    }
}

// Serialization/Deserialization
impl Config {
    pub fn save_to_file(&mut self, path: &Path) -> std::io::Result<()> {
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
        file.write(&[language_b])?;
        Ok(())
    }
    fn parse<T: Read>(&mut self, file: &mut T) -> std::io::Result<()> {
        // Read Language
        let mut lang_buf = [0u8; 1];
        file.read_exact(&mut lang_buf)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bytebuffer::ByteBuffer;
    use strum::{IntoEnumIterator};
    use super::*;

    #[test]
    fn test_bidirectional_serialization_for_language() {
        for lang in Language::iter() {
            let reference = &lang;
            let actual = Language::from_bytes(reference.to_bytes());
            assert!(actual.is_some(), "Deserializing Language {} (0x{:02X}) resulted in an error: Language not found in {}", reference.to_string(), reference.to_bytes(), stringify!(Tile::from_bytes()));
            let actual = actual.unwrap();
            assert_eq!(*reference, actual, "The Language {} (0x{:02X}) deserialized into Language {} (0x{:02X}) !",
                       reference.to_string(), reference.to_bytes(), actual.to_string(), actual.to_bytes(), );
        }
    }

    fn test_write_and_read_config(config: &mut Config) {
        let mut buf = ByteBuffer::new();
        let result = config.serialize(&mut buf);
        assert!(result.is_ok(), "Config failed to serialize with error: {}", result.unwrap_err().to_string());
        let mut result_config = Config::default();
        buf.set_rpos(0);
        let result = result_config.parse(&mut buf);
        assert!(result.is_ok(), "Config failed to parse with error: {}", result.unwrap_err().to_string());
        assert_eq!(config.game_language, result_config.game_language);
    }

    #[test]
    fn test_write_and_read_default_config() {
        let mut config = Config::default();
        test_write_and_read_config(&mut config);
    }
}