use std::{fs};
use std::io::{BufReader};
use std::path::Path;
use crate::world::GameWorld;

///
/// This file contains code used to manipulate physical data representing game worlds.
///

impl GameWorld {
    /// Load a map from the given file.
    pub fn load_from_file(file: &Path) -> std::io::Result<Self> {
        let filepath = file.to_str();
        let file = fs::File::open(file)?;
        let buf = BufReader::new(file);
        let mut deserialized: Self = serde_json::from_reader(buf)?;
        if let Some(fname) = filepath {
            deserialized.set_filename(fname);
        } else {
            deserialized.remove_filename();
        }
        Ok(deserialized)
    }
    /// Save the map to the given file.
    pub fn save_to_file(&self, file: &Path) -> std::io::Result<()> {
        // TODO: Use a buffered writer to avoid having to copy over the entire file content to RAM
        fs::write(file, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}