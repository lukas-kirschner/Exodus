use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::iter::Map;
use std::rc::Rc;
use std::sync::Arc;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy_egui::{egui, EguiContext};
use bevy_egui::egui::{TextureId, Ui, Widget};
use libexodus::tiles::{AtlasIndex, Tile};
use strum::{IntoEnumIterator};
use libexodus::player::Player;
use crate::{AppState, CurrentMapTextureAtlasHandle, CurrentPlayerTextureAtlasHandle};
use crate::game::constants::MAPEDITOR_BUTTON_SIZE;
use crate::game::tilewrapper::MapWrapper;
use crate::mapeditor::{compute_cursor_position_in_world, SelectedTile};
use crate::mapeditor::player_spawn::{destroy_player_spawn, init_player_spawn, PlayerSpawnComponent};
use crate::mapeditor::save_file_dialog::{SaveFileDialog, UIDialog};
use crate::uicontrols::{MAPEDITOR_CONTROLS_HEIGHT};


pub struct MapEditorUiPlugin;

impl Plugin for MapEditorUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectedTile>()
            .add_system_set(SystemSet::on_enter(AppState::MapEditor)
                .with_system(init_player_spawn).after("world").label("player_spawn_placeholder_init")
            )
            .add_system_set(SystemSet::on_exit(AppState::MapEditor)
                .with_system(destroy_player_spawn)
            )
            .add_system_set(SystemSet::on_enter(AppState::MapEditor)
                .with_system(atlas_to_egui_textures).after("player_spawn_placeholder_init")
            )
            .add_system_set(SystemSet::on_update(AppState::MapEditor)
                .with_system(mapeditor_ui)
            )
            .add_system_set(SystemSet::on_update(AppState::MapEditorDialog)
                .with_system(mapeditor_dialog)
            )
        ;
    }
}


pub struct EguiButtonTextures {
    textures: HashMap<AtlasIndex, (TextureId, egui::Vec2, egui::Rect)>,
    player_textures: HashMap<AtlasIndex, (TextureId, egui::Vec2, egui::Rect)>,
}

impl FromWorld for EguiButtonTextures {
    fn from_world(_: &mut World) -> Self {
        EguiButtonTextures {
            textures: HashMap::new(),
            player_textures: HashMap::new(),
        }
    }
}

struct MapEditorDialogResource {
    ui_dialog: Box<dyn UIDialog + Send + Sync>,
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
fn atlas_to_egui_textures(
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

fn tile_kind_selector_button_for(
    ui: &mut Ui,
    egui_textures: &EguiButtonTextures,
    tile: &Tile,
    mut selected_tile: &mut ResMut<SelectedTile>,
    player: &PlayerSpawnComponent,
) {
    ui.add_enabled_ui(selected_tile.tile != *tile, |ui| {
        let button =
            if let Some(atlas_index) = tile.atlas_index() {
                let (id, size, uv) = egui_textures.textures.get(&atlas_index)
                    .expect(format!("Textures for {:?} were not loaded as Egui textures!", tile).as_str())
                    ;
                ui.add_sized([MAPEDITOR_BUTTON_SIZE, MAPEDITOR_BUTTON_SIZE], egui::ImageButton::new(*id, *size).uv(*uv))
            } else {
                if *tile == Tile::PLAYERSPAWN {
                    let (id, size, uv) = egui_textures.player_textures.get(&player.player.atlas_index())
                        .expect("The Player Texture was not found in the Egui textures!");
                    ui.add_sized([MAPEDITOR_BUTTON_SIZE, MAPEDITOR_BUTTON_SIZE], egui::ImageButton::new(*id, *size).uv(*uv))
                } else {
                    ui.add_sized([MAPEDITOR_BUTTON_SIZE, MAPEDITOR_BUTTON_SIZE], egui::Button::new(""))
                }
            }
                .on_hover_text(tile.to_string())
                .on_disabled_hover_text(format!("{} (currently selected)", tile)) // unfortunately there is no on_disabled_hover_text_at_pointer
            ;
        if button.clicked() {
            selected_tile.tile = tile.clone();
        }
    });
}

fn mapeditor_ui(
    mut commands: Commands,
    mut egui_ctx: ResMut<EguiContext>,
    mut selected_tile: ResMut<SelectedTile>,
    egui_textures: Res<EguiButtonTextures>,
    player: Query<&PlayerSpawnComponent>,
    mut state: ResMut<State<AppState>>,
    mut worldwrapper: ResMut<MapWrapper>,
) {
    let player_it = player.iter().next().expect("There was no Player Spawn set up");
    egui::TopBottomPanel::top("")
        .resizable(false)
        .default_height(MAPEDITOR_CONTROLS_HEIGHT)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                // Save Button
                ui.scope(|ui| {
                    ui.set_height(MAPEDITOR_BUTTON_SIZE);
                    ui.set_width(MAPEDITOR_BUTTON_SIZE);
                    ui.centered_and_justified(|ui| {
                        let sbutton = ui.button("S").on_hover_text("Save Map or Set Map Properties");
                        if sbutton.clicked() {
                            commands.insert_resource(MapEditorDialogResource {
                                ui_dialog: Box::new(SaveFileDialog::new(&worldwrapper.world)),
                            });
                            state.set(AppState::MapEditorDialog).expect("Could not open save dialog!");
                        }
                    });
                });
                ui.separator();
                // Buttons for the different tiles
                tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::AIR, &mut selected_tile, player_it);
                tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALL, &mut selected_tile, player_it);
                tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::SPIKES, &mut selected_tile, player_it);
                tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::SPIKESALT, &mut selected_tile, player_it);
                ui.separator();
                tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::COIN, &mut selected_tile, player_it);
                tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::LADDER, &mut selected_tile, player_it);
                ui.separator();
                tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::PLAYERSPAWN, &mut selected_tile, player_it);
                ui.scope(|ui| {
                    ui.set_height(MAPEDITOR_BUTTON_SIZE);
                    ui.set_width(MAPEDITOR_BUTTON_SIZE);
                    ui.centered_and_justified(|ui| {
                        ui.menu_button("w", |ui| { // TODO Icon on a Menu Button?
                            tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::SPIKESSLOPED, &mut selected_tile, player_it);
                            tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESL, &mut selected_tile, player_it);
                            tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESR, &mut selected_tile, player_it);
                            tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESB, &mut selected_tile, player_it);
                        }).response.on_hover_text("Select a wall spike edge");
                    });
                });
                ui.scope(|ui| {
                    ui.set_height(MAPEDITOR_BUTTON_SIZE);
                    ui.set_width(MAPEDITOR_BUTTON_SIZE);
                    ui.centered_and_justified(|ui| {
                        ui.menu_button("W", |ui| {
                            tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESLR, &mut selected_tile, player_it);
                            tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESRB, &mut selected_tile, player_it);
                            tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESLB, &mut selected_tile, player_it);
                            tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESLTB, &mut selected_tile, player_it);
                            tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESRTB, &mut selected_tile, player_it);
                            tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESRLTB, &mut selected_tile, player_it);
                        }).response.on_hover_text("Select a wall spike corner");
                    });
                });
            });
        });
}

fn mapeditor_dialog(mut egui_ctx: ResMut<EguiContext>,
                    egui_textures: Res<EguiButtonTextures>,
                    mut dialog: ResMut<MapEditorDialogResource>,
                    mut state: ResMut<State<AppState>>,
                    mut commands: Commands,
) {
    egui::Window::new(dialog.ui_dialog.dialog_title())
        .resizable(false)
        .collapsible(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            dialog.ui_dialog.draw(ui, &*egui_textures);
        });
    if dialog.ui_dialog.is_done() {
        state.set(AppState::MapEditor);
    }
}