use crate::highscores::highscore::Highscore;
use crate::highscores::highscore_records::HighscoreRecords;
use std::collections::HashMap;

/// A highscores database, containing the highscores for an arbitrary number of maps
pub struct HighscoresDatabase {
    records: HashMap<[u8; 32], HighscoreRecords>,
}

impl Default for HighscoresDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl HighscoresDatabase {
    /// Initialize a new empty database
    pub fn new() -> Self {
        HighscoresDatabase {
            records: HashMap::new(),
        }
    }
    /// Put the given highscore into the database, creating all necessary data structures automatically
    pub fn put(
        &mut self,
        map: [u8; 32],
        player: String,
        timestamp: i64,
        highscore: Highscore,
    ) -> &mut Self {
        self.records
            .entry(map)
            .or_insert_with(|| HighscoreRecords::new(map))
            .put(player, timestamp, highscore);
        self
    }
    /// Put the given highscore with the current time into the database, creating all necessary data structures automatically
    pub fn put_with_current_time(
        &mut self,
        map: [u8; 32],
        player: String,
        highscore: Highscore,
    ) -> &mut Self {
        self.records
            .entry(map)
            .or_insert_with(|| HighscoreRecords::new(map))
            .put_with_current_time(player, highscore);
        self
    }
    /// Get the (best) highscore for the given map and player
    pub fn get_best(&self, map: &[u8; 32], player: &String) -> Option<(i64, &Highscore)> {
        self.records
            .get(map)
            .and_then(|highscore_records| highscore_records.get_best(player))
            .map(|(timestamp, highscore)| (timestamp, highscore))
    }
    /// Get the size, i.e., the number of maps stored in this database
    pub fn len(&self) -> usize {
        self.records.len()
    }
    /// Check if this highscores database is empty
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
    /// Get the highscore record for the given map
    pub fn get(&self, map: &[u8; 32]) -> Option<&HighscoreRecords> {
        self.records.get(map)
    }
    /// Get the highscore record for the given map as mutable reference
    pub fn get_mut(&mut self, map: &[u8; 32]) -> Option<&mut HighscoreRecords> {
        self.records.get_mut(map)
    }
}

#[cfg(test)]
mod tests {
    use crate::highscores::highscore::Highscore;
    use crate::highscores::highscores_database::HighscoresDatabase;
    use std::time::Duration;

    #[test]
    fn test_len_default() {
        let database = HighscoresDatabase::new();
        assert_eq!(0, database.len());
    }

    #[test]
    fn test_get_default() {
        let database = HighscoresDatabase::new();
        let res = database.get(&[0u8; 32]);
        assert!(res.is_none());
    }

    #[test]
    fn test_get_mut_default() {
        let mut database = HighscoresDatabase::new();
        let res = database.get_mut(&[0u8; 32]);
        assert!(res.is_none());
    }

    #[test]
    fn test_get_best_default() {
        let database = HighscoresDatabase::new();
        let res = database.get_best(&[0u8; 32], &"Max".to_string());
        assert!(res.is_none());
    }

    #[test]
    fn test_multiple_highscores() {
        let mut database = HighscoresDatabase::new();
        database.put(
            [0u8; 32],
            "Thorsten".to_string(),
            1337,
            Highscore::new(10, 0),
        );
        assert_eq!(1, database.len());
        assert_eq!(1, database.get(&[0u8; 32]).unwrap().len());
        assert_eq!(
            1,
            database
                .get(&[0u8; 32])
                .unwrap()
                .get(&"Thorsten".to_string())
                .unwrap()
                .len()
        );

        database.put([0u8; 32], "Thorsten".to_string(), 69, Highscore::new(10, 0));
        assert_eq!(1, database.len());
        assert_eq!(1, database.get(&[0u8; 32]).unwrap().len());
        assert_eq!(
            2,
            database
                .get(&[0u8; 32])
                .unwrap()
                .get(&"Thorsten".to_string())
                .unwrap()
                .len()
        );

        database.put([0u8; 32], "Frank".to_string(), 1337, Highscore::new(10, 0));
        assert_eq!(1, database.len());
        assert_eq!(2, database.get(&[0u8; 32]).unwrap().len());
        assert_eq!(
            2,
            database
                .get(&[0u8; 32])
                .unwrap()
                .get(&"Thorsten".to_string())
                .unwrap()
                .len()
        );
        assert_eq!(
            1,
            database
                .get(&[0u8; 32])
                .unwrap()
                .get(&"Frank".to_string())
                .unwrap()
                .len()
        );

        database.put(
            [1u8; 32],
            "Thorsten".to_string(),
            1337,
            Highscore::new(10, 0),
        );
        assert_eq!(2, database.len());
        assert_eq!(2, database.get(&[0u8; 32]).unwrap().len());
        assert_eq!(
            2,
            database
                .get(&[0u8; 32])
                .unwrap()
                .get(&"Thorsten".to_string())
                .unwrap()
                .len()
        );
        assert_eq!(
            1,
            database
                .get(&[0u8; 32])
                .unwrap()
                .get(&"Frank".to_string())
                .unwrap()
                .len()
        );
        assert_eq!(1, database.get(&[1u8; 32]).unwrap().len());
        assert_eq!(
            1,
            database
                .get(&[1u8; 32])
                .unwrap()
                .get(&"Thorsten".to_string())
                .unwrap()
                .len()
        );

        let (time, best1) = database
            .get_best(&[0u8; 32], &"Thorsten".to_string())
            .unwrap();
        assert_eq!(1337, time);
        assert_eq!(10, best1.moves());
        assert_eq!(0, best1.coins());
    }

    #[test]
    pub fn test_put_with_current_time() {
        let mut database = HighscoresDatabase::new();
        database.put_with_current_time([0u8; 32], "Leo".to_string(), Highscore::new(3, 2));
        std::thread::sleep(Duration::from_millis(2));
        database.put_with_current_time([0u8; 32], "Leo".to_string(), Highscore::new(3, 1));
        std::thread::sleep(Duration::from_millis(2));
        assert_eq!(1, database.len());
        assert_eq!(1, database.get(&[0u8; 32]).unwrap().len());
        assert_eq!(
            2,
            database
                .get(&[0u8; 32])
                .unwrap()
                .get(&"Leo".to_string())
                .unwrap()
                .len()
        );
        let (_, best) = database.get_best(&[0u8; 32], &"Leo".to_string()).unwrap();
        assert_eq!(3, best.moves());
        assert_eq!(2, best.coins());
    }
}
