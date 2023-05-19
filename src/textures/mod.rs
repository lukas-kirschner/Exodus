use crate::textures::fonts::egui_fonts;
use crate::textures::tileset_manager::{
    file_name_for_tileset, find_handle_with_path, RpgSpriteHandles, TilesetManager,
};
use crate::AppState;
use bevy::asset::LoadState;
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
        app.add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_textures))
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(egui_fonts))
            .add_system_set(
                SystemSet::on_update(AppState::Loading).with_system(check_and_init_textures),
            );
    }

    fn name(&self) -> &str {
        "Textures Handler"
    }
}
fn load_asset_folder_or_panic(asset_server: &AssetServer, path: &str) -> Vec<HandleUntyped> {
    asset_server
        .load_folder(path)
        .unwrap_or_else(|_| panic!("Could not find asset folder at {}", path))
}

fn load_textures(mut rpg_sprite_handles: ResMut<RpgSpriteHandles>, asset_server: Res<AssetServer>) {
    // Load the textures - Bevy takes care of resolving the paths, see https://bevy-cheatbook.github.io/assets/assetserver.html
    rpg_sprite_handles.handles = load_asset_folder_or_panic(&asset_server, "textures/tilesets");
}

fn check_and_init_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut tileset_manager: ResMut<TilesetManager>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        // Load Tilesets
        for tileset in Tileset::iter() {
            let tileset: Tileset = tileset;
            let mut textures_folder = PathBuf::from("tilesets");
            textures_folder.push(file_name_for_tileset(&tileset));
            let handle = find_handle_with_path(
                textures_folder.as_path(),
                &asset_server,
                &sprite_handles.handles,
            );
            let texture_atlas = TextureAtlas::from_grid(
                handle.clone(),
                Vec2::splat(tileset.texture_size() as f32),
                16,
                16,
                None,
                None,
            );
            let atlas_handle = texture_atlases.add(texture_atlas);
            tileset_manager.set_handle(tileset, atlas_handle);
            debug!(
                "Successfully loaded texture atlas {0} with tile size {1}x{1}",
                asset_server
                    .get_handle_path(handle)
                    .unwrap()
                    .path()
                    .to_str()
                    .unwrap(),
                tileset.texture_size()
            );
        }
        // Finish loading and start the main menu
        state.set(AppState::MainMenu).unwrap();
    }
}
