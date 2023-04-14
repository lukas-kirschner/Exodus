use crate::exodus_serializable::ExodusSerializable;
use crate::highscores::io_error::HighscoreParseError;
use std::io::{Read, Write};

/// A single high score record
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Highscore {
    num_moves: u32,
    num_coins: u32,
}

impl Highscore {
    pub fn new(num_moves: u32, num_coins: u32) -> Self {
        Highscore {
            num_moves,
            num_coins,
        }
    }
    pub fn moves(&self) -> u32 {
        self.num_moves
    }
    pub fn coins(&self) -> u32 {
        self.num_coins
    }
}

impl Default for Highscore {
    fn default() -> Self {
        Self {
            num_moves: 0,
            num_coins: 0,
        }
    }
}

/// Implementation for Serializer
impl ExodusSerializable for Highscore {
    const CURRENT_VERSION: u8 = 0x01;
    type ParseError = HighscoreParseError;
    fn serialize<T: Write>(&self, file: &mut T) -> Result<(), HighscoreParseError> {
        // Write Highscore Version
        file.write_all(&[Self::CURRENT_VERSION])?;

        // Write number of moves
        let moves_b = bincode::serialize(&self.num_moves)?;
        file.write_all(&moves_b)?;

        // Write number of coins
        let coins_b = bincode::serialize(&self.num_coins)?;
        file.write_all(&coins_b)?;

        Ok(())
    }
    fn parse<T: Read>(&mut self, file: &mut T) -> Result<(), HighscoreParseError> {
        // Parse Single Highscore Format
        let mut buf: [u8; 1] = [0; 1];
        file.read_exact(&mut buf)?;
        match buf[0] {
            Self::CURRENT_VERSION => self.parse_current_version(file),
            // Add older versions here
            _ => {
                return Err(HighscoreParseError::InvalidVersion {
                    invalid_version: buf[0],
                })
            },
        }?;

        Ok(())
    }

    /// Parse a highscore with the current version.
    /// The read position must be already behind the version byte.
    fn parse_current_version<T: Read>(&mut self, file: &mut T) -> Result<(), HighscoreParseError> {
        // Parse moves
        let moves: u32 = bincode::deserialize_from::<&mut T, u32>(file)?;
        self.num_moves = moves;

        // Parse coins
        let coins: u32 = bincode::deserialize_from::<&mut T, u32>(file)?;
        self.num_coins = coins;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::exodus_serializable::ExodusSerializable;
    use crate::highscores::highscore::Highscore;
    use crate::highscores::io_error::HighscoreParseError;
    use bytebuffer::ByteBuffer;
    use std::io;
    use std::io::ErrorKind;

    #[test]
    fn test_highscore_getters() {
        let highscore = Highscore::new(69, 42069);
        assert_eq!(69, highscore.moves());
        assert_eq!(42069, highscore.coins());
    }

    #[test]
    #[ignore]
    fn test_highscore_incomplete_data() {
        let full_data: [u8; 100] = [0x00; 100];
        let mut full_buf = ByteBuffer::from_bytes(&full_data);
        let mut highscore = Highscore::new(1, 1);
        highscore.serialize(&mut full_buf).unwrap();
        full_buf.set_rpos(0);

        let mut data: [u8; 5] = [0x00; 5];
        data.copy_from_slice(&full_data[0..5]);
        let mut buf = ByteBuffer::from_bytes(&data);
        let result = highscore.parse(&mut buf);
        //TODO serialize
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().numeric_error(),
            HighscoreParseError::BincodeError {
                bincode_error: Box::new(bincode::ErrorKind::Custom("Test".into())),
            }
            .numeric_error()
        );
    }

    #[test]
    fn test_highscore_invalid_version() {
        let data: [u8; 100] = [0x00; 100];
        let mut buf = ByteBuffer::from_bytes(&data);
        let mut highscore = Highscore::new(1, 1);
        highscore.serialize(&mut buf).unwrap();
        buf.set_wpos(0);
        buf.set_rpos(0);
        buf.write_u8(0xff);
        buf.set_rpos(0);
        let result = highscore.parse(&mut buf);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().numeric_error(),
            HighscoreParseError::InvalidVersion {
                invalid_version: 0xff,
            }
            .numeric_error()
        );
    }

    #[test]
    fn test_highscore_constructor() {
        let highscore = Highscore::new(0, 111);
        let highscore2 = Highscore {
            num_moves: 0,
            num_coins: 111,
        };
        assert_eq!(highscore, highscore2);
    }
    macro_rules! serialize_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (num_moves, num_coins) = $value;
            let highscore = Highscore::new(num_moves,num_coins);
            let mut buf = ByteBuffer::new();
            let result = highscore.serialize(&mut buf);
            assert!(
                result.is_ok(),
                "Highscore failed to serialize with error: {}",
                result.unwrap_err().to_string()
            );
            buf.set_rpos(0);
            let mut result_highscore = Highscore::new(123,123);
            let result = result_highscore.parse(&mut buf);
            assert!(
                result.is_ok(),
                "Highscore failed to parse with error: {}",
                result.unwrap_err().to_string()
            );
            assert_eq!(num_moves, result_highscore.moves());
            assert_eq!(num_coins, result_highscore.coins());
        }
    )*
    }
}

    serialize_tests! {
        serialize_highscore_0: (0, 0),
        serialize_highscore_11: (1, 1),
        serialize_highscore_01: (0, 1),
        serialize_highscore_10: (1, 0),
        serialize_highscore_1337: (1337, 69),
        serialize_highscore_limit1: (u32::MAX, u32::MIN),
        serialize_highscore_limit2: (u32::MIN, u32::MAX),
        serialize_highscore_limit3: (u32::MAX, u32::MAX),
    }
}
