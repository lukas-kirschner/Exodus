use directories::ProjectDirs;
///
/// This file contains code used to build and query file system directories
/// and does not contain any save/load or other functional logic.
///
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::fs::metadata;
use std::path::PathBuf;
use walkdir::WalkDir;

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
pub enum InvalidMapNameError {
    EmptyName,
    InvalidCharacter { c: char },
}

impl fmt::Display for InvalidMapNameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InvalidMapNameError::EmptyName => {
                write!(f, "Empty file name!")
            },
            InvalidMapNameError::InvalidCharacter { c } => {
                write!(f, "Invalid character in map file name: {}", c)
            },
        }
    }
}

impl Error for InvalidMapNameError {}

/// A data type that includes all system paths and methods to query files and directories.
pub struct GameDirectories {
    _base_dir: PathBuf,
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
        Ok(GameDirectories {
            _base_dir: PathBuf::from(game_base_dir),
            maps_dir: game_maps_dir,
            config_dir: game_config_dir,
        })
    }

    /// Iterate over all map files that are found inside the maps folder and all subfolders.
    pub fn iter_maps(&self) -> impl Iterator<Item = walkdir::DirEntry> + '_ {
        WalkDir::new(&self.maps_dir)
            .into_iter()
            .filter_map(|e| {
                e.map_err(|error| {
                    eprintln!("{}", error);
                    error
                })
                .ok()
            })
            .filter(|file| {
                let meta = metadata(file.path());
                meta.is_ok() && meta.unwrap().is_file()
            })
            .filter(|file| {
                let ret = file
                    .path()
                    .extension()
                    .map(|s| s.to_ascii_lowercase())
                    .unwrap_or_else(|| "".into())
                    == GameDirectories::MAP_FILE_SUFFIX
                        .to_ascii_lowercase()
                        .as_str();
                if cfg!(debug_assertions) && !ret {
                    println!(
                        "Skipped {} because its file extension did not match.",
                        file.path().to_str().unwrap()
                    );
                }

                ret
            })
    }
    /// Get the path of a map with the given name.
    /// Automatically converts the map name to a file name by replacing whitespaces with underscores
    ///  and converting all printable characters to lower-case.
    /// If the file name that results from converting the name is invalid, an error is returned.
    pub fn path_from_mapname(&self, map_name: &str) -> Result<PathBuf, InvalidMapNameError> {
        let map_file_name: String = map_name
            .trim()
            .chars()
            .map(|c| {
                if c.is_whitespace() {
                    '_'
                } else {
                    c.to_ascii_lowercase()
                }
            })
            .collect();
        if map_file_name.is_empty() {
            return Err(InvalidMapNameError::EmptyName);
        }
        let map_folder: PathBuf = self.maps_dir.join(format!(
            "{}.{}",
            map_file_name,
            GameDirectories::MAP_FILE_SUFFIX
        ));
        Ok(map_folder)
    }

    /// Get the path of a map with the given user input path.
    /// The user input is sanitized and resolved as subdirectory of the maps folder.
    /// If the file name that results from converting the name is invalid, an error is returned.
    pub fn path_from_userinput(&self, user_input: &str) -> Result<PathBuf, InvalidMapNameError> {
        let user_input_t = user_input.trim();
        let map_subdir_name: Result<String, InvalidMapNameError> = user_input_t
            .chars()
            .map(|c: char| {
                match c {
                    // see https://is.gd/VLNWIM
                    // TODO This needs to be changed once we support subdirectories
                    '<' | '>' | ':' | '"' | '|' | '/' | '\\' | '?' | '*' => {
                        Err(InvalidMapNameError::InvalidCharacter { c })
                    },
                    _ => {
                        if c.is_whitespace() {
                            Ok('_')
                        } else {
                            Ok(c.to_ascii_lowercase())
                        }
                    },
                }
            })
            .collect();
        let map_subdir_name: String = map_subdir_name?;
        if map_subdir_name.is_empty() {
            return Err(InvalidMapNameError::EmptyName);
        }
        let map_file_name: String = if map_subdir_name
            .as_str()
            .ends_with(format!(".{}", GameDirectories::MAP_FILE_SUFFIX).as_str())
        {
            map_subdir_name
        } else {
            format!("{}.{}", map_subdir_name, GameDirectories::MAP_FILE_SUFFIX)
        };
        let map_folder: PathBuf = self.maps_dir.join(map_file_name);
        Ok(map_folder)
    }

    pub fn config_file(&self) -> PathBuf {
        self.config_dir.as_path().join("config.exc")
    }
}
