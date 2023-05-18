use crate::textures::egui_textures::EguiButtonTextures;
use crate::ui::{image_button, BUTTON_HEIGHT};
use crate::AppState;
/// This module contains UI elements and styles that are reusable throughout the program
use bevy::prelude::*;
use bevy_egui::egui::Align;
use bevy_egui::{egui, EguiContext};
use libexodus::tiles::UITiles;

#[derive(Resource, PartialEq, Copy, Clone, Debug)]
pub struct WindowUiOverlayInfo {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Default for WindowUiOverlayInfo {
    fn default() -> Self {
        WindowUiOverlayInfo {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: 0.0,
        }
    }
}

pub fn menu_esc_control(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if *app_state.current() != AppState::MainMenu && keys.just_pressed(KeyCode::Escape) {
        app_state
            .set(AppState::MainMenu)
            .expect("Could not return to Main Menu");
        keys.reset(KeyCode::Escape);
    }
}

pub fn add_navbar(
    egui_ctx: &mut ResMut<EguiContext>,
    state: &mut ResMut<State<AppState>>,
    egui_textures: &EguiButtonTextures,
) {
    egui::TopBottomPanel::top("navbar").show(egui_ctx.ctx_mut(), |ui| {
        ui.set_height(BUTTON_HEIGHT);
        ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
            ui.scope(|ui| {
                ui.set_width(BUTTON_HEIGHT);
                ui.centered_and_justified(|ui| {
                    let back_button = image_button(
                        ui,
                        egui_textures,
                        &UITiles::BACKBUTTON,
                        "navbar.back_button_tooltip",
                    );
                    if back_button.clicked() {
                        state
                            .set(AppState::MainMenu)
                            .expect("Could not switch back to Main Menu");
                    }
                });
            });
        });
    });
}
