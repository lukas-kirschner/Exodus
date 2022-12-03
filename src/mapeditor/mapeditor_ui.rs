use std::borrow::Borrow;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_egui::egui::Ui;
use libexodus::tiles::{Tile};
use crate::{AppState, GameDirectoriesWrapper};
use crate::dialogs::save_file_dialog::SaveFileDialog;
use crate::dialogs::UIDialog;
use crate::dialogs::unsaved_changes_dialog::UnsavedChangesDialog;
use crate::egui_textures::{atlas_to_egui_textures, EguiButtonTextures};
use crate::game::constants::MAPEDITOR_BUTTON_SIZE;
use crate::game::tilewrapper::MapWrapper;
use crate::mapeditor::{MapeditorSystems, SelectedTile};
use crate::mapeditor::player_spawn::{destroy_player_spawn, init_player_spawn, PlayerSpawnComponent};
use crate::uicontrols::{MAPEDITOR_CONTROLS_HEIGHT, NAVBAR_BACK_TEXT};

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
                .with_system(mapeditor_ui.label(MapeditorSystems::UiDrawing).after(MapeditorSystems::GameBoardMouseHandlers))
            )
            .add_system_set(SystemSet::on_update(AppState::MapEditorDialog)
                .with_system(mapeditor_dialog)
            )
        ;
    }
}

#[derive(Resource)]
struct MapEditorDialogResource {
    ui_dialog: Box<dyn UIDialog + Send + Sync>,
}

/// Create an egui button to select a tile that can currently be placed
fn tile_kind_selector_button_for(
    ui: &mut Ui,
    egui_textures: &EguiButtonTextures,
    tile: &Tile,
    selected_tile: &mut ResMut<SelectedTile>,
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
            ui.close_menu();
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
    worldwrapper: ResMut<MapWrapper>,
) {
    let player_it = player.iter().next().expect("There was no Player Spawn set up");
    let _panel = egui::TopBottomPanel::top("")
        .resizable(false)
        .default_height(MAPEDITOR_CONTROLS_HEIGHT)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.scope(|ui| {
                    ui.horizontal(|ui| {
                        // Exit Button
                        ui.scope(|ui| {
                            ui.set_height(MAPEDITOR_BUTTON_SIZE);
                            ui.set_width(MAPEDITOR_BUTTON_SIZE);
                            ui.centered_and_justified(|ui| {
                                let xbutton = ui.button(NAVBAR_BACK_TEXT).on_hover_text("Exit and return to Map Selection Screen");
                                if xbutton.clicked() {
                                    if worldwrapper.world.is_dirty() {
                                        commands.insert_resource(MapEditorDialogResource {
                                            ui_dialog: Box::new(UnsavedChangesDialog::new(
                                                "The current map has unsaved changes! Do you really want to quit and discard the changes?"
                                            )),
                                        });
                                        state.set(AppState::MapEditorDialog).expect("Could not change state to overwrite dialog!");
                                    } else {
                                        state.set(AppState::MapSelectionScreen).expect("Could not change state back to map selection screen!");
                                    }
                                }
                            });
                        });
                        // Save Button
                        ui.scope(|ui| {
                            ui.set_height(MAPEDITOR_BUTTON_SIZE);
                            ui.set_width(MAPEDITOR_BUTTON_SIZE);
                            ui.centered_and_justified(|ui| {
                                let sbutton = ui.button("S").on_hover_text("Save Map or Set Map Properties");
                                if sbutton.clicked() {
                                    commands.insert_resource(MapEditorDialogResource {
                                        ui_dialog: Box::new(SaveFileDialog::new(
                                            worldwrapper.world.get_filename(),
                                            worldwrapper.world.get_name(),
                                            worldwrapper.world.get_author(),
                                            worldwrapper.world.uuid().as_str(),
                                        )),
                                    });
                                    state.set(AppState::MapEditorDialog).expect("Could not change state to save dialog!");
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
                        tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::KEY, &mut selected_tile, player_it);
                        tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::LADDER, &mut selected_tile, player_it);
                        ui.separator();
                        tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::PLAYERSPAWN, &mut selected_tile, player_it);
                        tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::DOOR, &mut selected_tile, player_it);
                        ui.separator();
                        tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::ARROWLEFT, &mut selected_tile, player_it);
                        tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::ARROWDOWN, &mut selected_tile, player_it);
                        tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::ARROWRIGHT, &mut selected_tile, player_it);
                        tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::ARROWUP, &mut selected_tile, player_it);
                    });
                });
                ui.scope(|ui| {
                    ui.horizontal(|ui| {
                        tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::SPIKESSLOPED, &mut selected_tile, player_it);
                        tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESL, &mut selected_tile, player_it);
                        tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESR, &mut selected_tile, player_it);
                        tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESB, &mut selected_tile, player_it);
                        ui.separator();
                        tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESLR, &mut selected_tile, player_it);
                        tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESRB, &mut selected_tile, player_it);
                        tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESLB, &mut selected_tile, player_it);
                        tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESLTB, &mut selected_tile, player_it);
                        tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESRTB, &mut selected_tile, player_it);
                        tile_kind_selector_button_for(ui, egui_textures.borrow(), &Tile::WALLSPIKESRLTB, &mut selected_tile, player_it);
                    })
                });
            });
        })
        ;
    // let ui_height = panel.response.rect.height();
    // println!("Height {:?}", ui_height);
}

fn mapeditor_dialog(mut egui_ctx: ResMut<EguiContext>,
                    egui_textures: Res<EguiButtonTextures>,
                    mut dialog: ResMut<MapEditorDialogResource>,
                    mut state: ResMut<State<AppState>>,
                    mut worldwrapper: ResMut<MapWrapper>,
                    directories: Res<GameDirectoriesWrapper>,
) {
    egui::Window::new(dialog.ui_dialog.dialog_title())
        .resizable(false)
        .collapsible(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            dialog.ui_dialog.draw(ui, &*egui_textures, &directories.game_directories);
        });
    if dialog.ui_dialog.is_done() {
        if let Some(save_dialog) = dialog.ui_dialog.as_save_file_dialog() {
            if let Some(upd_fname) = save_dialog.get_filename() {
                worldwrapper.world.set_filename(upd_fname);
            }
            worldwrapper.world.set_name(save_dialog.get_map_title());
            worldwrapper.world.set_author(save_dialog.get_map_author());
            if worldwrapper.world.get_filename().is_some() {
                let result = worldwrapper.world.save_to_file(worldwrapper.world.get_filename().unwrap());
                match result {
                    Ok(_) => {
                        // Mark map as clean, i.e. there are no unsaved changes
                        worldwrapper.world.set_clean();
                    }
                    Err(v) => {
                        eprintln!("Could not save map file {} - {}", worldwrapper.world.get_filename().unwrap().to_str().unwrap_or("<invalid>"), v.to_string());
                        // Mark map as dirty, because saving the map was not successful
                        worldwrapper.world.set_dirty();
                    }
                }
            }
            state.set(AppState::MapEditor).expect("Could not close dialog");
        } else if let Some(_) = dialog.ui_dialog.as_unsaved_changes_dialog() {
            state.set(AppState::MapSelectionScreen).expect("Could not exit Map Editor");
        }
    } else if dialog.ui_dialog.is_cancelled() {
        state.set(AppState::MapEditor).expect("Could not cancel dialog");
    }
}