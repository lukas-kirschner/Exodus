use crate::TilesetManager;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_egui::egui::{Pos2, TextureId};
use bevy_egui::{egui, EguiContexts};
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

fn scale_texture(
    uv: &Rect,
    assets: &mut Assets<Image>,
    texture_handle: &Handle<Image>,
) -> (Handle<Image>, usize) {
    const TEXTURE_SIZE: usize = 32;
    let image = assets.get(texture_handle).unwrap();
    let scale: f64 = (uv.max.x - uv.min.x) as f64 / TEXTURE_SIZE as f64;
    assert_eq!(
        scale,
        (uv.max.y - uv.min.y) as f64 / TEXTURE_SIZE as f64,
        "Expected square textures!"
    );
    let rgba_image = image.clone().try_into_dynamic().unwrap().into_rgba8();
    let data = rgba_image.as_raw();
    assert_eq!(
        image.texture_descriptor.format,
        TextureFormat::Rgba8UnormSrgb,
        "Image format {:?} expected! Got {:?} instead",
        TextureFormat::Rgba8UnormSrgb,
        image.texture_descriptor.format
    );
    let mut target_arr = Vec::with_capacity(TEXTURE_SIZE * TEXTURE_SIZE * 4);
    for y in 0..TEXTURE_SIZE {
        for arr_x in (0..TEXTURE_SIZE * 4).step_by(4) {
            assert_eq!(arr_x % 4, 0);
            let x = arr_x / 4;
            let x_img = (x + uv.min.x as usize) as f64;
            let y_img = (y + uv.min.y as usize) as f64;
            let x_nearest: i32 = (((x_img + 0.5) * scale).floor() as i32) * 4;
            let y_nearest: i32 = ((y_img + 0.5) * scale).floor() as i32;
            println!("From ({},{})", x_nearest, y_nearest);
            for offset in 0..4 {
                assert_eq!(
                    (x_nearest / 4) * 4,
                    x_nearest,
                    "x_nearest was not divisible by 4!"
                );
                let pixel = data[(x_nearest + (x_nearest * y_nearest) + offset) as usize];
                // target_arr[x + (x * y) + offset] = pixel;
                target_arr.push(pixel);
            }
            println!(
                "Set ({},{}) to ({},{},{},{})",
                arr_x,
                y,
                target_arr[arr_x + (arr_x * y)],
                target_arr[arr_x + (arr_x * y) + 1],
                target_arr[arr_x + (arr_x * y) + 2],
                target_arr[arr_x + (arr_x * y) + 3]
            );
        }
    }
    (
        assets.add(Image::new(
            Extent3d {
                width: TEXTURE_SIZE as u32,
                height: TEXTURE_SIZE as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            target_arr,
            image.texture_descriptor.format,
        )),
        TEXTURE_SIZE,
    )
}

fn convert(
    texture_atlas: &TextureAtlas,
    texture_handle: &Handle<Image>,
    egui_ctx: &mut EguiContexts,
    atlas_index: &AtlasIndex,
    assets: &mut Assets<Image>,
) -> (TextureId, egui::Vec2, egui::Rect) {
    let rect: Rect = texture_atlas.textures[*atlas_index];
    let (mut handle, size) = scale_texture(&rect, assets, texture_handle);
    let uv: egui::Rect = egui::Rect::from_min_max(Pos2::new(0., 0.), Pos2::new(1., 1.));
    let rect_vec2: egui::Vec2 = egui::Vec2::new(size as f32, size as f32);
    // Convert bevy::prelude::Image to bevy_egui::egui::TextureId?
    handle.make_strong(assets);
    let tex: TextureId = egui_ctx.add_image(handle);
    (tex, rect_vec2, uv)
}

/// Convert Bevy Textures to Egui Textures to show those on the buttons
pub fn atlas_to_egui_textures(
    texture_atlas_handle: Res<TilesetManager>,
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut egui_ctx: EguiContexts,
    mut assets: ResMut<Assets<Image>>,
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
                convert(
                    texture_atlas,
                    texture_handle,
                    &mut egui_ctx,
                    &atlas_index,
                    &mut assets,
                ),
            );
        }
    }
    // Convert Button Textures
    for extratexture in UITiles::iter() {
        if let Some(atlas_index) = extratexture.atlas_index() {
            textures.insert(
                atlas_index,
                convert(
                    texture_atlas,
                    texture_handle,
                    &mut egui_ctx,
                    &atlas_index,
                    &mut assets,
                ),
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
            &mut assets,
        ),
    );

    commands.insert_resource(EguiButtonTextures { textures });
}
