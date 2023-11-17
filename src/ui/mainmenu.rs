use crate::textures::egui_textures::atlas_to_egui_textures;
use crate::ui::{BUTTON_HEIGHT, UIMAINMENUMARGIN};
use crate::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::egui::{Align, Frame, Layout, TextStyle};
use bevy_egui::{egui, EguiContexts};

/// Set up the UI for the Main Menu
fn configure_visuals(mut egui_ctx: EguiContexts) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

/// Draw the Main Menu Buttons
fn mainmenu_buttons(
    ui: &mut egui::Ui,
    mut state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    let num_buttons = 5f32;
    ui.scope(|ui| {
        ui.set_height(num_buttons * BUTTON_HEIGHT);
        ui.set_width(400.0);
        ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
            ui.scope(|ui| {
                ui.set_height(BUTTON_HEIGHT);
                ui.centered_and_justified(|ui| {
                    let campaign_btn = ui.button(t!("main_menu.campaign_screen"));
                    if campaign_btn.clicked() {
                        state.set(AppState::CampaignTrailScreen);
                    }
                });
            });
            ui.scope(|ui| {
                ui.set_height(BUTTON_HEIGHT);
                ui.centered_and_justified(|ui| {
                    let maps_btn = ui.button(t!("main_menu.map_selection_screen"));
                    if maps_btn.clicked() {
                        state.set(AppState::MapSelectionScreen);
                    }
                });
            });
            ui.scope(|ui| {
                ui.set_height(BUTTON_HEIGHT);
                ui.centered_and_justified(|ui| {
                    let credits_btn = ui.button(t!("main_menu.credits_screen"));
                    if credits_btn.clicked() {
                        state.set(AppState::CreditsScreen);
                    }
                });
            });
            ui.scope(|ui| {
                ui.set_height(BUTTON_HEIGHT);
                ui.centered_and_justified(|ui| {
                    let config_btn = ui.button(t!("main_menu.config_screen"));
                    if config_btn.clicked() {
                        state.set(AppState::ConfigScreen);
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
        })
    });
}

/// Main Menu main routine
fn mainmenu_ui(
    mut egui_ctx: EguiContexts,
    state: ResMut<NextState<AppState>>,
    exit: EventWriter<AppExit>,
) {
    egui::CentralPanel::default()
        .frame(Frame::none())
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                // Left-Justify everything
                // Margin Left
                ui.add_space(UIMAINMENUMARGIN);
                ui.scope(|ui| {
                    // Set the approximate height and width to center the text vertically.
                    // This is a limitation of any immediate-mode GUI framework (egui)
                    ui.set_height(128.0);
                    // ui.set_width(300.0);
                    ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                        ui.label(
                            egui::RichText::new(
                                format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
                                    .as_str(),
                            )
                            .text_style(TextStyle::Name("MainMenuGameTitle".into())),
                        );
                        if cfg!(debug_assertions) {
                            ui.label(
                                egui::RichText::new(
                                    format!("Debug Build {}", env!("GIT_SHORTHASH")).as_str(),
                                )
                                .text_style(TextStyle::Small),
                            );
                        }
                    });
                });
                ui.add_space(UIMAINMENUMARGIN);
                ui.separator();
                mainmenu_buttons(ui, state, exit);
            });
        });
}

/// The Main Menu Plugin
pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), configure_visuals)
            .add_systems(Update, mainmenu_ui.run_if(in_state(AppState::MainMenu)))
            .add_systems(OnEnter(AppState::MainMenu), atlas_to_egui_textures);
    }
}
