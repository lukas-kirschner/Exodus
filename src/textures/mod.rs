use crate::textures::fonts::egui_fonts;
use crate::textures::tileset_manager::{file_name_for_tileset, ImageHandles, TilesetManager};
use crate::{AllAssetHandles, AppState};
use bevy::asset::{LoadedFolder, RecursiveDependencyLoadState};
use bevy::prelude::*;
use libexodus::tilesets::Tileset;
use std::path::PathBuf;
use strum::IntoEnumIterator;

pub mod egui_textures;
pub mod fonts;
pub mod tileset_manager;

/// The Textures Plugin that takes care of loading textures
pub struct Textures;

impl Plugin for Textures {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), load_textures)
            .add_systems(OnEnter(AppState::Loading), egui_fonts)
            .add_systems(
                Update,
                check_and_init_textures.run_if(in_state(AppState::Loading)),
            );
    }

    fn name(&self) -> &str {
        "Textures Handler"
    }
}
/// Begin loading the textures through the AssetServer.
/// This will initialize the Texture Handles, but the actual loading will take place concurrently in the background.
fn load_textures(
    mut rpg_sprite_handles: ResMut<ImageHandles>,
    asset_server: Res<AssetServer>,
    mut all_assets: ResMut<AllAssetHandles>,
) {
    // Load the textures - Bevy takes care of resolving the paths, see https://bevy-cheatbook.github.io/assets/assetserver.html
    rpg_sprite_handles.handles = asset_server.load_folder("textures/tilesets");
    all_assets.handles.push(rpg_sprite_handles.handles.clone());
}
/// This function checks repeatedly whether all textures have been loaded successfully in the background.
/// If this is the case, it will trigger the next stage and initialize the texture slices.
fn check_and_init_textures(
    mut state: ResMut<NextState<AppState>>,
    sprite_handles: ResMut<ImageHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut tileset_manager: ResMut<TilesetManager>,
    all_assets: Res<AllAssetHandles>,
    mut folder_assets: ResMut<Assets<LoadedFolder>>,
) {
    let all_loaded = all_assets
        .handles
        .iter()
        .map(|folder_handle| {
            asset_server
                .get_recursive_dependency_load_state(folder_handle)
                .map(|state| matches!(state, RecursiveDependencyLoadState::Loaded))
                .unwrap_or(false)
        })
        .all(|b| b);
    if all_loaded {
        // Remove all handles inside the textures folder, because they are only needed here
        let all_texture_handles: Vec<UntypedHandle> = folder_assets
            .remove(sprite_handles.handles.id())
            .unwrap()
            .handles;
        // Load Tilesets
        for tileset in Tileset::iter() {
            let tileset: Tileset = tileset;
            let mut textures_folder = PathBuf::from("tilesets");
            textures_folder.push(file_name_for_tileset(&tileset));
            // Get handle for texture pack with correct path
            let handle = all_texture_handles
                .iter()
                .find(|handle| {
                    asset_server
                        .get_path(handle.id())
                        .unwrap()
                        .path()
                        .ends_with(textures_folder.as_path())
                })
                .unwrap_or_else(|| {
                    panic!("Texture not found: {}", textures_folder.to_str().unwrap())
                });
            let texture_atlas = TextureAtlasLayout::from_grid(
                Vec2::splat(tileset.texture_size() as f32),
                16,
                16,
                None,
                None,
            );
            let atlas_size = texture_atlas.size;
            let atlas_handle = texture_atlases.add(texture_atlas);
            tileset_manager.set_handle(tileset, atlas_handle.clone(), handle.clone().typed());
            debug!(
                "Successfully loaded texture atlas {0} with tile size {1}x{1} and atlas size {2}x{3}",
                asset_server
                    .get_path(handle.id())
                    .unwrap()
                    .path()
                    .to_str()
                    .unwrap(),
                tileset.texture_size(),
                atlas_size.x,atlas_size.y
            );
        }
        // Finish loading and start the processing
        state.set(AppState::Process);
    }
}
