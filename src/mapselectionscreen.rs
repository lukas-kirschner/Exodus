use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_egui::egui::Align;
use libexodus::world::{GameWorld, presets};
use crate::{AppState, GameDirectoriesWrapper};
use crate::uicontrols::{add_navbar, DELETE_TEXT, EDIT_TEXT, menu_esc_control, NAVBAR_HEIGHT, PLAY_TEXT};
use crate::game::tilewrapper::MapWrapper;

#[derive(Resource)]
struct Maps {
    maps: Vec<MapWrapper>,
}

impl FromWorld for Maps {
    fn from_world(world: &mut World) -> Self {
        Maps {
            maps: vec![MapWrapper::from_world(world)],
        }
    }
}

/// Load all maps from the Map Directory. This might take a while, depending on how many maps there are in the maps folder
fn load_maps(
    mut maps: ResMut<Maps>,
    directories: Res<GameDirectoriesWrapper>,
) {
    // Delete all maps
    maps.maps = Vec::new();

    // Load all maps from the game's map directory and all subdirectories
    directories.game_directories.iter_maps()
        .for_each(|map_file| {
            if let Ok(map) = GameWorld::load_from_file(map_file.path())
                .map_err(|err| eprintln!("Could not load map file at {}! Error: {}", map_file.path().to_str().unwrap_or("<Invalid Path>"), err))
                .map(|mut map| {
                    println!("Successfully loaded map file {}", map_file.path().to_str().unwrap_or("<Invalid Path>"));
                    map.set_clean();
                    map
                }) {
                maps.maps.push(MapWrapper {
                    world: map,
                })
            }
        });

    //If we are in debug mode, insert the debug map exampleworld()!
    if cfg!(debug_assertions) {
        maps.maps.push(MapWrapper {
            world: GameWorld::exampleworld(),
        });
        // Fill the list to test scrolling
        for i in 1..=20 {
            let mut map = presets::map_with_border(24 + i, i + 3);
            map.set_name(format!("Empty {}x{} world", 24 + i, i + 3).as_str());
            maps.maps.push(MapWrapper {
                world: map,
            })
        }
    }
}

#[derive(Resource)]
enum MapSelectionScreenAction {
    PLAY { map_index: usize },
    DELETE { map_index: usize },
    EDIT { map_index: usize },
    NONE,
}

impl FromWorld for MapSelectionScreenAction {
    fn from_world(_: &mut World) -> Self {
        MapSelectionScreenAction::NONE
    }
}

fn map_selection_screen_execute_event_queue(
    mut commands: Commands,
    action: Res<MapSelectionScreenAction>,
    mut maps: ResMut<Maps>,
    mut state: ResMut<State<AppState>>,
) {
    match *action {
        MapSelectionScreenAction::PLAY { map_index } => {
            let mapwrapper = maps.maps.remove(map_index);
            commands.insert_resource(mapwrapper);
            state.set(AppState::Playing)
                .expect("Could not start game");
            commands.insert_resource(MapSelectionScreenAction::NONE)
        }
        MapSelectionScreenAction::DELETE { map_index } => {
            //TODO Delete Map
            //TODO there is no need for locking here? Avoid deleting more maps than necessary
            maps.maps.remove(map_index);
            commands.insert_resource(MapSelectionScreenAction::NONE)
        }
        MapSelectionScreenAction::EDIT { map_index } => {
            let mapwrapper = maps.maps.remove(map_index);
            commands.insert_resource(mapwrapper);
            state.set(AppState::MapEditor)
                .expect("Could not start map editor");
            commands.insert_resource(MapSelectionScreenAction::NONE)
        }
        MapSelectionScreenAction::NONE => {}
    }
}

/// Map Selection Screen main routine
fn map_selection_screen_ui(
    mut commands: Commands,
    mut egui_ctx: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    maps: Res<Maps>,
) {
    add_navbar(&mut egui_ctx, &mut state);

    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.centered_and_justified(|ui| {
            ui.scope(|ui| {
                egui::ScrollArea::new([false, true]).show(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        for (i, map) in maps.maps.iter().enumerate() {
                            ui.scope(|ui| {
                                ui.set_height(NAVBAR_HEIGHT);
                                ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                                    ui.label(map.world.get_name());
                                    ui.label(" ");
                                    ui.label(map.world.get_author());
                                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                                        ui.scope(|ui| {
                                            ui.set_height(NAVBAR_HEIGHT);
                                            ui.set_width(NAVBAR_HEIGHT);
                                            ui.centered_and_justified(|ui| {
                                                let play_btn = ui.button(PLAY_TEXT).on_hover_text("Play Map");
                                                if play_btn.clicked() {
                                                    commands.insert_resource(MapSelectionScreenAction::PLAY { map_index: i });
                                                }
                                            })
                                        });
                                        ui.scope(|ui| {
                                            ui.set_height(NAVBAR_HEIGHT);
                                            ui.set_width(NAVBAR_HEIGHT);
                                            ui.centered_and_justified(|ui| {
                                                let edit_btn = ui.button(EDIT_TEXT).on_hover_text("Edit Map");
                                                if edit_btn.clicked() {
                                                    commands.insert_resource(MapSelectionScreenAction::EDIT { map_index: i });
                                                }
                                            })
                                        });
                                        ui.scope(|ui| {
                                            ui.set_height(NAVBAR_HEIGHT);
                                            ui.set_width(NAVBAR_HEIGHT);
                                            ui.centered_and_justified(|ui| {
                                                let delete_btn = ui.button(DELETE_TEXT).on_hover_text("Delete Map");
                                                if delete_btn.clicked() {
                                                    commands.insert_resource(MapSelectionScreenAction::DELETE { map_index: i });
                                                }
                                            })
                                        });
                                    });
                                });
                            });
                        }
                    });
                });
            });
        });
    });
}

pub struct MapSelectionScreenPlugin;

impl Plugin for MapSelectionScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Maps>()
            .init_resource::<MapSelectionScreenAction>()
            .add_system_set(
                SystemSet::on_enter(AppState::MapSelectionScreen)
                    .with_system(load_maps).label("load_maps"),
            )
            .add_system_set(SystemSet::on_update(AppState::MapSelectionScreen)
                .with_system(map_selection_screen_ui)
            )
            .add_system_set(SystemSet::on_update(AppState::MapSelectionScreen)
                .with_system(map_selection_screen_execute_event_queue)
            )
            .add_system_set(SystemSet::on_update(AppState::MapSelectionScreen)
                .with_system(menu_esc_control)
            )
        ;
    }
}