use crate::directories::InvalidMapNameError::{InvalidPath, NotASubpath};
use directories::ProjectDirs;
///
/// This file contains code used to build and query file system directories
/// and does not contain any save/load or other functional logic.
///
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::fs::metadata;
use std::path::{Path, PathBuf, StripPrefixError};
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
    InvalidPath { p: PathBuf },
    NotASubpath { e: StripPrefixError },
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
            InvalidPath { p } => {
                write!(f, "Invalid path: {:?}", p)
            },
            NotASubpath { e } => {
                write!(f, "Not a sub path of map folder: {}", e)
            },
        }
    }
}

impl From<StripPrefixError> for InvalidMapNameError {
    fn from(value: StripPrefixError) -> Self {
        InvalidMapNameError::NotASubpath { e: value }
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

    pub fn relative_map_dir_from_path(&self, path: &Path) -> Result<String, InvalidMapNameError> {
        let resolved_path = path.strip_prefix(&self.maps_dir)?;
        resolved_path
            .as_os_str()
            .to_str()
            .ok_or(InvalidPath {
                p: path.to_path_buf(),
            })
            .map(|s| s.to_string())
    }

    /// Iterate over all map files that are found inside the maps folder and all subfolders.
    /// Follow symlinks on the way.
    pub fn iter_maps(&self) -> impl Iterator<Item = walkdir::DirEntry> + '_ {
        WalkDir::new(&self.maps_dir)
            .follow_links(true)
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

    /// Get the path of a map with the given user input path.
    /// The user input is sanitized and resolved as subdirectory relative to the maps folder.
    /// If the file name that results from converting the name is invalid, an error is returned.
    pub fn path_from_userinput(&self, user_input: &str) -> Result<PathBuf, InvalidMapNameError> {
        let user_input_t = user_input.trim();
        let map_subdir_name: Result<String, InvalidMapNameError> = user_input_t
            .chars()
            .map(|c: char| {
                match c {
                    // see https://is.gd/VLNWIM
                    '<' | '>' | ':' | '"' | '|' | '\\' | '?' | '*' => {
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
        let map_file_name: String = Self::assure_map_extension(map_subdir_name);
        let mut map_folder: Option<PathBuf> = None;
        for path_part in map_file_name.split('/') {
            // We split 'stupidly' by the '/' character because we define it to be the only path separator in a user input. This is actually the desired behavior.
            if !path_part.is_empty() {
                if map_folder.is_none() {
                    map_folder = Some(PathBuf::from(path_part))
                } else {
                    map_folder = Some(map_folder.unwrap().as_path().join(PathBuf::from(path_part)));
                }
            }
        }
        match &map_folder {
            None => unreachable!(),
            Some(path) => {
                let mut ret: PathBuf = self.maps_dir.clone();
                ret.push(path);
                Ok(ret)
            },
        }
    }

    /// Make sure the given path has the .exm extension
    fn assure_map_extension(map_subdir_name: String) -> String {
        if map_subdir_name
            .as_str()
            .ends_with(format!(".{}", GameDirectories::MAP_FILE_SUFFIX).as_str())
        {
            map_subdir_name
        } else {
            format!("{}.{}", map_subdir_name, GameDirectories::MAP_FILE_SUFFIX)
        }
    }

    pub fn config_file(&self) -> PathBuf {
        self.config_dir.as_path().join("config.exc")
    }
    pub fn highscores_file(&self) -> PathBuf {
        self.config_dir.as_path().join("score.exh")
    }
}
#[cfg(test)]
impl GameDirectories {
    pub fn Mock(_base_dir: PathBuf, maps_dir: PathBuf, config_dir: PathBuf) -> Self {
        GameDirectories {
            _base_dir,
            maps_dir,
            config_dir,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use bincode::ErrorKind;
    use bytebuffer::ByteBuffer;
    use strum::{EnumCount, IntoEnumIterator};
    macro_rules! assert_map_path_resolves_to {
        ($name:ident: $basedir:expr, $mapinput:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let directories = GameDirectories::Mock("".into(), $basedir.into(), "".into());
                let result = directories.path_from_userinput($mapinput);
                assert!(result.is_ok());
                assert_eq!(result.as_ref().unwrap(), &PathBuf::from($expected));
            }
        };
    }
    macro_rules! assert_map_path_resolves_to_and_back {
        ($name:ident: $basedir:expr, $mapinput:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let directories = GameDirectories::Mock("".into(), $basedir.into(), "".into());
                let result = directories.path_from_userinput($mapinput);
                assert!(result.is_ok());
                assert_eq!(result.as_ref().unwrap(), &PathBuf::from($expected));
                let path_back = directories.relative_map_dir_from_path(&result.unwrap());
                assert!(path_back.is_ok());
                assert_eq!(path_back.unwrap(), $mapinput);
            }
        };
    }
    macro_rules! assert_map_errors {
        ($name:ident: $basedir:expr, $mapinput:expr, $offending_character:expr) => {
            #[test]
            fn $name() {
                let directories = GameDirectories::Mock("".into(), $basedir.into(), "".into());
                let result = directories.path_from_userinput($mapinput);
                assert!(result.is_err());
                assert!(matches!(
                    result.unwrap_err(),
                    InvalidMapNameError::InvalidCharacter {
                        c: $offending_character
                    }
                ));
            }
        };
    }

    assert_map_path_resolves_to_and_back!(map_from_simple: "/var/maps", "testmap.exm", "/var/maps/testmap.exm");
    assert_map_path_resolves_to!(map_from_with_whitespace: "/var/maps", "testmap 2.exm", "/var/maps/testmap_2.exm");
    assert_map_path_resolves_to!(map_from_without_extension: "/var/maps", "testmap 1234 new", "/var/maps/testmap_1234_new.exm");
    assert_map_path_resolves_to!(map_from_uppercase: "/var/maps", "TestMap 1234 NEW", "/var/maps/testmap_1234_new.exm");
    assert_map_path_resolves_to_and_back!(map_from_in_root: "/", "testmap.exm", "/testmap.exm");
    assert_map_path_resolves_to_and_back!(map_from_map_folder: "/home/user/.local/share/libexodus/maps", "testmap.exm", "/home/user/.local/share/libexodus/maps/testmap.exm");
    assert_map_path_resolves_to_and_back!(map_from_map_subfolder: "/home/user/.local/share/libexodus/maps", "campaign/testmap.exm", "/home/user/.local/share/libexodus/maps/campaign/testmap.exm");

    assert_map_errors!(map_with_lt: "/home", "test<map.exm", '<');
    assert_map_errors!(map_with_gt: "/home", "testmap>.exm", '>');
    assert_map_errors!(map_with_colon: "/home", "test:map.exm", ':');
    assert_map_errors!(map_with_quotes: "/home", "\"testmap.exm\"", '\"');
    assert_map_errors!(map_with_pipe: "/home", "testmap|.exm", '|');
    assert_map_errors!(map_with_backslash: "/home", "testmap\\.exm", '\\');
    assert_map_errors!(map_with_question: "/home", "testmap?.exm", '?');
    assert_map_errors!(map_with_asterisk: "/home", "testmap*.exm", '*');

    #[test]
    fn test_empty_map() {
        let directories = GameDirectories::Mock("".into(), "/maps".into(), "".into());
        let result = directories.path_from_userinput("");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InvalidMapNameError::EmptyName
        ))
    }
}
