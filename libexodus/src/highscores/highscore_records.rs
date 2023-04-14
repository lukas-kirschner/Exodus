use crate::exodus_serializable::ExodusSerializable;
use crate::highscores::highscore::Highscore;
use crate::highscores::io_error::HighscoreParseError;
use crate::highscores::player_highscores::PlayerHighscores;
use std::collections::HashMap;
use std::io::{Read, Write};

/// A highscores database for one single map
pub struct HighscoreRecords {
    map_hash: [u8; 32],
    player_records: HashMap<String, PlayerHighscores>,
}

impl HighscoreRecords {
    pub fn new(map_hash: [u8; 32]) -> Self {
        HighscoreRecords {
            map_hash,
            player_records: HashMap::new(),
        }
    }
    /// Put the given highscore into the record, creating all necessary data structures automatically
    pub fn put(&mut self, player: String, timestamp: i64, highscore: Highscore) -> &mut Self {
        self.player_records
            .entry(player.clone())
            .or_insert_with(|| PlayerHighscores::new(player))
            .store(timestamp, highscore);
        self
    }
    /// Put the given highscore with the current time into the record, creating all necessary data structures automatically
    pub fn put_with_current_time(&mut self, player: String, highscore: Highscore) -> &mut Self {
        self.player_records
            .entry(player.clone())
            .or_insert_with(|| PlayerHighscores::new(player))
            .store_with_current_time(highscore);
        self
    }
    /// Get the (best) highscore for the given player
    pub fn get_best(&self, player: &String) -> Option<(i64, &Highscore)> {
        self.player_records
            .get(player)
            .and_then(|player_highscores| player_highscores.best())
    }
    /// Get the high scores for the given player
    pub fn get(&self, playername: &String) -> Option<&PlayerHighscores> {
        self.player_records.get(playername)
    }
    /// Get the high scores for the given player as mutable reference
    pub fn get_mut(&mut self, playername: &String) -> Option<&mut PlayerHighscores> {
        self.player_records.get_mut(playername)
    }
    /// Get the number of players stored in these highscore records
    pub fn len(&self) -> usize {
        self.player_records.len()
    }
    /// Check if this highscore record is empty
    pub fn is_empty(&self) -> bool {
        self.player_records.is_empty()
    }
}

/// Serialization Code
impl ExodusSerializable for HighscoreRecords {
    type ParseError = HighscoreParseError;

    fn serialize<T: Write>(&self, file: &mut T) -> Result<(), Self::ParseError> {
        todo!()
    }

    fn parse<T: Read>(&mut self, file: &mut T) -> Result<(), Self::ParseError> {
        todo!()
    }

    fn parse_current_version<T: Read>(&mut self, file: &mut T) -> Result<(), Self::ParseError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::highscores::highscore::Highscore;
    use crate::highscores::highscore_records::HighscoreRecords;
    use std::time::Duration;

    #[test]
    fn test_get() {
        let mut records = HighscoreRecords::new([0u8; 32]);
        records.put("Thorsten".to_string(), 0, Highscore::new(3, 0));
        let highscore = records.get(&"Thorsten".to_string());
        assert!(highscore.is_some());
        assert_eq!(1, highscore.unwrap().len());
        let (timestamp, score) = highscore.unwrap().best().unwrap();
        assert_eq!(0, timestamp);
        assert_eq!(0, score.coins());
        assert_eq!(3, score.moves());
    }
    #[test]
    fn test_is_empty() {
        let mut records = HighscoreRecords::new([0u8; 32]);
        assert!(records.is_empty());
        records.put("Thorsten".to_string(), 0, Highscore::new(3, 0));
        assert!(!records.is_empty());
    }

    #[test]
    fn test_get_best() {
        let mut records = HighscoreRecords::new([0u8; 32]);
        records.put("Thorsten".to_string(), 0, Highscore::new(3, 0));
        records.put("Thorsten".to_string(), 1337, Highscore::new(3, 0));
        let (timestamp, score) = records.get_best(&"Thorsten".to_string()).unwrap();
        assert_eq!(1337, timestamp);
        assert_eq!(0, score.coins());
        assert_eq!(3, score.moves());
    }

    #[test]
    fn test_put_with_current_time() {
        let mut records = HighscoreRecords::new([0u8; 32]);
        records.put_with_current_time("Thorsten".to_string(), Highscore::new(5, 0));
        std::thread::sleep(Duration::from_millis(2));
        records.put_with_current_time("Thorsten".to_string(), Highscore::new(5, 1));
        std::thread::sleep(Duration::from_millis(2));
        records.put_with_current_time("Thorsten".to_string(), Highscore::new(5, 1));
        std::thread::sleep(Duration::from_millis(2));
        records.put_with_current_time("Thorsten".to_string(), Highscore::new(5, 0));
        assert_eq!(1, records.len());
        let record = records.get(&"Thorsten".to_string()).unwrap();
        assert_eq!(4, record.len());
        let (_, best) = record.best().unwrap();
        assert_eq!(5, best.moves());
        assert_eq!(1, best.coins());
    }

    #[test]
    fn test_get_mut() {
        let mut records = HighscoreRecords::new([0u8; 32]);
        records.put("Thorsten".to_string(), 1337, Highscore::new(5, 0));
        {
            let highscore = records.get_mut(&"Thorsten".to_string());
            assert!(highscore.is_some());
            let highscore = highscore.unwrap();
            assert_eq!(1, highscore.len());
            highscore.store(0, Highscore::new(3, 0));
        }
        let highscore = records.get(&"Thorsten".to_string());
        assert!(highscore.is_some());
        assert_eq!(2, highscore.unwrap().len());
        let (timestamp, score) = highscore.unwrap().best().unwrap();
        assert_eq!(0, timestamp);
        assert_eq!(0, score.coins());
        assert_eq!(3, score.moves());
    }

    #[test]
    fn test_multiple_players() {
        let mut records = HighscoreRecords::new([0u8; 32]);
        assert_eq!(0, records.len());
        records.put("Thorsten".to_string(), 1337, Highscore::new(5, 0));
        assert_eq!(1, records.len());
        records.put("Dieter".to_string(), 1337, Highscore::new(5, 0));
        assert_eq!(2, records.len());
        records.put("Frank".to_string(), 1337, Highscore::new(5, 0));
        assert_eq!(3, records.len());
        records.put("Bernhard".to_string(), 1337, Highscore::new(5, 0));
        assert_eq!(4, records.len());
        records.put("Thorsten".to_string(), 1337, Highscore::new(5, 0));
        assert_eq!(4, records.len());
    }
}
