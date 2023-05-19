use crate::game::constants::FONT_SIZE_HIGHSCORE;
use crate::game::scoreboard::Scoreboard;
use crate::game::tilewrapper::MapWrapper;
use crate::game::HighscoresDatabaseWrapper;
use crate::textures::egui_textures::EguiButtonTextures;
use crate::ui::uicontrols::{add_navbar, menu_esc_control};
use crate::ui::{image_button, BUTTON_HEIGHT, UIMARGIN};
use crate::{AppState, GameConfig, GameDirectoriesWrapper};
use bevy::prelude::*;
use bevy_egui::egui::{Align, Layout, RichText, Ui};
use bevy_egui::{egui, EguiContext};
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
    egui_textures: Res<EguiButtonTextures>,
    maps: Res<Maps>,
) {
    add_navbar(&mut egui_ctx, &mut state, &egui_textures);

    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        egui::ScrollArea::new([false, true])
            .auto_shrink([false; 2])
            .max_width(ui.available_width())
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    for (i, map) in maps.maps.iter().enumerate() {
                        ui.scope(|ui| {
                            ui.set_height(BUTTON_HEIGHT * 2.);
                            ui.set_width(ui.available_width());
                            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                                    buttons(ui, &egui_textures, &mut commands, i);
                                });
                                ui.with_layout(egui::Layout::top_down(Align::Min), |ui| {
                                    ui.set_max_size(ui.available_size());
                                    ui.with_layout(egui::Layout::left_to_right(Align::Min), |ui| {
                                        labels_row1(ui, &map.world);
                                    });
                                    ui.scope(|ui| ui.set_height(UIMARGIN));
                                    ui.with_layout(egui::Layout::left_to_right(Align::Min), |ui| {
                                        labels_row2(ui, &map.previous_best);
                                    });
                                });
                            });
                        });
                    }
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
}

fn labels_row1(ui: &mut Ui, world: &GameWorld) {
    ui.label(world.get_name());
    ui.label(" ");
    ui.label(world.get_author());
}

fn labels_row2(ui: &mut Ui, scoreboard: &Option<Scoreboard>) {
    match scoreboard {
        None => {
            ui.label(
                RichText::new(t!("map_selection_screen.no_highscore")).size(FONT_SIZE_HIGHSCORE),
            );
        },
        Some(score) => {
            ui.label(
                RichText::new(t!("map_selection_screen.highscore_heading"))
                    .size(FONT_SIZE_HIGHSCORE),
            );
            ui.label(RichText::new(" ").size(FONT_SIZE_HIGHSCORE));
            ui.label(
                RichText::new(t!(
                    "map_selection_screen.moves_fmt",
                    moves = &score.moves.to_string()
                ))
                .size(14.),
            );
            ui.label(RichText::new(" ").size(FONT_SIZE_HIGHSCORE));
            ui.label(
                RichText::new(t!(
                    "map_selection_screen.coins_fmt",
                    coins = &score.coins.to_string()
                ))
                .size(FONT_SIZE_HIGHSCORE),
            );
        },
    }
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
