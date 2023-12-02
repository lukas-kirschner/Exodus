use crate::TilesetManager;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_egui::egui::{Pos2, TextureId};
use bevy_egui::{egui, EguiContexts};
use libexodus::tiles::AtlasIndex;
use std::collections::HashMap;

/// The size in pixels of all square EGUI textures
const EGUI_TEX_SIZE: usize = 32;

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
/// Scale the given texture using Nearest Neighbor Interpolation
/// to match the TEXTURE_SIZE and create a new image.
///
/// # Returns
/// a tuple of the new handle to the scaled image and the texture size in pixels.
/// The resulting image is always square
fn scale_texture(
    uv: &Rect,
    assets: &mut Assets<Image>,
    texture_handle: &Handle<Image>,
) -> (Handle<Image>, usize) {
    let source_image = assets.get(texture_handle).unwrap();
    assert_eq!(
        uv.max.x - uv.min.x,
        uv.max.y - uv.min.y,
        "Expected square textures!"
    );
    assert_eq!(
        (uv.max.x - uv.min.x) * 16.,
        source_image.width() as f32,
        "Expected a source image width of {}, got {}",
        (uv.max.x - uv.min.x) * 16.,
        source_image.width()
    );
    debug_assert_eq!(
        (uv.max.x - uv.min.x).fract(),
        0.0,
        "Expected texture uv sizes to be an integer!"
    );
    let old_texture_size = (uv.max.x - uv.min.x) as usize;
    let ratio = old_texture_size as f64 / EGUI_TEX_SIZE as f64;
    let rgba_image = source_image
        .clone()
        .try_into_dynamic()
        .unwrap()
        .into_rgba8();
    let data = rgba_image.as_raw();
    let (data_w, data_h) = (
        source_image.size().x as usize * 4,
        source_image.size().y as usize,
    );
    assert_eq!(data_w * data_h, data.len());
    assert_eq!(
        source_image.texture_descriptor.format,
        TextureFormat::Rgba8UnormSrgb,
        "Image format {:?} expected! Got {:?} instead",
        TextureFormat::Rgba8UnormSrgb,
        source_image.texture_descriptor.format
    );
    let mut target_arr = Vec::with_capacity(EGUI_TEX_SIZE * EGUI_TEX_SIZE * 4);
    for y in 0..EGUI_TEX_SIZE {
        let py = (y as f64 * ratio).floor() as usize;
        let y_img = py + uv.min.y as usize;
        assert!(
            y_img < source_image.size().y as usize,
            "Y Index {} ({}) out of bounds! Height: {}",
            y_img,
            py,
            source_image.size().y
        );

        for x in 0..EGUI_TEX_SIZE {
            let px = (x as f64 * ratio).floor() as usize;
            let x_img = px + uv.min.x as usize;
            assert!(
                x_img < source_image.size().x as usize,
                "X Index {} on source UV ({},{})->({},{}) ({} * {} = {}) out of bounds! Width: {}",
                x_img,
                uv.min.x,
                uv.min.y,
                uv.max.x,
                uv.max.y,
                x,
                ratio,
                px,
                source_image.size().x
            );

            for offset in 0..4usize {
                let pixel = data[(x_img * 4) + offset + (data_w * y_img)];
                target_arr.push(pixel);
            }
        }
    }
    (
        assets.add(Image::new(
            Extent3d {
                width: EGUI_TEX_SIZE as u32,
                height: EGUI_TEX_SIZE as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            target_arr,
            source_image.texture_descriptor.format,
        )),
        EGUI_TEX_SIZE,
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
    let (handle, size) = scale_texture(&rect, assets, texture_handle);
    let uv: egui::Rect = egui::Rect::from_min_max(Pos2::new(0., 0.), Pos2::new(1., 1.));
    let rect_vec2: egui::Vec2 = egui::Vec2::new(size as f32, size as f32);
    let tex: TextureId = egui_ctx.add_image(handle);
    (tex, rect_vec2, uv)
}

/// Convert Bevy Textures to Egui Textures to show those on the buttons
pub fn atlas_to_egui_textures(
    tileset_manager: Res<TilesetManager>,
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut egui_ctx: EguiContexts,
    mut assets: ResMut<Assets<Image>>,
) {
    let texture_atlas: &TextureAtlas = texture_atlases
        .get(&tileset_manager.current_handle())
        .expect(
            format!(
                "The texture atlas of the tile set {} has not yet been loaded!",
                tileset_manager.current_tileset
            )
            .as_str(),
        );
    assert_eq!(
        texture_atlas.size.x / 16.,
        tileset_manager.current_tileset.texture_size() as f32
    );
    let texture_handle: &Handle<Image> = &texture_atlas.texture;
    let mut textures = HashMap::new();
    // Convert all available textures from the sprite sheet
    for atlas_index in 0..256 {
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
    commands.insert_resource(EguiButtonTextures { textures });
}
