use crate::exodus_serializable::ExodusSerializable;
use crate::highscores::highscore::Highscore;
use crate::highscores::highscore_records::HighscoreRecords;
use crate::highscores::io_error::HighscoreParseError;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

//00000000: 4578 6f64 7573 4869 6768 7363 6f72 6544  ExodusHighscoreD
//00000010: 420a                                     B.
pub(crate) const MAGICBYTES: [u8; 18] = [
    0x45, 0x78, 0x6f, 0x64, 0x75, 0x73, 0x48, 0x69, 0x67, 0x68, 0x73, 0x63, 0x6f, 0x72, 0x65, 0x44,
    0x42, 0x0a,
];

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
    /// Load a HighscoreDatabase from the given file
    pub fn load_from_file(path: &Path) -> Result<Self, HighscoreParseError> {
        let file = OpenOptions::new().read(true).open(path)?;
        let mut buf = BufReader::new(file);
        let mut ret = HighscoresDatabase::default();
        ret.parse(&mut buf)?;
        Ok(ret)
    }
    /// Save a HighscoreDatabase to the given file
    pub fn save_to_file(&self, path: &Path) -> Result<(), HighscoreParseError> {
        let file: File = OpenOptions::new().create(true).write(true).open(path)?;
        let mut buf = BufWriter::new(file);
        self.serialize(&mut buf)?;
        Ok(())
    }
}

// Serialization Code
impl ExodusSerializable for HighscoresDatabase {
    const CURRENT_VERSION: u8 = 0x01;
    type ParseError = HighscoreParseError;

    fn serialize<T: Write>(&self, file: &mut T) -> Result<(), Self::ParseError> {
        // Write magic bytes
        file.write_all(&MAGICBYTES)?;

        // Write Database Version
        file.write_all(&[Self::CURRENT_VERSION])?;

        // Write length of the database
        let b_length = bincode::serialize(&self.records.len())?;
        file.write_all(&b_length)?;

        for entry in &self.records {
            // Write Map Hash
            file.write_all(entry.0)?;

            // Write Highscore Record for map
            entry.1.serialize(file)?;
        }

        Ok(())
    }

    fn parse<T: Read>(&mut self, file: &mut T) -> Result<(), Self::ParseError> {
        // Parse Magic Bytes
        let mut buf: [u8; MAGICBYTES.len()] = [0; MAGICBYTES.len()];
        file.read_exact(&mut buf)?;
        if buf != MAGICBYTES {
            return Err(Self::ParseError::InvalidMagicBytes {
                expected: MAGICBYTES,
                actual: buf,
            });
        }

        // Parse Database Format
        let mut buf: [u8; 1] = [0; 1];
        file.read_exact(&mut buf)?;
        match buf[0] {
            Self::CURRENT_VERSION => self.parse_current_version(file),
            // Add older versions here
            _ => {
                return Err(Self::ParseError::InvalidVersion {
                    invalid_version: buf[0],
                })
            },
        }?;

        Ok(())
    }

    fn parse_current_version<T: Read>(&mut self, file: &mut T) -> Result<(), Self::ParseError> {
        // Parse length
        let db_len = bincode::deserialize_from::<&mut T, usize>(file)?;

        for _ in 0..db_len {
            let mut hash: [u8; 32] = [0u8; 32];
            file.read_exact(&mut hash)?;

            let mut record = HighscoreRecords::default();
            record.parse(file)?;

            if record.get_hash() != &hash {
                return Err(HighscoreParseError::HashMismatch {
                    expected: hash,
                    actual: *record.get_hash(),
                });
            }

            if self.records.insert(hash, record).is_some() {
                return Err(HighscoreParseError::DuplicateDatabaseEntry);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::exodus_serializable::ExodusSerializable;
    use crate::highscores::highscore::Highscore;
    use crate::highscores::highscores_database::HighscoresDatabase;
    use bytebuffer::ByteBuffer;
    use std::time::Duration;

    #[test]
    fn test_len_default() {
        let database = HighscoresDatabase::default();
        assert_eq!(0, database.len());
    }
    #[test]
    fn test_is_empty() {
        let mut database = HighscoresDatabase::new();
        assert!(database.is_empty());
        database.put(
            [0u8; 32],
            "Thorsten".to_string(),
            1337,
            Highscore::new(10, 0),
        );
        assert!(!database.is_empty());
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
        let _ = create_complex_database();
    }

    fn create_complex_database() -> HighscoresDatabase {
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
        database
    }

    #[test]
    fn test_serialize_complex_database() {
        let db = create_complex_database();
        let mut buf = ByteBuffer::new();
        let result = db.serialize(&mut buf);
        assert!(
            result.is_ok(),
            "Database failed to serialize with error: {}",
            result.unwrap_err().to_string()
        );
        buf.set_rpos(0);
        let mut db2 = HighscoresDatabase::default();
        let result = db2.parse(&mut buf);
        assert!(
            result.is_ok(),
            "Highscore Database failed to parse with error: {}",
            result.unwrap_err().to_string()
        );
        assert_eq!(db.len(), db2.len());
        assert_eq!(db.records.len(), db2.records.len());
        for (r_hash, _r_data) in &db.records {
            assert!(db2.records.contains_key(r_hash));
        }
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
