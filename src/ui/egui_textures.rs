use crate::TilesetManager;
use bevy::prelude::*;
use bevy_egui::egui::FontFamily::Proportional;
use bevy_egui::egui::{FontId, TextureId};
use bevy_egui::{egui, EguiContext};
use libexodus::player::Player;
use libexodus::tiles::{AtlasIndex, Tile};
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Resource)]
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
    mut egui_ctx: ResMut<EguiContext>,
) {
    let texture_atlas: &TextureAtlas = texture_atlases
        .get(&texture_atlas_handle.current_handle())
        .expect("The texture atlas of the tile set has not yet been loaded!");
    let texture_handle: &Handle<Image> = &texture_atlas.texture;
    let mut textures = HashMap::new();
    for tile in Tile::iter() {
        if let Some(atlas_index) = tile.atlas_index() {
            textures.insert(
                atlas_index,
                convert(texture_atlas, texture_handle, &mut egui_ctx, &atlas_index),
            );
        }
    }
    let mut textures_p = HashMap::new();
    // The Player Spawn needs a special atlas index:
    let player = Player::new(); // TODO The Query is not working in this stage, unfortunately
    let texture_atlas: &TextureAtlas = texture_atlases
        .get(&texture_atlas_handle.current_handle())
        .expect("The texture atlas of the player set has not yet been loaded!");
    let texture_handle: &Handle<Image> = &texture_atlas.texture;
    textures_p.insert(
        player.atlas_index(),
        convert(
            texture_atlas,
            texture_handle,
            &mut egui_ctx,
            &player.atlas_index(),
        ),
    );
    commands.insert_resource(EguiButtonTextures {
        textures,
        player_textures: textures_p,
    });
}

pub fn egui_fonts(ctx: &egui::Context) -> () {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "exodus".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/fonts/PublicPixel.ttf")),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "exodus".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("exodus".to_owned());
    ctx.set_fonts(fonts);

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (egui::TextStyle::Heading, FontId::new(30.0, Proportional)),
        (
            egui::TextStyle::Name("Heading2".into()),
            FontId::new(25.0, Proportional),
        ),
        (
            egui::TextStyle::Name("Context".into()),
            FontId::new(23.0, Proportional),
        ),
        (egui::TextStyle::Body, FontId::new(18.0, Proportional)),
        (egui::TextStyle::Monospace, FontId::new(16.0, Proportional)),
        (egui::TextStyle::Button, FontId::new(20.0, Proportional)),
        (egui::TextStyle::Small, FontId::new(10.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}
