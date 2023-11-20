use crate::textures::egui_textures::EguiButtonTextures;
use crate::ui::{image_button, BUTTON_HEIGHT};
use crate::AppState;
/// This module contains UI elements and styles that are reusable throughout the program
use bevy::prelude::*;
use bevy_egui::egui::{Align, InnerResponse, TextStyle};
use bevy_egui::{egui, EguiContexts};
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

pub fn menu_esc_control(
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
    current_app_state: ResMut<State<AppState>>,
) {
    if *current_app_state != AppState::MainMenu && keys.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::MainMenu);
        keys.reset(KeyCode::Escape);
    }
}

pub fn add_navbar(
    egui_ctx: &mut EguiContexts,
    state: &mut NextState<AppState>,
    egui_textures: &EguiButtonTextures,
    title: &str,
) -> InnerResponse<()> {
    egui::TopBottomPanel::top("navbar").show(egui_ctx.ctx_mut(), |ui| {
        ui.set_height(BUTTON_HEIGHT);
        ui.with_layout(egui::Layout::right_to_left(Align::BOTTOM), |ui| {
            ui.label(egui::RichText::new(title).text_style(TextStyle::Heading));
            ui.add_space(ui.available_width() - BUTTON_HEIGHT);
            ui.scope(|ui| {
                ui.set_width(BUTTON_HEIGHT);
                ui.set_height(BUTTON_HEIGHT);
                let back_button = image_button(
                    ui,
                    egui_textures,
                    &UITiles::BACKBUTTON,
                    "navbar.back_button_tooltip",
                );
                if back_button.clicked() {
                    state.set(AppState::MainMenu);
                }
            });
        });
    })
}
