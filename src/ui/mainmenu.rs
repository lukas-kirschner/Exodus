use crate::textures::egui_textures::atlas_to_egui_textures;
use crate::ui::{BUTTON_HEIGHT, UIMARGIN};
use crate::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::egui::Frame;
use bevy_egui::{egui, EguiContext};

/// Set up the UI for the Main Menu
fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

/// Draw the Main Menu Buttons
fn mainmenu_buttons(
    ui: &mut egui::Ui,
    mut state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    ui.scope(|ui| {
        // Scope for the buttons
        ui.horizontal(|ui| {
            // Left-Align Buttons
            ui.scope(|ui| {
                // Button Width
                ui.set_width(400.0);
                ui.vertical_centered_justified(|ui| {
                    ui.scope(|ui| {
                        ui.set_height(BUTTON_HEIGHT);
                        ui.centered_and_justified(|ui| {
                            let maps_btn = ui.button(t!("main_menu.map_selection_screen"));
                            if maps_btn.clicked() {
                                state
                                    .set(AppState::MapSelectionScreen)
                                    .expect("Could not switch state to Map Selection Screen");
                            }
                        });
                    });
                    ui.scope(|ui| {
                        ui.set_height(BUTTON_HEIGHT);
                        ui.centered_and_justified(|ui| {
                            let credits_btn = ui.button(t!("main_menu.credits_screen"));
                            if credits_btn.clicked() {
                                state
                                    .set(AppState::CreditsScreen)
                                    .expect("Could not switch state to Credits Screen");
                            }
                        });
                    });
                    ui.scope(|ui| {
                        ui.set_height(BUTTON_HEIGHT);
                        ui.centered_and_justified(|ui| {
                            let config_btn = ui.button(t!("main_menu.config_screen"));
                            if config_btn.clicked() {
                                state
                                    .set(AppState::ConfigScreen)
                                    .expect("Could not switch state to Config Screen");
                            }
                        });
                    });
                    ui.scope(|ui| {
                        ui.set_height(BUTTON_HEIGHT);
                        ui.centered_and_justified(|ui| {
                            let quit_btn = ui.button(t!("main_menu.quit"));
                            if quit_btn.clicked() {
                                exit.send(AppExit);
                            }
                        });
                    });
                });
            });
        });
    });
}

/// Main Menu main routine
fn mainmenu_ui(
    mut egui_ctx: ResMut<EguiContext>,
    state: ResMut<State<AppState>>,
    exit: EventWriter<AppExit>,
) {
    egui::CentralPanel::default()
        .frame(Frame::none())
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.centered_and_justified(|ui| {
                ui.scope(|ui| {
                    ui.horizontal(|ui| {
                        // Left-Justify everything
                        ui.scope(|ui| {
                            // Margin Left
                            ui.set_width(UIMARGIN)
                        });
                        ui.heading(
                            format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
                                .as_str(),
                        );
                        ui.separator();
                        mainmenu_buttons(ui, state, exit);
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
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu)
                    .with_system(atlas_to_egui_textures),
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
