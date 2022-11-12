use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::iter::Map;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_egui::egui::Widget;
use libexodus::tiles::{AtlasIndex, Tile};
use libexodus::tiles::Tile::WALL;
use strum::{IntoEnumIterator};
use crate::AppState;
use crate::game::tilewrapper::MapWrapper;
use crate::uicontrols::{MAPEDITOR_CONTROLS_HEIGHT};


pub struct MapEditorUiPlugin;

impl Plugin for MapEditorUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectedTile>()
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
    texture_atlas: Image,
    textures: HashMap<AtlasIndex, egui::Rect>,
}

impl FromWorld for EguiButtonTextures {
    fn from_world(world: &mut World) -> Self {
        todo!()
    }
}

/// Convert Bevy Textures to Egui Textures to show those on the buttons
// fn atlas_to_egui_textures(
//     texture_atlas_handle: Res<MapTextureAtlasHandle>,
//     mut commands: Commands,
//     texture_atlases: Res<Assets<TextureAtlas>>,
//     images: Res<Assets<Image>>,
// ) {
//     let texture_atlas = texture_atlases.get(&texture_atlas_handle.handle).expect("The atlas has not yet been loaded!");
//     let texture_handle = &texture_atlas.texture;
//     let mut textures = HashMap::new();
//     for tile in Tile::iter() {
//         if let Some(atlas_index) = tile.atlas_index() {
//             let rect = texture_atlas.textures[atlas_index];
//             let uv: egui::Rect = egui::Rect::from_min_max(
//                 egui::pos2(rect.min.x / texture_atlas.size.x, rect.min.y / texture_atlas.size.y),
//                 egui::pos2(rect.max.x / texture_atlas.size.x, rect.max.y / texture_atlas.size.y),
//             );
//             textures.insert(atlas_index, uv);
//         }
//     }
//     commands.insert_resource(EguiButtonTextures {
//         texture_atlas: images.get(texture_handle).cloned().expect("The texture has not yet been loaded!"),
//         textures,
//     });
// }

fn mapeditor_ui(
    mut egui_ctx: ResMut<EguiContext>,
    mut selected_tile: ResMut<SelectedTile>,
) {
    let mut selected: Tile = WALL;
    egui::TopBottomPanel::bottom("")
        .resizable(false)
        .default_height(MAPEDITOR_CONTROLS_HEIGHT)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                // let btn_air_state = egui::ImageButton::new(texture, Vec2::new(32.0, 32.0)).ui(ui);
                egui::ComboBox::from_label("Spikes")
                    .selected_text(format!("{:?}", selected))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut selected, Tile::SPIKES, Tile::SPIKES.to_string());
                    });
            });
        });
    selected_tile.tile = selected.clone();
}