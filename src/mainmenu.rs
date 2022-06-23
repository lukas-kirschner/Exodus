use bevy::prelude::*;
use bevy::app::AppExit;
use bevy_egui::{egui, EguiContext};
use bevy_egui::egui::Frame;
use crate::{AppState};
use crate::uicontrols::NAVBAR_HEIGHT;

/// Set up the UI for the Main Menu
fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

/// Main Menu main routine
fn mainmenu_ui(
    mut egui_ctx: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    egui::CentralPanel::default()
        .frame(Frame::none())
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                ui.vertical_centered(|ui| {
                    ui.scope(|ui| {
                        ui.set_width(400.0);
                        ui.vertical_centered_justified(|ui| {
                            ui.heading(format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).as_str());
                            ui.separator();
                            ui.scope(|ui| {
                                ui.set_height(NAVBAR_HEIGHT);
                                ui.centered_and_justified(|ui| {
                                    let maps_btn = ui.button("Maps");
                                    if maps_btn.clicked() {
                                        state
                                            .set(AppState::MapSelectionScreen)
                                            .expect("Could not switch state to Map Selection Screen");
                                    }
                                });
                            });
                            ui.scope(|ui| {
                                ui.set_height(NAVBAR_HEIGHT);
                                ui.centered_and_justified(|ui| {
                                    let credits_btn = ui.button("Credits");
                                    if credits_btn.clicked() {
                                        state
                                            .set(AppState::CreditsScreen)
                                            .expect("Could not switch state to Credits Screen");
                                    }
                                });
                            });
                            ui.scope(|ui| {
                                ui.set_height(NAVBAR_HEIGHT);
                                ui.centered_and_justified(|ui| {
                                    let quit_btn = ui.button("Quit");
                                    if quit_btn.clicked() {
                                        exit.send(AppExit);
                                    }
                                });
                            });
                        });
                    });
                });
            });
        });
}

/// The Main Menu Plugin
pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::MainMenu)
                                .with_system(configure_visuals),
            )
            .add_system_set(SystemSet::on_update(AppState::MainMenu)
                .with_system(mainmenu_ui)
            )
        // .add_system_set(SystemSet::on_update(AppState::MainMenu)
        //     .with_system(button_press_system)
        // )
        // .add_system_set(SystemSet::on_exit(AppState::MainMenu)
        //     .with_system(cleanup)
        // )
        ;
    }
}