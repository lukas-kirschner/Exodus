use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::iter::Map;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_egui::egui::{TextureId, Ui, Widget};
use libexodus::tiles::{AtlasIndex, Tile};
use strum::{IntoEnumIterator};
use crate::{AppState, CurrentMapTextureAtlasHandle};
use crate::game::tilewrapper::MapWrapper;
use crate::uicontrols::{MAPEDITOR_CONTROLS_HEIGHT};


pub struct MapEditorUiPlugin;

impl Plugin for MapEditorUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectedTile>()
            .add_system_set(SystemSet::on_enter(AppState::MapEditor)
                .with_system(atlas_to_egui_textures)
            )
            .add_system_set(SystemSet::on_update(AppState::MapEditor)
                .with_system(mapeditor_ui)
            )
        ;
    }
}

struct SelectedTile {
    pub tile: Tile,
}

impl FromWorld for SelectedTile {
    fn from_world(_: &mut World) -> Self {
        SelectedTile { tile: Tile::AIR }
    }
}

struct EguiButtonTextures {
    textures: HashMap<AtlasIndex, (TextureId, egui::Vec2, egui::Rect)>,
}

impl FromWorld for EguiButtonTextures {
    fn from_world(_: &mut World) -> Self {
        EguiButtonTextures {
            textures: HashMap::new()
        }
    }
}

/// Convert Bevy Textures to Egui Textures to show those on the buttons
fn atlas_to_egui_textures(
    texture_atlas_handle: Res<CurrentMapTextureAtlasHandle>,
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    images: Res<Assets<Image>>,
    mut egui_ctx: ResMut<EguiContext>,
) {
    let texture_atlas: &TextureAtlas = texture_atlases.get(&texture_atlas_handle.handle).expect("The texture atlas of the tile set has not yet been loaded!");
    let texture_handle: &Handle<Image> = &texture_atlas.texture;
    let mut textures = HashMap::new();
    // let img: &Image = images.get(texture_handle)
    //     .expect("The texture atlas image has not been found!");
    for tile in Tile::iter() {
        if let Some(atlas_index) = tile.atlas_index() {
            let rect: bevy::sprite::Rect = texture_atlas.textures[atlas_index];
            let uv: egui::Rect = egui::Rect::from_min_max(
                egui::pos2(rect.min.x / texture_atlas.size.x, rect.min.y / texture_atlas.size.y),
                egui::pos2(rect.max.x / texture_atlas.size.x, rect.max.y / texture_atlas.size.y),
            );
            let rect_vec2: egui::Vec2 = egui::Vec2::new(rect.max.x - rect.min.x, rect.max.y - rect.min.y);
            // Convert bevy::prelude::Image to bevy_egui::egui::TextureId?
            let tex: TextureId = egui_ctx.add_image(texture_handle.clone_weak());
            textures.insert(atlas_index, (tex, rect_vec2, uv));
        }
    }
    commands.insert_resource(EguiButtonTextures {
        textures,
    });
}

fn tile_kind_selector_button_for(
    ui: &mut Ui,
    egui_textures: &EguiButtonTextures,
    tile: &Tile,
    mut selected_tile: &mut ResMut<SelectedTile>,
) {
    let button =
        if let Some(atlas_index) = tile.atlas_index() {
            let (id, size, uv) = egui_textures.textures.get(&atlas_index)
                .expect(format!("Textures for {:?} were not loaded as Egui textures!", tile).as_str())
                ;
            ui.add_enabled(selected_tile.tile != *tile, egui::ImageButton::new(*id, *size).uv(*uv))
        } else {
            ui.add_enabled(selected_tile.tile != *tile, egui::Button::new(""))
        }
            .on_hover_text(tile.to_string())
        ;
    if button.clicked() {
        selected_tile.tile = tile.clone();
    }
}

fn mapeditor_ui(
    mut egui_ctx: ResMut<EguiContext>,
    mut selected_tile: ResMut<SelectedTile>,
    egui_textures: Res<EguiButtonTextures>,
) {
    egui::TopBottomPanel::bottom("")
        .resizable(false)
        .default_height(MAPEDITOR_CONTROLS_HEIGHT)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::AIR, &mut selected_tile);
                tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALL, &mut selected_tile);
                tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::SPIKES, &mut selected_tile);
                ui.separator();
                tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::COIN, &mut selected_tile);
                tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESLR, &mut selected_tile);
            });
        });
}