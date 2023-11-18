use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use libexodus::tilesets::Tileset;
use std::collections::HashMap;

#[derive(Resource)]
/// A struct that contains the handles for all tile sets and information about the current tile set
pub struct TilesetManager {
    pub current_tileset: Tileset,
    atlas_handle_for_tileset: HashMap<Tileset, Handle<TextureAtlas>>,
}

impl FromWorld for TilesetManager {
    fn from_world(_: &mut World) -> Self {
        TilesetManager {
            current_tileset: Tileset::TinyPlatformQuestTiles,
            atlas_handle_for_tileset: HashMap::new(),
        }
    }
}

impl TilesetManager {
    pub fn set_handle(&mut self, tileset: Tileset, handle: Handle<TextureAtlas>) {
        self.atlas_handle_for_tileset.insert(tileset, handle);
    }
    /// Get a clone of the current handle
    pub fn current_handle(&self) -> Handle<TextureAtlas> {
        self.atlas_handle_for_tileset
            .get(&self.current_tileset)
            .unwrap_or_else(|| {
                panic!(
                    "No Texture Atlas was initialized for {}",
                    self.current_tileset
                )
            })
            .clone()
    }
    pub fn current_tileset(&self) -> &Tileset {
        &self.current_tileset
    }
}

/// Get the file name of the tile set
pub fn file_name_for_tileset(tileset: &Tileset) -> &str {
    match tileset {
        Tileset::TinyPlatformQuestTiles => "Tiny_Platform_Quest_Tiles.png",
        Tileset::Classic => "Classic.png",
    }
}

#[derive(Resource)]
/// A struct containing all loaded handles from the tilesets folder
pub struct RpgSpriteHandles {
    pub handles: Handle<LoadedFolder>,
}

impl FromWorld for RpgSpriteHandles {
    fn from_world(_: &mut World) -> Self {
        RpgSpriteHandles {
            handles: Default::default(),
        }
    }
}
