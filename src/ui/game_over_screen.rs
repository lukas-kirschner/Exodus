use crate::game::tilewrapper::GameOverState;
use crate::ui::egui_textures::EguiButtonTextures;
use crate::ui::{image_button, UIBIGMARGIN, UIMARGIN, UIPANELWIDTH};
use crate::AppState;
use bevy::prelude::*;
use bevy_egui::egui::Frame;
use bevy_egui::{egui, EguiContext};
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
    // config: Res<GameConfig>,
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
                                ui.label(format!("{}!", t!("game_over_screen.game_over_heading")));
                            });
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
                                },
                            });
                            //TODO Table with previous best/Map Name/Coins,Moves/Player Name
                            //TODO Button to discard results?
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
                    save_highscores();
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
                    save_highscores();
                    state
                        .set(AppState::Playing)
                        .expect("Could not switch back to Playing State");
                }
            });
        });
}

fn save_highscores() {}

impl Plugin for GameOverScreen {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::GameOverScreen).with_system(game_over_screen_ui),
        );
    }

    fn name(&self) -> &str {
        "Game Over Screen"
    }
}
