use std::{fs};
use std::io::{BufReader};
use std::path::{Path, PathBuf};
use crate::world::GameWorld;

///
/// This file contains code used to manipulate physical data representing game worlds.
///

impl GameWorld {
    /// Load a map from the given file.
    pub fn load_from_file(file: &Path) -> std::io::Result<Self> {
        let filepath = file.clone();
        let file = fs::File::open(file)?;
        let buf = BufReader::new(file);
        let mut deserialized: Self = serde_json::from_reader(buf)?;
        let mut p = PathBuf::new();
        p.push(filepath);
        deserialized.set_filename(p);
        Ok(deserialized)
    }
    /// Save the map to the given file.
    pub fn save_to_file(&self, file: &Path) -> std::io::Result<()> {
        // TODO: Use a buffered writer to avoid having to copy over the entire file content to RAM
        fs::write(file, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}