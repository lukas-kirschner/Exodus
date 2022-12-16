use std::collections::HashMap;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy_egui::{egui, EguiContext};
use bevy_egui::egui::FontFamily::Proportional;
use bevy_egui::egui::{FontId, Pos2, TextureId};
use libexodus::player::Player;
use libexodus::tiles::{AtlasIndex, Tile};
use crate::{TilesetManager};
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

fn scale_texture(
    uv: &Rect,
    assets: &mut Assets<Image>,
    texture_handle: &Handle<Image>,
) -> (Handle<Image>, usize) {
    const target_size: usize = 32;
    let image = assets.get(texture_handle).unwrap();
    let scale: f64 = (uv.max.x as f64 - uv.min.x as f64) / target_size as f64;
    assert_eq!(scale, (uv.max.y as f64 - uv.min.y as f64) / target_size as f64, "Expected square textures!");
    let rgba_image = image.clone().try_into_dynamic().unwrap().into_rgba8();
    let data = rgba_image.as_raw();
    assert!(image.texture_descriptor.format == TextureFormat::Rgba8UnormSrgb,
            "Image format {:?} expected! Got {:?} instead",
            TextureFormat::Rgba8UnormSrgb,
            image.texture_descriptor.format);
    let mut target_arr = Vec::with_capacity(target_size * target_size * 4);
    for y in 0..target_size {
        for x in (0..target_size * 4).step_by(4) {
            let real_x = x / 4;
            let x_nearest: i32 = ((((real_x + uv.min.x as usize) as f64 + 0.5) * scale).floor() as i32) * 4;
            let y_nearest: i32 = (((y + uv.min.y as usize) as f64 + 0.5) * scale).floor() as i32;
            println!("From ({},{})", x_nearest, y_nearest);
            for offset in 0..4 {
                assert_eq!((x_nearest / 4) * 4, x_nearest, "x_nearest was not divisible by 4!");
                let pixel = data[(x_nearest + (x_nearest * y_nearest) + offset as i32) as usize];
                // target_arr[x + (x * y) + offset] = pixel;
                target_arr.push(pixel);
            }
            println!("Set ({},{}) to ({},{},{},{})", x, y, target_arr[x + (x * y) + 0], target_arr[x + (x * y) + 1], target_arr[x + (x * y) + 2], target_arr[x + (x * y) + 3]);
        }
    }
    (assets.add(Image::new(
        Extent3d {
            width: target_size as u32,
            height: target_size as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        target_arr,
        image.texture_descriptor.format.clone())), target_size)
}

fn convert(
    texture_atlas: &TextureAtlas,
    texture_handle: &Handle<Image>,
    egui_ctx: &mut ResMut<EguiContext>,
    atlas_index: &AtlasIndex,
    assets: &mut Assets<Image>,
) -> (TextureId, egui::Vec2, egui::Rect) {
    let rect: Rect = texture_atlas.textures[*atlas_index];
    let (mut handle, size) = scale_texture(&rect, assets, texture_handle);
    let uv: egui::Rect = egui::Rect::from_min_max(Pos2::new(0., 0.), Pos2::new(1., 1.));
    let rect_vec2: egui::Vec2 = egui::Vec2::new(size as f32, size as f32);
    // Convert bevy::prelude::Image to bevy_egui::egui::TextureId?
    handle.make_strong(assets);
    let tex: TextureId = egui_ctx.add_image(handle); // Move the handle, binding the lifetime to egui
    (tex, rect_vec2, uv)
}

/// Convert Bevy Textures to Egui Textures to show those on the buttons
pub fn atlas_to_egui_textures(
    texture_atlas_handle: Res<TilesetManager>,
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut assets: ResMut<Assets<Image>>,
) {
    let texture_atlas: &TextureAtlas = texture_atlases.get(&texture_atlas_handle.current_handle()).expect("The texture atlas of the tile set has not yet been loaded!");
    let texture_handle: &Handle<Image> = &texture_atlas.texture;
    let mut textures = HashMap::new();
    for tile in Tile::iter() {
        if let Some(atlas_index) = tile.atlas_index() {
            textures.insert(atlas_index, convert(texture_atlas, texture_handle, &mut egui_ctx, &atlas_index, &mut *assets));
        }
    }//TODO unify
    let mut textures_p = HashMap::new();
    // The Player Spawn needs a special atlas index:
    let player = Player::new();
    textures_p.insert(player.atlas_index(), convert(texture_atlas, texture_handle, &mut egui_ctx, &player.atlas_index(), &mut *assets));
    commands.insert_resource(EguiButtonTextures {
        textures,
        player_textures: textures_p,
    });
}

pub fn egui_fonts(ctx: &egui::Context) -> () {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert("exodus".to_owned(),
                           egui::FontData::from_static(include_bytes!("../../assets/fonts/PublicPixel.ttf")));
    fonts.families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "exodus".to_owned());
    fonts.families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("exodus".to_owned());
    ctx.set_fonts(fonts);

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (egui::TextStyle::Heading, FontId::new(30.0, Proportional)),
        (egui::TextStyle::Name("Heading2".into()), FontId::new(25.0, Proportional)),
        (egui::TextStyle::Name("Context".into()), FontId::new(23.0, Proportional)),
        (egui::TextStyle::Body, FontId::new(18.0, Proportional)),
        (egui::TextStyle::Monospace, FontId::new(16.0, Proportional)),
        (egui::TextStyle::Button, FontId::new(20.0, Proportional)),
        (egui::TextStyle::Small, FontId::new(10.0, Proportional)),
    ]
        .into();
    ctx.set_style(style);
}