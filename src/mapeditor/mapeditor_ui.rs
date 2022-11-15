use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::iter::Map;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_egui::egui::{TextureId, Widget};
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
    textures: HashMap<AtlasIndex, egui::Image>,
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
            textures.insert(atlas_index, egui::Image::new(tex, rect_vec2).uv(uv));
        }
    }
    commands.insert_resource(EguiButtonTextures {
        textures,
    });
}

fn mapeditor_ui(
    mut egui_ctx: ResMut<EguiContext>,
    mut selected_tile: ResMut<SelectedTile>,
    egui_textures: Res<EguiButtonTextures>,
) {
    let mut selected: Tile = Tile::WALL;
    egui::TopBottomPanel::bottom("")
        .resizable(false)
        .default_height(MAPEDITOR_CONTROLS_HEIGHT)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                let btn_spikes_state = egui_textures.textures.get(&Tile::WALL.atlas_index().expect("Wall needs a sprite")).unwrap().ui(ui);
                egui::ComboBox::from_label("Spikes")
                    .selected_text(format!("{:?}", selected))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut selected, Tile::SPIKES, Tile::SPIKES.to_string());
                    });
            });
        });
    selected_tile.tile = selected.clone();
}