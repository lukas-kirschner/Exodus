///
/// This file contains code used to build and query file system directories
/// and does not contain any save/load or other functional logic.
///
use std::error::Error;
use std::{fmt};
use std::fmt::Formatter;
use std::fs::metadata;
use std::path::{PathBuf};
use directories::ProjectDirs;
use walkdir::{WalkDir};

#[derive(Debug)]
pub struct InvalidSystemConfigurationError {
    error_message: String,
}

impl fmt::Display for InvalidSystemConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

impl Error for InvalidSystemConfigurationError {
    fn description(&self) -> &str {
        self.error_message.as_str()
    }
}

#[derive(Debug)]
enum InvalidMapNameError {
    EmptyName
}

impl fmt::Display for InvalidMapNameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            InvalidMapNameError::EmptyName => { "Empty file name!" }
        })
    }
}

impl Error for InvalidMapNameError {}

/// A data type that includes all system paths and methods to query files and directories.
pub struct GameDirectories {
    base_dir: PathBuf,
    pub maps_dir: PathBuf,
    pub config_dir: PathBuf,
}

impl GameDirectories {
    const MAP_FILE_SUFFIX: &'static str = "exm";

    /// Automatically get the game directories from the system directories.
    /// Returns an `InvalidSystemConfigurationError` if the system variables are set up incorrectly.
    pub fn from_system_config() -> Result<Self, InvalidSystemConfigurationError> {
        let basedir_struct: ProjectDirs = ProjectDirs::from("exodus.game", "", env!("CARGO_PKG_NAME"))
            .ok_or(InvalidSystemConfigurationError {
                error_message: String::from("No valid home directory could be determined from your operating system's system variables!\
            Please set the appropriate variables.")
            })?;
        let game_base_dir = basedir_struct.data_dir();
        let game_maps_dir = game_base_dir.join("maps");
        let game_config_dir = game_base_dir.join("config");
        Ok(
            GameDirectories {
                base_dir: PathBuf::from(game_base_dir),
                maps_dir: PathBuf::from(game_maps_dir),
                config_dir: PathBuf::from(game_config_dir),
            }
        )
    }

    /// Iterate over all map files that are found inside the maps folder and all subfolders.
    pub fn iter_maps(self: &Self) -> impl Iterator<Item=walkdir::DirEntry> + '_ {
        WalkDir::new(&self.maps_dir)
            .into_iter()
            .filter_map(|e| e.map_err(|error| {
                eprintln!("{}", error);
                error
            }).ok())
            .filter(|file| {
                let meta = metadata(file.path());
                meta.is_ok() && meta.unwrap().is_file()
            })
            .filter(|file| {
                file.path().extension()
                    .and_then(|s| Some(s.to_ascii_lowercase()))
                    .unwrap_or("".into())
                    == GameDirectories::MAP_FILE_SUFFIX.to_ascii_lowercase().as_str()
            })
    }
    /// Get the path of a map with the given name.
    /// Automatically converts the map name to a file name by replacing whitespaces with underscores
    ///  and converting all printable characters to lower-case.
    /// If the file name that results from converting the name is invalid, an error is returned.
    fn path_from_mapname(self: &Self, map_name: &str) -> Result<PathBuf, InvalidMapNameError> {
        let map_file_name: String = map_name.trim().chars().map(|c| {
            if c.is_whitespace() {
                '_'
            } else {
                c.to_ascii_lowercase()
            }
        }).collect();
        if map_file_name.is_empty() {
            return Err(InvalidMapNameError::EmptyName);
        }
        let map_folder: PathBuf = self.maps_dir.join(format!("{}.{}", map_file_name, GameDirectories::MAP_FILE_SUFFIX));
        Ok(map_folder)
    }
}