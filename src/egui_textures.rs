use std::collections::HashMap;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_egui::egui::TextureId;
use libexodus::player::Player;
use libexodus::tiles::{AtlasIndex, Tile};
use crate::{CurrentMapTextureAtlasHandle, CurrentPlayerTextureAtlasHandle};
use strum::IntoEnumIterator;

pub struct EguiButtonTextures {
    pub textures: HashMap<AtlasIndex, (TextureId, egui::Vec2, egui::Rect)>,
    pub player_textures: HashMap<AtlasIndex, (TextureId, egui::Vec2, egui::Rect)>,
}

impl FromWorld for EguiButtonTextures {
    fn from_world(_: &mut World) -> Self {
        EguiButtonTextures {
            textures: HashMap::new(),
            player_textures: HashMap::new(),
        }
    }
}

fn convert(
    texture_atlas: &TextureAtlas,
    texture_handle: &Handle<Image>,
    egui_ctx: &mut ResMut<EguiContext>,
    atlas_index: &AtlasIndex,
) -> (TextureId, egui::Vec2, egui::Rect) {
    let rect: bevy::sprite::Rect = texture_atlas.textures[*atlas_index];
    let uv: egui::Rect = egui::Rect::from_min_max(
        egui::pos2(rect.min.x / texture_atlas.size.x, rect.min.y / texture_atlas.size.y),
        egui::pos2(rect.max.x / texture_atlas.size.x, rect.max.y / texture_atlas.size.y),
    );
    let rect_vec2: egui::Vec2 = egui::Vec2::new(rect.max.x - rect.min.x, rect.max.y - rect.min.y);
    // Convert bevy::prelude::Image to bevy_egui::egui::TextureId?
    let tex: TextureId = egui_ctx.add_image(texture_handle.clone_weak());
    (tex, rect_vec2, uv)
    // TODO if the button size is smaller than the texture size, Egui textures need to be resized here
}

/// Convert Bevy Textures to Egui Textures to show those on the buttons
pub fn atlas_to_egui_textures(
    texture_atlas_handle: Res<CurrentMapTextureAtlasHandle>,
    player_atlas_handle: Res<CurrentPlayerTextureAtlasHandle>,
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut egui_ctx: ResMut<EguiContext>,
) {
    let texture_atlas: &TextureAtlas = texture_atlases.get(&texture_atlas_handle.handle).expect("The texture atlas of the tile set has not yet been loaded!");
    let texture_handle: &Handle<Image> = &texture_atlas.texture;
    let mut textures = HashMap::new();
    for tile in Tile::iter() {
        if let Some(atlas_index) = tile.atlas_index() {
            textures.insert(atlas_index, convert(texture_atlas, texture_handle, &mut egui_ctx, &atlas_index));
        }
    }
    let mut textures_p = HashMap::new();
    // The Player Spawn needs a special texture, since it comes from a different atlas:
    let player = Player::new(); // TODO The Query is not working in this stage, unfortunately
    let texture_atlas: &TextureAtlas = texture_atlases.get(&player_atlas_handle.handle).expect("The texture atlas of the player set has not yet been loaded!");
    let texture_handle: &Handle<Image> = &texture_atlas.texture;
    textures_p.insert(player.atlas_index(), convert(texture_atlas, texture_handle, &mut egui_ctx, &player.atlas_index()));
    commands.insert_resource(EguiButtonTextures {
        textures,
        player_textures: textures_p,
    });
}
