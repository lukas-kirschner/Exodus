use crate::game::tilewrapper::{GameOverState, MapWrapper};
use crate::game::HighscoresDatabaseWrapper;
use crate::ui::egui_textures::EguiButtonTextures;
use crate::ui::{image_button, UIBIGMARGIN, UIMARGIN, UIPANELWIDTH};
use crate::{AppState, GameConfig};
use bevy::prelude::*;
use bevy_egui::egui::Frame;
use bevy_egui::{egui, EguiContext};
use libexodus::highscores::highscore::Highscore;
use libexodus::highscores::io_error::HighscoreParseError;
use libexodus::tiles::UITiles;

pub struct GameOverScreen;

/// Game Over Screen Drawing Routine
fn game_over_screen_ui(
    mut egui_ctx: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    egui_textures: Res<EguiButtonTextures>,
    // map: Res<MapWrapper>,
    game_status: Res<GameOverState>,
    // mut highscore_database: ResMut<HighscoresDatabaseWrapper>,
    config: Res<GameConfig>,
    mut save_state: ResMut<SaveHighscoreState>,
) {
    egui::CentralPanel::default()
        .frame(Frame::none())
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                ui.vertical_centered(|ui| {
                    ui.scope(|ui| {
                        ui.set_width(UIPANELWIDTH);
                        ui.vertical_centered_justified(|ui| {
                            ui.set_width(UIPANELWIDTH - UIBIGMARGIN);
                            ui.scope(|ui| {
                                ui.set_height(UIBIGMARGIN);
                            });
                            ui.scope(|ui| {
                                ui.heading(format!(
                                    "{}!",
                                    match &*game_status {
                                        GameOverState::Lost =>
                                            t!("game_over_screen.game_over_heading"),
                                        GameOverState::Won { .. } =>
                                            t!("game_over_screen.victory_heading"),
                                    }
                                ));
                            });
                            ui.separator();
                            ui.scope(|ui| {
                                ui.set_height(UIMARGIN);
                            });
                            ui.scope(|ui| match &*game_status {
                                //TODO nicer UI
                                GameOverState::Lost => {
                                    ui.label("You lost");
                                },
                                GameOverState::Won { score } => {
                                    ui.label(format!(
                                        "Coins: {} Moves: {}",
                                        score.coins, score.moves
                                    ));
                                    //TODO Table with previous best/Map Name/Coins,Moves/Player Name
                                },
                            });
                            ui.separator();
                            ui.scope(|ui| {
                                ui.set_height(UIMARGIN);
                            });
                            ui.scope(|ui| {
                                ui.label(match &*game_status {
                                    GameOverState::Lost => {
                                        t!("game_over_screen.highscore_info.lost")
                                    },
                                    GameOverState::Won { .. } => match &*save_state {
                                        SaveHighscoreState::SAVE => t!(
                                            "game_over_screen.highscore_info.won",
                                            player = &config.config.player_id
                                        ),
                                        SaveHighscoreState::NOSAVE => {
                                            t!("game_over_screen.highscore_info.won_discard")
                                        },
                                    },
                                });
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(
                                        match &*save_state {
                                            SaveHighscoreState::SAVE => match &*game_status {
                                                GameOverState::Lost => false,
                                                GameOverState::Won { .. } => true,
                                            },
                                            SaveHighscoreState::NOSAVE => false,
                                        },
                                        |ui| {
                                            let discard_button = image_button(
                                                ui,
                                                &egui_textures,
                                                &UITiles::DISCARDBUTTON,
                                                "game_over_screen.discard_tooltip",
                                            );
                                            if discard_button.clicked() {
                                                *save_state = SaveHighscoreState::NOSAVE;
                                            }
                                        },
                                    );
                                });
                            });
                        });
                    });
                });
            });
        });
    egui::TopBottomPanel::bottom("navbar")
        .frame(Frame::none())
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                let back_button = image_button(
                    ui,
                    &egui_textures,
                    &UITiles::BACKBUTTON,
                    "game_over_screen.back_button_tooltip",
                );
                if back_button.clicked() {
                    state
                        .set(AppState::MapSelectionScreen)
                        .expect("Could not switch back to Map Selection Screen");
                }
                let replay_button = image_button(
                    ui,
                    &egui_textures,
                    &UITiles::REPLAYBUTTON,
                    "game_over_screen.replay_button_tooltip",
                );
                if replay_button.clicked() {
                    state
                        .set(AppState::Playing)
                        .expect("Could not switch back to Playing State");
                }
            });
        });
}

fn save_highscores(
    mut commands: Commands,
    config: Res<GameConfig>,
    game_status: Res<GameOverState>,
    map: Res<MapWrapper>,
    mut highscore_database: ResMut<HighscoresDatabaseWrapper>,
    save_state: Res<SaveHighscoreState>,
) {
    match &*game_status {
        GameOverState::Lost => {
            // TODO Save number of retries into the High Score Database in a future release?
        },
        GameOverState::Won { score } => match &*save_state {
            SaveHighscoreState::SAVE => {
                highscore_database.highscores.put_with_current_time(
                    map.world.hash().clone(),
                    config.config.player_id.clone(),
                    Highscore::new(score.moves as u32, score.coins as u32),
                );
                info!("Added Highscore for player {} with {} moves and {} coins in map with hash {} to the highscores database.", config.config.player_id,score.moves,score.coins,map.world.hash_str());
                match highscore_database
                    .highscores
                    .save_to_file(highscore_database.file.as_path())
                {
                    Ok(_) => {
                        info!(
                            "Successfully saved Highscore Database to {}",
                            highscore_database
                                .file
                                .as_path()
                                .to_str()
                                .unwrap_or("<invalid>")
                        );
                    },
                    Err(e) => {
                        error!(
                            "Could not save Highscore Database File at {}: {}",
                            highscore_database
                                .file
                                .as_path()
                                .to_str()
                                .unwrap_or("<invalid>"),
                            e
                        );
                    },
                }
            },
            SaveHighscoreState::NOSAVE => {},
        },
    };
    // Make sure to remove high scores after saving
    commands.remove_resource::<GameOverState>();
}

/// Init Function for the Game Over Screen
fn init_game_over_screen_ui(
    mut commands: Commands,
    config: Res<GameConfig>,
    game_status: Res<GameOverState>,
) {
    commands.insert_resource(match *game_status {
        GameOverState::Lost => SaveHighscoreState::NOSAVE,
        GameOverState::Won { .. } => {
            if config.config.player_id.trim().is_empty() {
                SaveHighscoreState::NOSAVE
            } else {
                SaveHighscoreState::SAVE
            }
        },
    });
}

#[derive(Resource)]
enum SaveHighscoreState {
    SAVE,
    NOSAVE,
}

impl Plugin for GameOverScreen {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::GameOverScreen).with_system(game_over_screen_ui),
        );
        app.add_system_set(
            SystemSet::on_enter(AppState::GameOverScreen).with_system(init_game_over_screen_ui),
        );
        app.add_system_set(
            SystemSet::on_exit(AppState::GameOverScreen).with_system(save_highscores),
        );
    }

    fn name(&self) -> &str {
        "Game Over Screen"
    }
}
