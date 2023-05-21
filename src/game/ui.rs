use crate::game::scoreboard::Scoreboard;
use crate::ui::uicontrols::WindowUiOverlayInfo;
use crate::ui::{check_ui_size_changed, UiSizeChangedEvent, UIMARGIN};
use crate::{AppLabels, AppState};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

// The font has been taken from https://ggbot.itch.io/public-pixel-font (CC0 Public Domain)

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Scoreboard>().add_system(
            game_ui_system
                .in_set(OnUpdate(AppState::Playing))
                .in_set(AppLabels::GameUI),
        );
    }
}

#[derive(Component)]
pub struct ScoreboardUICounter;

fn game_ui_system(
    mut egui_ctx: EguiContexts,
    scoreboard: Res<Scoreboard>,
    current_size: ResMut<WindowUiOverlayInfo>,
    mut event_writer: EventWriter<UiSizeChangedEvent>,
) {
    let bot_panel =
        egui::TopBottomPanel::bottom("")
            .resizable(false)
            .show(egui_ctx.ctx_mut(), |ui| {
                ui.vertical(|ui| {
                    ui.scope(|ui| {
                        ui.set_height(UIMARGIN / 2.);
                    });
                    ui.scope(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(format!("Coins: {}", scoreboard.coins));
                            ui.separator();
                            ui.label(format!("Moves: {}", scoreboard.moves));
                            ui.separator();
                            ui.label(format!("Keys: {}", scoreboard.keys));
                        });
                    });
                    ui.scope(|ui| {
                        ui.set_height(UIMARGIN / 2.);
                    });
                });
            });
    let bot_size = bot_panel.response.rect.height();
    check_ui_size_changed(
        &WindowUiOverlayInfo {
            bottom: bot_size,
            ..default()
        },
        current_size,
        &mut event_writer,
    );
}
