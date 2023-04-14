use crate::exodus_serializable::ExodusSerializable;
use crate::highscores::highscore::Highscore;
use crate::highscores::io_error::HighscoreParseError;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::io::{Read, Write};

const CURRENT_PLAYERHIGHSCORE_VERSION: u8 = 0x01;

/// A highscores entry for one single player.
/// A highscore record typically exists per-map and contains all highscores for that specific player.
pub struct PlayerHighscores {
    player: String,
    scores: BTreeSet<PlayerHighscoresWrapper>,
}

impl PlayerHighscores {
    /// Create a new empty set of highscores for the given player
    pub fn new(player: String) -> Self {
        PlayerHighscores {
            player,
            scores: BTreeSet::new(),
        }
    }
    /// Get the number of stored highscores
    pub fn len(&self) -> usize {
        self.scores.len()
    }
    /// Check if this highscores object is empty
    pub fn is_empty(&self) -> bool {
        self.scores.is_empty()
    }
    /// Get the best highscore
    pub fn best(&self) -> Option<(i64, &Highscore)> {
        self.scores
            .last()
            .map(|phw| (phw.timestamp, &phw.highscore))
    }
    /// Store the given highscore with the current time
    pub fn store_with_current_time(&mut self, highscore: Highscore) {
        let timestamp = chrono::offset::Local::now().timestamp_millis();
        self.store(timestamp, highscore);
    }
    /// Store the given highscore with the given timestamp (ms from UNIX Epoch)
    pub fn store(&mut self, timestamp: i64, highscore: Highscore) {
        let wrapper = PlayerHighscoresWrapper {
            timestamp,
            highscore,
        };
        self.scores.insert(wrapper);
    }
}

/// Implementation for Serializer
impl ExodusSerializable for PlayerHighscores {
    type ParseError = HighscoreParseError;

    fn serialize<T: Write>(&self, file: &mut T) -> Result<(), HighscoreParseError> {
        // Write PlayerHighscore Version
        file.write_all(&[CURRENT_PLAYERHIGHSCORE_VERSION])?;

        // Store Player Name
        let bin_name = bincode::serialize(&self.player)?;
        file.write_all(&bin_name)?;

        // Store Length of Highscores
        let bin_highscore_length = bincode::serialize(&self.scores.len())?;
        file.write_all(&bin_highscore_length)?;

        // Store Highscores
        for highscore in &self.scores {
            highscore.serialize(file)?;
        }
        Ok(())
    }

    fn parse<T: Read>(&mut self, file: &mut T) -> Result<(), Self::ParseError> {
        let mut buf: [u8; 1] = [0; 1];
        file.read_exact(&mut buf)?;
        match buf[0] {
            CURRENT_HIGHSCORE_VERSION => self.parse_current_version(file),
            // Add older versions here
            _ => {
                return Err(HighscoreParseError::InvalidVersion {
                    invalid_version: buf[0],
                })
            },
        }?;

        Ok(())
    }

    fn parse_current_version<T: Read>(&mut self, file: &mut T) -> Result<(), Self::ParseError> {
        // Parse Player Name
        let p_name = bincode::deserialize_from::<&mut T, String>(file)?;
        self.player = p_name;

        // Parse length of highscores
        let hs_len = bincode::deserialize_from::<&mut T, usize>(file)?;

        // Deserialize Highscores
        for _ in 0..hs_len {
            let mut highscore = PlayerHighscoresWrapper::default();
            highscore.parse(file)?;
            if !self.scores.insert(highscore) {
                return Err(HighscoreParseError::DuplicateHighscoreEntry);
            }
        }
        Ok(())
    }
}

#[derive(Eq, PartialEq, Debug)]
struct PlayerHighscoresWrapper {
    pub timestamp: i64,
    pub highscore: Highscore,
}

impl Default for PlayerHighscoresWrapper {
    fn default() -> Self {
        Self {
            timestamp: 0,
            highscore: Default::default(),
        }
    }
}

/// Implementation for Serializer
impl ExodusSerializable for PlayerHighscoresWrapper {
    type ParseError = HighscoreParseError;

    fn serialize<T: Write>(&self, file: &mut T) -> Result<(), HighscoreParseError> {
        // Write PlayerHighscoresWrapper Version
        file.write_all(&[CURRENT_PLAYERHIGHSCORE_VERSION])?;

        // Store Timestamp
        let timestamp_b = bincode::serialize(&self.timestamp)?;
        file.write_all(&timestamp_b)?;

        // Store Highscore
        self.highscore.serialize(file)?;
        Ok(())
    }

    fn parse<T: Read>(&mut self, file: &mut T) -> Result<(), Self::ParseError> {
        todo!()
    }

    fn parse_current_version<T: Read>(&mut self, file: &mut T) -> Result<(), Self::ParseError> {
        todo!()
    }
}

impl PartialOrd for PlayerHighscoresWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            match (self.highscore.coins() as i32 - self.highscore.moves() as i32)
                .cmp(&(other.highscore.coins() as i32 - other.highscore.moves() as i32))
            {
                Ordering::Equal => self.timestamp.cmp(&other.timestamp),
                x => x,
            },
        )
    }
}

impl Ord for PlayerHighscoresWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::highscores::highscore::Highscore;
    use crate::highscores::player_highscores::{PlayerHighscores, PlayerHighscoresWrapper};
    use std::cmp::Ordering;
    use std::time::Duration;

    #[test]
    fn test_total_ordering() {
        let good_score = PlayerHighscoresWrapper {
            timestamp: 0,
            highscore: Highscore::new(5, 4),
        };
        let bad_score = PlayerHighscoresWrapper {
            timestamp: 0,
            highscore: Highscore::new(6, 4),
        };
        assert_eq!(Ordering::Greater, good_score.cmp(&bad_score));
        assert!(good_score > bad_score);
    }

    #[test]
    fn test_total_newest() {
        let good_score = PlayerHighscoresWrapper {
            timestamp: 1,
            highscore: Highscore::new(6, 4),
        };
        let bad_score = PlayerHighscoresWrapper {
            timestamp: 0,
            highscore: Highscore::new(6, 4),
        };
        assert_eq!(Ordering::Greater, good_score.cmp(&bad_score));
        assert!(good_score > bad_score);
    }

    #[test]
    fn test_total_ordering_equal() {
        let good_score = PlayerHighscoresWrapper {
            timestamp: 0,
            highscore: Highscore::new(5, 0),
        };
        let equal_score = PlayerHighscoresWrapper {
            timestamp: 0,
            highscore: Highscore::new(5, 0),
        };
        assert_eq!(Ordering::Equal, good_score.cmp(&equal_score));
    }

    #[test]
    fn test_player_highscores_default_empty() {
        let player_highscores = PlayerHighscores::new("Thorsten".to_string());
        assert_eq!(0, player_highscores.len());
        assert_eq!("Thorsten", player_highscores.player);
    }
    #[test]
    fn test_player_highscores_is_empty() {
        let mut player_highscores = PlayerHighscores::new("Thorsten".to_string());
        assert!(player_highscores.is_empty());
        player_highscores.store_with_current_time(Highscore::new(3, 3));
        assert!(!player_highscores.is_empty());
    }

    #[test]
    fn test_player_highscores_get_best() {
        let mut player_highscores = PlayerHighscores::new("Thorsten".to_string());
        player_highscores.store(0, Highscore::new(5, 0));
        player_highscores.store(3, Highscore::new(5, 1));
        player_highscores.store(2, Highscore::new(5, 2));
        player_highscores.store(4, Highscore::new(5, 0));
        assert_eq!(4, player_highscores.len());
        let (best_ts, best) = player_highscores.best().unwrap();
        assert_eq!(2, best.coins());
        assert_eq!(5, best.moves());
        assert_eq!(2, best_ts);
    }

    #[test]
    fn test_player_highscores_get_best_same_time() {
        let mut player_highscores = PlayerHighscores::new("Thorsten".to_string());
        player_highscores.store(3, Highscore::new(5, 0));
        player_highscores.store(3, Highscore::new(5, 1));
        player_highscores.store(3, Highscore::new(5, 2));
        player_highscores.store(0, Highscore::new(5, 0));
        assert_eq!(4, player_highscores.len());
        let (best_ts, best) = player_highscores.best().unwrap();
        assert_eq!(2, best.coins());
        assert_eq!(5, best.moves());
        assert_eq!(3, best_ts);
    }

    #[test]
    fn test_player_highscores_get_best_same_score() {
        let mut player_highscores = PlayerHighscores::new("Thorsten".to_string());
        player_highscores.store(1, Highscore::new(5, 2));
        player_highscores.store(2, Highscore::new(5, 2));
        player_highscores.store(3, Highscore::new(5, 2));
        player_highscores.store(4, Highscore::new(5, 2));
        assert_eq!(4, player_highscores.len());
        let (best_ts, best) = player_highscores.best().unwrap();
        assert_eq!(2, best.coins());
        assert_eq!(5, best.moves());
        assert_eq!(4, best_ts);
    }

    #[test]
    fn test_player_highscores_get_best_current_time() {
        let mut player_highscores = PlayerHighscores::new("Thorsten".to_string());
        player_highscores.store_with_current_time(Highscore::new(5, 2));
        std::thread::sleep(Duration::from_millis(2));
        player_highscores.store_with_current_time(Highscore::new(5, 2));
        assert_eq!(2, player_highscores.len());
        let (best_ts, best) = player_highscores.best().unwrap();
        assert_eq!(2, best.coins());
        assert_eq!(5, best.moves());
    }

    #[test]
    fn test_player_highscores_get_best_empty() {
        let player_highscores = PlayerHighscores::new("Thorsten".to_string());
        assert!(player_highscores.best().is_none());
    }
}
