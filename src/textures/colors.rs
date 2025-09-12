use crate::AppState;
use bevy::color::Color;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
/// System that resets the colors of menus that are shown in full-screen mode
pub fn set_menu_colors(mut commands: Commands, mut egui_ctx: EguiContexts) {
    let ctx = egui_ctx.ctx_mut().unwrap();
    let visuals = &ctx.style().visuals;
    commands.insert_resource(ClearColor(Color::srgb_u8(
        visuals.window_fill.r(),
        visuals.window_fill.g(),
        visuals.window_fill.b(),
    )));
}

pub struct ColorsPlugin;

impl Plugin for ColorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), set_menu_colors)
            .add_systems(OnEnter(AppState::MapSelectionScreen), set_menu_colors)
            .add_systems(OnEnter(AppState::CreditsScreen), set_menu_colors)
            .add_systems(OnEnter(AppState::ConfigScreen), set_menu_colors);
    }
}
