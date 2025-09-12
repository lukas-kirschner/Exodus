use crate::AppState;
use crate::textures::egui_textures::EguiButtonTextures;
use crate::ui::{BUTTON_HEIGHT, image_button};
/// This module contains UI elements and styles that are reusable throughout the program
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::egui::{Align, InnerResponse, TextStyle};
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
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
    current_app_state: ResMut<State<AppState>>,
) {
    if *current_app_state != AppState::MainMenu && keys.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::MainMenu);
        keys.reset(KeyCode::Escape);
    }
}

pub fn add_navbar(
    ctx: &mut egui::Context,
    state: &mut NextState<AppState>,
    egui_textures: &EguiButtonTextures,
    title: &str,
) -> InnerResponse<()> {
    add_navbar_with_extra_buttons(ctx, state, egui_textures, title, |_, _| {}, 0)
}

pub fn add_navbar_with_extra_buttons<R>(
    ctx: &mut egui::Context,
    state: &mut NextState<AppState>,
    egui_textures: &EguiButtonTextures,
    title: &str,
    extra_buttons: impl FnOnce(&mut egui::Ui, &mut NextState<AppState>) -> R,
    num_extra_buttons: usize,
) -> InnerResponse<()> {
    egui::TopBottomPanel::top("navbar").show(ctx, |ui| {
        ui.set_height(BUTTON_HEIGHT + 2. * ctx.style().spacing.item_spacing.y);
        ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
            ui.label(
                egui::RichText::new(title)
                    .text_style(TextStyle::Heading)
                    .color(ui.visuals().weak_text_color()),
            );
            ui.add_space(
                ui.available_width()
                    - BUTTON_HEIGHT
                    - (BUTTON_HEIGHT * num_extra_buttons as f32)
                    - (ctx.style().spacing.item_spacing.x * num_extra_buttons as f32),
            );
            extra_buttons(ui, state);
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
    })
}
