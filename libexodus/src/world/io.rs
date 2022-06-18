use std::{fs};
use std::error::Error;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use crate::world::GameWorld;

///
/// This file contains code used to manipulate physical data representing game worlds.
///

impl GameWorld {
    /// Load a map from the given file.
    pub fn load_from_file(file: &Path) -> std::io::Result<Self> {
        let mut file = fs::File::open(file)?;
        let mut buf = BufReader::new(file);
        let deserialized: Self = serde_json::from_reader(buf)?;
        Ok(deserialized)
    }
    /// Save the map to the given file.
    pub fn save_to_file(&self, file: &Path) -> std::io::Result<()> {
        let mut file: fs::File = fs::File::open(file)?;
        let mut buf = BufWriter::new(file);
        serde_json::to_writer(buf, self)?;
        Ok(())
    }
}