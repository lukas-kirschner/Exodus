use crate::game::tilewrapper::MapWrapper;
use crate::ui::uicontrols::{add_navbar, menu_esc_control};
use crate::ui::{BUTTON_HEIGHT, DELETE_TEXT, EDIT_TEXT, PLAY_TEXT};
use crate::{AppState, GameDirectoriesWrapper};
use bevy::prelude::*;
use bevy_egui::egui::Align;
use bevy_egui::{egui, EguiContext};
use libexodus::world::{presets, GameWorld};

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
fn load_maps(mut maps: ResMut<Maps>, directories: Res<GameDirectoriesWrapper>) {
    // Delete all maps
    maps.maps = Vec::new();

    // Load all maps from the game's map directory and all subdirectories
    directories
        .game_directories
        .iter_maps()
        .for_each(|map_file| {
            if let Ok(map) = GameWorld::load_from_file(map_file.path())
                .map_err(|err| {
                    error!(
                        "Could not load map file at {}! Error: {}",
                        map_file.path().to_str().unwrap_or("<Invalid Path>"),
                        err
                    )
                })
                .map(|mut map| {
                    debug!(
                        "Successfully loaded map file {}",
                        map_file.path().to_str().unwrap_or("<Invalid Path>")
                    );
                    map.set_clean();
                    map.recompute_hash();
                    map
                })
            {
                maps.maps.push(MapWrapper { world: map })
            }
        });

    //If we are in debug mode, insert the debug maps
    if cfg!(debug_assertions) {
        let mut example_map = GameWorld::exampleworld();
        example_map.set_name(t!("debug.map_presets.example_world").as_str());
        example_map.recompute_hash();
        maps.maps.push(MapWrapper { world: example_map });
        let mut showcasemap = GameWorld::showcaseworld();
        showcasemap.set_name(t!("debug.map_presets.showcase").as_str());
        showcasemap.recompute_hash();
        maps.maps.push(MapWrapper { world: showcasemap });
        let mut psion_sized_map = presets::map_with_border(35, 15);
        psion_sized_map.set_name(t!("debug.map_presets.empty5mx").as_str());
        psion_sized_map.recompute_hash();
        maps.maps.push(MapWrapper {
            world: psion_sized_map,
        });
        // Fill the list to test scrolling
        for i in 1..=20 {
            let mut map = presets::map_with_border(24 + i, i + 3);
            map.set_name(
                t!(
                    "debug.map_presets.empty_sized",
                    h = (i + 3).to_string().as_str(),
                    w = (24 + i).to_string().as_str()
                )
                .as_str(),
            );
            map.recompute_hash();
            maps.maps.push(MapWrapper { world: map })
        }
    }
}

#[derive(Resource)]
enum MapSelectionScreenAction {
    Play { map_index: usize },
    Delete { map_index: usize },
    Edit { map_index: usize },
    None,
}

impl FromWorld for MapSelectionScreenAction {
    fn from_world(_: &mut World) -> Self {
        MapSelectionScreenAction::None
    }
}

fn map_selection_screen_execute_event_queue(
    mut commands: Commands,
    action: Res<MapSelectionScreenAction>,
    mut maps: ResMut<Maps>,
    mut state: ResMut<State<AppState>>,
) {
    match *action {
        MapSelectionScreenAction::Play { map_index } => {
            let mapwrapper = maps.maps.remove(map_index);
            commands.insert_resource(mapwrapper);
            state.set(AppState::Playing).expect("Could not start game");
            commands.insert_resource(MapSelectionScreenAction::None)
        },
        MapSelectionScreenAction::Delete { map_index } => {
            //TODO Delete Map
            //TODO there is no need for locking here? Avoid deleting more maps than necessary
            maps.maps.remove(map_index);
            commands.insert_resource(MapSelectionScreenAction::None)
        },
        MapSelectionScreenAction::Edit { map_index } => {
            let mapwrapper = maps.maps.remove(map_index);
            commands.insert_resource(mapwrapper);
            state
                .set(AppState::MapEditor)
                .expect("Could not start map editor");
            commands.insert_resource(MapSelectionScreenAction::None)
        },
        MapSelectionScreenAction::None => {},
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
                                ui.set_height(BUTTON_HEIGHT);
                                ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                                    ui.label(map.world.get_name());
                                    ui.label(" ");
                                    ui.label(map.world.get_author());
                                    ui.with_layout(
                                        egui::Layout::right_to_left(Align::Center),
                                        |ui| {
                                            ui.scope(|ui| {
                                                ui.set_height(BUTTON_HEIGHT);
                                                ui.set_width(BUTTON_HEIGHT);
                                                ui.centered_and_justified(|ui| {
                                                    let play_btn =
                                                        ui.button(PLAY_TEXT).on_hover_text(t!(
                                                            "map_selection_screen.play_map"
                                                        ));
                                                    if play_btn.clicked() {
                                                        commands.insert_resource(
                                                            MapSelectionScreenAction::Play {
                                                                map_index: i,
                                                            },
                                                        );
                                                    }
                                                })
                                            });
                                            ui.scope(|ui| {
                                                ui.set_height(BUTTON_HEIGHT);
                                                ui.set_width(BUTTON_HEIGHT);
                                                ui.centered_and_justified(|ui| {
                                                    let edit_btn =
                                                        ui.button(EDIT_TEXT).on_hover_text(t!(
                                                            "map_selection_screen.edit_map"
                                                        ));
                                                    if edit_btn.clicked() {
                                                        commands.insert_resource(
                                                            MapSelectionScreenAction::Edit {
                                                                map_index: i,
                                                            },
                                                        );
                                                    }
                                                })
                                            });
                                            ui.scope(|ui| {
                                                ui.set_height(BUTTON_HEIGHT);
                                                ui.set_width(BUTTON_HEIGHT);
                                                ui.centered_and_justified(|ui| {
                                                    let delete_btn =
                                                        ui.button(DELETE_TEXT).on_hover_text(t!(
                                                            "map_selection_screen.delete_map"
                                                        ));
                                                    if delete_btn.clicked() {
                                                        commands.insert_resource(
                                                            MapSelectionScreenAction::Delete {
                                                                map_index: i,
                                                            },
                                                        );
                                                    }
                                                })
                                            });
                                        },
                                    );
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
        app.init_resource::<Maps>()
            .init_resource::<MapSelectionScreenAction>()
            .add_system_set(
                SystemSet::on_enter(AppState::MapSelectionScreen)
                    .with_system(load_maps)
                    .label("load_maps"),
            )
            .add_system_set(
                SystemSet::on_update(AppState::MapSelectionScreen)
                    .with_system(map_selection_screen_ui),
            )
            .add_system_set(
                SystemSet::on_update(AppState::MapSelectionScreen)
                    .with_system(map_selection_screen_execute_event_queue),
            )
            .add_system_set(
                SystemSet::on_update(AppState::MapSelectionScreen).with_system(menu_esc_control),
            );
    }
}
