use crate::game::player::ReturnTo;
use crate::game::scoreboard::{egui_highscore_label, Scoreboard};
use crate::game::tilewrapper::MapWrapper;
use crate::game::HighscoresDatabaseWrapper;
use crate::textures::egui_textures::EguiButtonTextures;
use crate::ui::uicontrols::{add_navbar, menu_esc_control};
use crate::ui::{image_button, BUTTON_HEIGHT, UIMARGIN};
use crate::{AppLabels, AppState, GameConfig, GameDirectoriesWrapper};
use bevy::prelude::*;
use bevy_egui::egui::{Align, Layout, Ui};
use bevy_egui::{egui, EguiContexts};
use libexodus::highscores::highscores_database::HighscoresDatabase;
use libexodus::tiles::UITiles;
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

fn get_highscore(
    highscores: &HighscoresDatabase,
    map: &GameWorld,
    player_name: &String,
) -> Option<Scoreboard> {
    highscores
        .get_best(map.hash(), player_name)
        .map(|highscore| highscore.1.into())
}

/// Load all maps from the Map Directory. This might take a while, depending on how many maps there are in the maps folder
fn load_maps(
    mut maps: ResMut<Maps>,
    directories: Res<GameDirectoriesWrapper>,
    highscores: Res<HighscoresDatabaseWrapper>,
    config: Res<GameConfig>,
) {
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
                let highscore =
                    get_highscore(&highscores.highscores, &map, &config.config.player_id);
                maps.maps.push(MapWrapper {
                    world: map,
                    previous_best: highscore,
                })
            }
        });

    //If we are in debug mode, insert the debug maps
    if cfg!(debug_assertions) {
        let mut example_map = GameWorld::exampleworld();
        example_map.set_name(t!("debug.map_presets.example_world").as_str());
        example_map.recompute_hash();
        let example_map_highscore = get_highscore(
            &highscores.highscores,
            &example_map,
            &config.config.player_id,
        );
        maps.maps.push(MapWrapper {
            world: example_map,
            previous_best: example_map_highscore,
        });
        let mut showcasemap = GameWorld::showcaseworld();
        showcasemap.set_name(t!("debug.map_presets.showcase").as_str());
        showcasemap.recompute_hash();
        let showcasemap_highscore = get_highscore(
            &highscores.highscores,
            &showcasemap,
            &config.config.player_id,
        );
        maps.maps.push(MapWrapper {
            world: showcasemap,
            previous_best: showcasemap_highscore,
        });
        let mut psion_sized_map = presets::map_with_border(35, 15);
        psion_sized_map.set_name(t!("debug.map_presets.empty5mx").as_str());
        psion_sized_map.recompute_hash();
        maps.maps.push(MapWrapper {
            world: psion_sized_map,
            previous_best: None,
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
            maps.maps.push(MapWrapper {
                world: map,
                previous_best: None,
            })
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
    mut state: ResMut<NextState<AppState>>,
) {
    match *action {
        MapSelectionScreenAction::Play { map_index } => {
            let mapwrapper = maps.maps.remove(map_index);
            commands.insert_resource(mapwrapper);
            commands.insert_resource(ReturnTo(AppState::MapSelectionScreen));
            state.set(AppState::Playing);
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
            commands.insert_resource(ReturnTo(AppState::MapSelectionScreen));
            state.set(AppState::MapEditor);
            commands.insert_resource(MapSelectionScreenAction::None)
        },
        MapSelectionScreenAction::None => {},
    }
}

/// Map Selection Screen main routine
fn map_selection_screen_ui(
    mut commands: Commands,
    mut egui_ctx: EguiContexts,
    mut state: ResMut<NextState<AppState>>,
    egui_textures: Res<EguiButtonTextures>,
    maps: Res<Maps>,
) {
    add_navbar(
        egui_ctx.ctx_mut(),
        &mut state,
        &egui_textures,
        &t!("map_selection_screen.title"),
    );

    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        egui::ScrollArea::new([false, true])
            .auto_shrink([false; 2])
            .max_width(ui.available_width())
            .show(ui, |ui| {
                ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                    egui::Grid::new("maps_grid")
                        .striped(true)
                        .num_columns(1)
                        .show(ui, |ui| {
                            for (i, map) in maps.maps.iter().enumerate() {
                                ui.scope(|ui| {
                                    ui.set_height(BUTTON_HEIGHT * 1.);
                                    ui.set_width(ui.available_width());
                                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                                        buttons(ui, &egui_textures, &mut commands, i);
                                        ui.scope(|ui| {
                                            ui.style_mut().spacing.item_spacing = (0.0, 0.0).into();
                                            ui.style_mut().spacing.indent = 0.0;
                                            ui.set_max_size(ui.available_size());
                                            ui.with_layout(
                                                egui::Layout::top_down(Align::LEFT),
                                                |ui| {
                                                    labels_name_author(ui, &map.world);
                                                    ui.add_space(UIMARGIN);
                                                    egui_highscore_label(
                                                        ui,
                                                        &map.previous_best,
                                                        &egui_textures,
                                                    );
                                                },
                                            );
                                        });
                                    });
                                });
                                ui.end_row();
                            }
                        });
                });
            });
    });
}

fn buttons(
    ui: &mut Ui,
    egui_textures: &EguiButtonTextures,
    commands: &mut Commands,
    map_index: usize,
) {
    ui.scope(|ui| {
        ui.set_width(3. * BUTTON_HEIGHT);
        ui.set_height(BUTTON_HEIGHT);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let play_btn = image_button(
                ui,
                egui_textures,
                &UITiles::PLAYBUTTON,
                "map_selection_screen.play_map",
            );
            if play_btn.clicked() {
                commands.insert_resource(MapSelectionScreenAction::Play { map_index });
            }
            let edit_btn = image_button(
                ui,
                egui_textures,
                &UITiles::EDITBUTTON,
                "map_selection_screen.edit_map",
            );
            if edit_btn.clicked() {
                commands.insert_resource(MapSelectionScreenAction::Edit { map_index });
            }
            let delete_btn = image_button(
                ui,
                egui_textures,
                &UITiles::DELETEBUTTON,
                "map_selection_screen.delete_map",
            );
            if delete_btn.clicked() {
                commands.insert_resource(MapSelectionScreenAction::Delete { map_index });
            }
        });
    });
}

fn labels_name_author(ui: &mut Ui, world: &GameWorld) {
    ui.with_layout(egui::Layout::left_to_right(Align::TOP), |ui| {
        ui.label(
            egui::RichText::new(world.get_name())
                .text_style(egui::TextStyle::Name("MapTitle".into())),
        );
        ui.add_space(UIMARGIN);
        ui.vertical(|ui| {
            ui.add_space(4.);
            ui.label(
                egui::RichText::new(world.get_author())
                    .text_style(egui::TextStyle::Name("MapAuthor".into())),
            );
        });
    });
}

pub struct MapSelectionScreenPlugin;

impl Plugin for MapSelectionScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Maps>()
            .init_resource::<MapSelectionScreenAction>()
            .add_systems(
                OnEnter(AppState::MapSelectionScreen),
                load_maps.in_set(AppLabels::LoadMaps),
            )
            .add_systems(
                Update,
                map_selection_screen_ui.run_if(in_state(AppState::MapSelectionScreen)),
            )
            .add_systems(
                Update,
                map_selection_screen_execute_event_queue
                    .run_if(in_state(AppState::MapSelectionScreen)),
            )
            .add_systems(
                Update,
                menu_esc_control.run_if(in_state(AppState::MapSelectionScreen)),
            );
    }
}
