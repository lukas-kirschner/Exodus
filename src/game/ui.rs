use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use crate::AppState;
use crate::game::scoreboard::Scoreboard;
use crate::uicontrols::NAVBAR_HEIGHT;

// The font has been taken from https://ggbot.itch.io/public-pixel-font (CC0 Public Domain)

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Scoreboard>()
            .add_system_set(SystemSet::on_update(AppState::Playing)
                .with_system(game_ui_system)
            )
        ;
    }
}

#[derive(Component)]
pub struct ScoreboardUICounter;

fn game_ui_system(
    mut egui_ctx: ResMut<EguiContext>,
    scoreboard: Res<Scoreboard>,
) {
    egui::TopBottomPanel::bottom("")
        .resizable(false)
        .min_height(NAVBAR_HEIGHT)
        .max_height(NAVBAR_HEIGHT)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(format!("Coins: {}", scoreboard.coins));
                ui.separator();
                ui.label(format!("Moves: {}", scoreboard.moves));
                ui.separator();
                ui.label(format!("Keys: {}", scoreboard.keys));
            })
        });
}