use crate::dialogs::save_file_dialog::SaveFileDialog;
use crate::dialogs::unsaved_changes_dialog::UnsavedChangesDialog;
use crate::dialogs::DialogResource;
use crate::game::constants::MAPEDITOR_BUTTON_SIZE;
use crate::game::player::ReturnTo;
use crate::game::tilewrapper::MapWrapper;
use crate::mapeditor::player_spawn::{
    destroy_player_spawn, init_player_spawn, PlayerSpawnComponent,
};
use crate::mapeditor::{MapeditorSystems, SelectedTile};
use crate::textures::egui_textures::{atlas_to_egui_textures, EguiButtonTextures};
use crate::ui::uicontrols::WindowUiOverlayInfo;
use crate::ui::{check_ui_size_changed, image_button, UiSizeChangedEvent};
use crate::{AppLabels, AppState, GameDirectoriesWrapper};
use bevy::prelude::*;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Align, Layout, TextBuffer, Ui};
use bevy_egui::{egui, EguiContexts};
use libexodus::tiles::{TeleportId, Tile, UITiles};
use std::borrow::Borrow;
use strum::IntoEnumIterator;
pub struct MapEditorUiPlugin;

impl Plugin for MapEditorUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedTile>()
            .add_systems(
                OnEnter(AppState::MapEditor),
                init_player_spawn
                    .after(AppLabels::World)
                    .in_set(MapeditorSystems::PlayerSpawnPlaceholderInit),
            )
            .add_systems(OnExit(AppState::MapEditor), destroy_player_spawn)
            .add_systems(
                OnEnter(AppState::MapEditor),
                atlas_to_egui_textures.after(MapeditorSystems::PlayerSpawnPlaceholderInit),
            )
            .add_systems(
                Update,
                mapeditor_ui
                    .run_if(in_state(AppState::MapEditor))
                    .after(MapeditorSystems::GameBoardMouseHandlers)
                    .in_set(MapeditorSystems::UiDrawing),
            )
            .add_systems(
                Update,
                mapeditor_dialog.run_if(
                    in_state(AppState::MapEditorDialog).and(resource_exists::<DialogResource>),
                ),
            );
    }
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
                    .unwrap_or_else(|| panic!("Textures for {:?} were not loaded as Egui textures!", tile));
                ui.add_sized([MAPEDITOR_BUTTON_SIZE, MAPEDITOR_BUTTON_SIZE], egui::ImageButton::new(SizedTexture::new(*id, *size)).uv(*uv))
            } else if *tile == Tile::PLAYERSPAWN {
                    let (id, size, uv) = egui_textures.textures.get(&player.player.atlas_index())
                        .expect("The Player Texture was not found in the Egui textures!");
                    ui.add_sized([MAPEDITOR_BUTTON_SIZE, MAPEDITOR_BUTTON_SIZE], egui::ImageButton::new(SizedTexture::new(*id, *size)).uv(*uv))
            } else {
                ui.add_sized([MAPEDITOR_BUTTON_SIZE, MAPEDITOR_BUTTON_SIZE], egui::Button::new(""))
            }

                .on_hover_text(t!(format!("tile.{}",tile.str_id())))
                .on_disabled_hover_text(format!("{} ({})", t!(format!("tile.{}",tile.str_id())), t!("map_editor.buttons.currently_selected"))) // unfortunately there is no on_disabled_hover_text_at_pointer
            ;
        if button.clicked() {
            selected_tile.tile = tile.clone();
            ui.close_menu();
        }
    });
}

fn mapeditor_ui(
    mut commands: Commands,
    mut egui_ctx: EguiContexts,
    mut selected_tile: ResMut<SelectedTile>,
    egui_textures: Res<EguiButtonTextures>,
    player: Query<&PlayerSpawnComponent>,
    mut state: ResMut<NextState<AppState>>,
    mut worldwrapper: ResMut<MapWrapper>,
    current_window_size: ResMut<WindowUiOverlayInfo>,
    mut window_size_event_writer: EventWriter<UiSizeChangedEvent>,
    directories: Res<GameDirectoriesWrapper>,
    return_to: Res<ReturnTo>,
) {
    let player_it = player
        .iter()
        .next()
        .expect("There was no Player Spawn set up");
    let panel = egui::TopBottomPanel::top("")
        .resizable(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.scope(|ui| {
                    ui.horizontal(|ui| {
                        // Exit Button
                        ui.scope(|ui| {
                            ui.set_height(MAPEDITOR_BUTTON_SIZE);
                            ui.set_width(MAPEDITOR_BUTTON_SIZE);
                            ui.centered_and_justified(|ui| {
                                let xbutton = image_button(
                                    ui,
                                    &egui_textures,
                                    &UITiles::BACKBUTTON,
                                    "map_editor.dialog.exit_tooltip",
                                );
                                if xbutton.clicked() {
                                    if worldwrapper.world.is_dirty() {
                                        commands.insert_resource(DialogResource {
                                            ui_dialog: Box::new(UnsavedChangesDialog::new(
                                                t!("map_editor.dialog.unsaved_changes_dialog_text")
                                                    .as_str(),
                                            )),
                                        });
                                        state.set(AppState::MapEditorDialog);
                                    } else {
                                        state.set(return_to.0);
                                    }
                                }
                            });
                        });
                        // Save Button
                        ui.scope(|ui| {
                            ui.set_height(MAPEDITOR_BUTTON_SIZE);
                            ui.set_width(MAPEDITOR_BUTTON_SIZE);
                            ui.centered_and_justified(|ui| {
                                let sbutton = image_button(
                                    ui,
                                    &egui_textures,
                                    &UITiles::SAVEBUTTON,
                                    "map_editor.dialog.save_tooltip",
                                );
                                if sbutton.clicked() {
                                    worldwrapper.world.recompute_hash();
                                    commands.insert_resource(DialogResource {
                                        ui_dialog: Box::new(SaveFileDialog::new(
                                            worldwrapper.world.get_filename(),
                                            worldwrapper.world.get_name(),
                                            worldwrapper.world.get_author(),
                                            &worldwrapper.world.hash_str().as_str()[..16],
                                            &directories.game_directories,
                                            worldwrapper.world.forced_tileset(),
                                        )),
                                    });
                                    state.set(AppState::MapEditorDialog);
                                }
                            });
                        });
                        ui.separator();
                        // Buttons for the different tiles
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALL,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLCOBBLE,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLSMOOTH,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLNATURE,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLCHISELED,
                            &mut selected_tile,
                            player_it,
                        );
                        ui.separator();
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::SLOPE,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::PILLAR,
                            &mut selected_tile,
                            player_it,
                        );
                        ui.separator();
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::AIR,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::SPIKES,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::SPIKESALT,
                            &mut selected_tile,
                            player_it,
                        );
                        ui.separator();
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::COIN,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::KEY,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::STARCRYSTAL,
                            &mut selected_tile,
                            player_it,
                        );
                        ui.separator();
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::LADDER,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::LADDERSLOPE,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::LADDERNATURE,
                            &mut selected_tile,
                            player_it,
                        );
                        ui.separator();
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::PLAYERSPAWN,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::EXIT,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::DOOR,
                            &mut selected_tile,
                            player_it,
                        );
                        ui.separator();
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::COBBLEROOFSLOPEL,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::COBBLEROOFSLOPER,
                            &mut selected_tile,
                            player_it,
                        );
                        ui.separator();

                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::MESSAGE { message_id: 0 },
                            &mut selected_tile,
                            player_it,
                        );
                    });
                });
                ui.scope(|ui| {
                    ui.horizontal(|ui| {
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::SPIKESSLOPED,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLSPIKEST,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLSPIKESL,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLSPIKESR,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLSPIKESB,
                            &mut selected_tile,
                            player_it,
                        );
                        ui.separator();
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLSPIKESLR,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLSPIKESRB,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLSPIKESLB,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLSPIKESLT,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLSPIKESRT,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLSPIKESLTB,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLSPIKESTB,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLSPIKESRTB,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLSPIKESRLB,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLSPIKESRLT,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::WALLSPIKESRLTB,
                            &mut selected_tile,
                            player_it,
                        );
                        ui.separator();
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::ARROWLEFT,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::ARROWDOWN,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::ARROWRIGHT,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::ARROWUP,
                            &mut selected_tile,
                            player_it,
                        );
                        ui.separator();
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::VENDINGMACHINEL,
                            &mut selected_tile,
                            player_it,
                        );
                        tile_kind_selector_button_for(
                            ui,
                            egui_textures.borrow(),
                            &Tile::VENDINGMACHINER,
                            &mut selected_tile,
                            player_it,
                        );
                    })
                });
            });
        });
    let top = panel.response.rect.height();
    let left_panel = egui::SidePanel::left("side_panel")
        .resizable(false)
        .exact_width(MAPEDITOR_BUTTON_SIZE)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                for teleport_id in TeleportId::iter() {
                    tile_kind_selector_button_for(
                        ui,
                        egui_textures.borrow(),
                        &Tile::TELEPORTENTRY { teleport_id },
                        &mut selected_tile,
                        player_it,
                    );
                    tile_kind_selector_button_for(
                        ui,
                        egui_textures.borrow(),
                        &Tile::TELEPORTEXIT { teleport_id },
                        &mut selected_tile,
                        player_it,
                    );
                }
            });
        });
    let left = left_panel.response.rect.width();
    check_ui_size_changed(
        &WindowUiOverlayInfo {
            top,
            left,
            ..default()
        },
        current_window_size,
        &mut window_size_event_writer,
    );
}
/// Handle all possible kinds of dialogs that can occur in the Map Editor
fn mapeditor_dialog(
    mut egui_ctx: EguiContexts,
    egui_textures: Res<EguiButtonTextures>,
    mut dialog: ResMut<DialogResource>,
    mut state: ResMut<NextState<AppState>>,
    mut worldwrapper: ResMut<MapWrapper>,
    directories: Res<GameDirectoriesWrapper>,
    return_to: Res<ReturnTo>,
    mut commands: Commands,
) {
    egui::Window::new(dialog.ui_dialog.dialog_title())
        .resizable(false)
        .collapsible(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            dialog.ui_dialog.draw(
                ui,
                &egui_textures,
                &directories.game_directories,
                &mut commands,
            );
        });
    if dialog.ui_dialog.is_done() {
        if let Some(save_dialog) = dialog.ui_dialog.as_save_file_dialog() {
            if let Some(updated_filename) = save_dialog.get_filename() {
                worldwrapper.world.set_filename(updated_filename);
            }
            worldwrapper.world.set_name(save_dialog.get_map_title());
            worldwrapper.world.set_author(save_dialog.get_map_author());
            worldwrapper
                .world
                .set_forced_tileset(save_dialog.get_forced_tileset());
            if worldwrapper.world.get_filename().is_some() {
                let result = worldwrapper
                    .world
                    .save_to_file(worldwrapper.world.get_filename().unwrap());
                match result {
                    Ok(_) => {
                        // Mark map as clean, i.e. there are no unsaved changes
                        worldwrapper.world.set_clean();
                    },
                    Err(v) => {
                        error!(
                            "Could not save map file {} - {}",
                            worldwrapper
                                .world
                                .get_filename()
                                .unwrap()
                                .to_str()
                                .unwrap_or("<invalid>"),
                            v.to_string()
                        );
                        // Mark map as dirty, because saving the map was not successful
                        worldwrapper.world.set_dirty();
                    },
                }
            }
            state.set(AppState::MapEditor);
        } else if dialog.ui_dialog.as_unsaved_changes_dialog().is_some() {
            state.set(return_to.0);
        } else if let Some(edit_dialog) = dialog.ui_dialog.as_edit_message_dialog() {
            worldwrapper
                .world
                .set_message(
                    edit_dialog.get_message_id(),
                    edit_dialog.get_message().to_string(),
                )
                .map(|_| {
                    debug!(
                        "Successfully set message {} to {}",
                        edit_dialog.get_message_id(),
                        edit_dialog.get_message()
                    );
                })
                .unwrap_or_else(|e| {
                    error!(
                        "Could not set message {} to {}: {}!",
                        edit_dialog.get_message_id(),
                        edit_dialog.get_message(),
                        e
                    )
                });
            state.set(AppState::MapEditor);
        }
    } else if dialog.ui_dialog.is_cancelled() {
        state.set(AppState::MapEditor);
    }
}
