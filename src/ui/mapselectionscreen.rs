use crate::dialogs::create_new_map_dialog::{
    bevy_job_handler, CreateMapBackgroundWorkerThread, CreateNewMapDialog,
};
use crate::dialogs::delete_map_dialog::DeleteMapDialog;
use crate::dialogs::DialogResource;
use crate::game::camera::{destroy_camera, handle_ui_resize, setup_camera};
use crate::game::player::ReturnTo;
use crate::game::scoreboard::{egui_highscore_label, Scoreboard};
use crate::game::tilewrapper::MapWrapper;
use crate::game::world::destroy_world;
use crate::game::HighscoresDatabaseWrapper;
use crate::textures::egui_textures::EguiButtonTextures;
use crate::ui::uicontrols::{add_navbar_with_extra_buttons, menu_esc_control, WindowUiOverlayInfo};
use crate::ui::{check_ui_size_changed, image_button, UiSizeChangedEvent, BUTTON_HEIGHT, UIMARGIN};
use crate::{AppLabels, AppState, GameConfig, GameDirectoriesWrapper};
use bevy::prelude::*;
use bevy_egui::egui::{emath, Align, Color32, Id, LayerId, Layout, Order, Pos2, Stroke, Ui};
use bevy_egui::{egui, EguiContexts};
use libexodus::highscores::highscores_database::HighscoresDatabase;
use libexodus::tiles::UITiles;
use libexodus::world::{presets, GameWorld};
use std::ops::Sub;

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
        example_map.set_name(t!("debug.map_presets.example_world").as_ref());
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
        showcasemap.set_name(t!("debug.map_presets.showcase").as_ref());
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
        psion_sized_map.set_name(t!("debug.map_presets.empty5mx").as_ref());
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
                .as_ref(),
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
            commands.insert_resource(DialogResource {
                ui_dialog: Box::new(DeleteMapDialog::new(maps.maps.remove(map_index))),
            });
            state.set(AppState::MapSelectionScreenDialog);
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
    add_navbar_with_extra_buttons(
        egui_ctx.ctx_mut(),
        &mut state,
        &egui_textures,
        &t!("map_selection_screen.title"),
        |ui, state| {
            let new_button = image_button(
                ui,
                &egui_textures,
                &UITiles::CREATENEWBUTTON,
                "map_selection_screen.create_new_map",
            );
            if new_button.clicked() {
                commands.insert_resource(DialogResource {
                    ui_dialog: Box::<CreateNewMapDialog>::default(),
                });
                state.set(AppState::MapSelectionScreenDialog);
            }
        },
        1,
    );
    let spacing = (
        egui_ctx.ctx_mut().style().spacing.item_spacing.x,
        egui_ctx.ctx_mut().style().spacing.item_spacing.y,
    );
    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        egui::ScrollArea::new([false, true])
            .auto_shrink([false; 2])
            .max_width(ui.available_width())
            .show(ui, |ui| {
                ui.style_mut().visuals.faint_bg_color = Color32::from_gray(100);
                ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                    egui::Grid::new("maps_grid")
                        .striped(true)
                        .num_columns(1)
                        .show(ui, |ui| {
                            for (i, map) in maps.maps.iter().enumerate() {
                                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                                    let height = BUTTON_HEIGHT * 1.8;
                                    let width = ui.available_width();
                                    // ui.with_layer_id(LayerId::background(), |ui| {
                                    let next_pos: Pos2 = ui
                                        .next_widget_position()
                                        .sub(Pos2::from((width, 0.0)))
                                        .to_pos2();
                                    let rect = egui::Rect::from_two_pos(
                                        next_pos,
                                        (next_pos.x + width, next_pos.y + height).into(),
                                    )
                                    .expand2(ui.style().spacing.item_spacing);
                                    ui.painter().rect_filled(
                                        rect,
                                        0.,
                                        if (i % 2) == 0 {
                                            ui.style().visuals.widgets.noninteractive.bg_fill
                                        } else {
                                            ui.style().visuals.faint_bg_color
                                        },
                                    );
                                    // });
                                    ui.set_height(height);
                                    ui.set_width(width);
                                    // ui.style_mut().visuals.widgets.noninteractive.bg_fill =;
                                    ui.add_space(ui.spacing().item_spacing.x);
                                    buttons(spacing, ui, &egui_textures, &mut commands, i);
                                    ui.scope(|ui| {
                                        ui.set_max_size(ui.available_size());
                                        ui.with_layout(egui::Layout::top_down(Align::LEFT), |ui| {
                                            labels_name_author(ui, &map.world);
                                            ui.add_space(UIMARGIN);
                                            egui_highscore_label(
                                                ui,
                                                &map.previous_best,
                                                &egui_textures,
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
    spacing: (f32, f32),
    ui: &mut Ui,
    egui_textures: &EguiButtonTextures,
    commands: &mut Commands,
    map_index: usize,
) {
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.set_width(3. * (BUTTON_HEIGHT + spacing.0));
        ui.set_height(BUTTON_HEIGHT + 2. * spacing.1);
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
}

pub fn labels_name_author(ui: &mut Ui, world: &GameWorld) {
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
/// Handle all possible kinds of dialogs that can occur in the Map Selection Screen
fn map_selection_screen_dialog(
    mut egui_ctx: EguiContexts,
    egui_textures: Res<EguiButtonTextures>,
    mut dialog: ResMut<DialogResource>,
    mut state: ResMut<NextState<AppState>>,
    directories: Res<GameDirectoriesWrapper>,
    mut commands: Commands,
    current_size: ResMut<WindowUiOverlayInfo>,
    mut event_writer: EventWriter<UiSizeChangedEvent>,
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
    check_ui_size_changed(
        &WindowUiOverlayInfo {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: 0.0,
        },
        current_size,
        &mut event_writer,
    );
    if dialog.ui_dialog.is_done() {
        if let Some(create_map_dialog) = dialog.ui_dialog.as_create_new_map_dialog() {
            if let Some(world) = create_map_dialog.generate_map(&mut commands) {
                commands.insert_resource(MapWrapper {
                    world,
                    previous_best: None,
                });
                commands.insert_resource(ReturnTo(AppState::MapSelectionScreen));
                state.set(AppState::MapEditor);
            }
        } else if let Some(delete_map_dialog) = dialog.ui_dialog.as_delete_map_dialog() {
            // Delete the map by moving it to system trash
            if let Some(path) = delete_map_dialog.map().world.get_filename() {
                trash::delete(path)
                    .map(|_| {
                        info!(
                            "Successfully moved map {} to system trash bin.",
                            path.display()
                        )
                    })
                    .unwrap_or_else(|e| error!("Could not delete map! {}", e));
            } else {
                error!(
                    "Tried to delete a map that did not have a path! \
                Hint: In Debug Mode, some of the maps do not have a path and \
                are re-generated on map selection screen launch. Thus, \
                debug maps cannot be deleted!"
                );
            }
            state.set(AppState::MapSelectionScreen);
        }
    } else if dialog.ui_dialog.is_cancelled() {
        state.set(AppState::MapSelectionScreen);
    }
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
            )
            // Map Selection Screen Dialog Logic:
            .add_systems(
                Update,
                map_selection_screen_dialog.run_if(
                    in_state(AppState::MapSelectionScreenDialog)
                        .and_then(resource_exists::<DialogResource>),
                ),
            )
            .add_systems(
                Update,
                bevy_job_handler.run_if(
                    in_state(AppState::MapSelectionScreenDialog)
                        .and_then(resource_exists::<DialogResource>)
                        .and_then(resource_exists::<CreateMapBackgroundWorkerThread>),
                ),
            )
            .add_systems(OnEnter(AppState::MapSelectionScreenDialog), setup_camera.after(AppLabels::World).in_set(AppLabels::Camera))
            .add_systems(Update, handle_ui_resize.run_if(in_state(AppState::MapSelectionScreenDialog)))
            .add_systems(OnExit(AppState::MapSelectionScreenDialog), destroy_camera)
            .add_systems(OnExit(AppState::MapSelectionScreenDialog), destroy_world);
    }
}
