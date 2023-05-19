use crate::TilesetManager;
use bevy::prelude::*;
use bevy_egui::egui::TextureId;
use bevy_egui::{egui, EguiContext, EguiContexts};
use libexodus::player::Player;
use libexodus::tiles::{AtlasIndex, Tile, UITiles};
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Resource)]
pub struct EguiButtonTextures {
    pub textures: HashMap<AtlasIndex, (TextureId, egui::Vec2, egui::Rect)>,
}

impl FromWorld for EguiButtonTextures {
    fn from_world(_: &mut World) -> Self {
        EguiButtonTextures {
            textures: HashMap::new(),
        }
    }
}

fn convert(
    texture_atlas: &TextureAtlas,
    texture_handle: &Handle<Image>,
    egui_ctx: &mut EguiContexts,
    atlas_index: &AtlasIndex,
) -> (TextureId, egui::Vec2, egui::Rect) {
    // TODO Up/downscale to egui texture size (32px)
    let rect: Rect = texture_atlas.textures[*atlas_index];
    let uv: egui::Rect = egui::Rect::from_min_max(
        egui::pos2(
            rect.min.x / texture_atlas.size.x,
            rect.min.y / texture_atlas.size.y,
        ),
        egui::pos2(
            rect.max.x / texture_atlas.size.x,
            rect.max.y / texture_atlas.size.y,
        ),
    );
    // Convert bevy::prelude::Image to bevy_egui::egui::TextureId?
    let tex: TextureId = egui_ctx.add_image(texture_handle.clone_weak());
    (tex, egui::Vec2::splat(32.0), uv)
    // TODO if the button size is smaller than the texture size, Egui textures need to be resized here
}

/// Convert Bevy Textures to Egui Textures to show those on the buttons
pub fn atlas_to_egui_textures(
    texture_atlas_handle: Res<TilesetManager>,
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut egui_ctx: EguiContexts,
) {
    let texture_atlas: &TextureAtlas = texture_atlases
        .get(&texture_atlas_handle.current_handle())
        .expect("The texture atlas of the tile set has not yet been loaded!");
    let texture_handle: &Handle<Image> = &texture_atlas.texture;
    let mut textures = HashMap::new();
    // Convert game world tiles
    for tile in Tile::iter() {
        if let Some(atlas_index) = tile.atlas_index() {
            textures.insert(
                atlas_index,
                convert(texture_atlas, texture_handle, &mut egui_ctx, &atlas_index),
            );
        }
    }
    // Convert Button Textures
    for extratexture in UITiles::iter() {
        if let Some(atlas_index) = extratexture.atlas_index() {
            textures.insert(
                atlas_index,
                convert(texture_atlas, texture_handle, &mut egui_ctx, &atlas_index),
            );
        }
    }

    // Convert Player Texture for the Player Spawn Button
    let player = Player::new();
    textures.insert(
        player.atlas_index(),
        convert(
            texture_atlas,
            texture_handle,
            &mut egui_ctx,
            &player.atlas_index(),
        ),
    );

    commands.insert_resource(EguiButtonTextures { textures });
}
