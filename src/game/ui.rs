use crate::game::player::PlayerComponent;
use crate::game::scoreboard::Scoreboard;
use crate::game::tilewrapper::MapWrapper;
use crate::ui::uicontrols::WindowUiOverlayInfo;
use crate::ui::{check_ui_size_changed, UiSizeChangedEvent, UIMARGIN};
use crate::{AppLabels, AppState, GameConfig};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use libexodus::tiles::Tile;

// The font has been taken from https://ggbot.itch.io/public-pixel-font (CC0 Public Domain)

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Scoreboard>().add_systems(
            Update,
            (game_ui_system, sign_message_system)
                .chain()
                .run_if(in_state(AppState::Playing))
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

fn sign_message_system(
    mut egui_ctx: EguiContexts,
    player_positions: Query<&Transform, With<PlayerComponent>>,
    worldwrapper: Res<MapWrapper>,
    config: Res<GameConfig>,
) {
    let mut messages_to_show: Vec<&str> = vec![];
    for player_position in player_positions.iter() {
        let player_map_position = player_position.translation / Vec3::splat(config.texture_size());
        if let Some(Tile::MESSAGE { message_id }) = worldwrapper
            .world
            .get(player_map_position.x as i32, player_map_position.y as i32)
        {
            if let Some(message) = worldwrapper.world.get_message(*message_id) {
                messages_to_show.push(message);
            }
        }
    }
    if !messages_to_show.is_empty() {
        let _sign_floating_window = egui::Window::new("Floating Message Window")
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .title_bar(false)
            .show(egui_ctx.ctx_mut(), |ui| {
                // Set max width to 1/4 of available screen size
                // Show the actual message inside a label. If there are multiple players triggering
                // messages simultaneously, show all messages concatenated with a " / ".
                ui.label(messages_to_show.join(" / "));
            });
        // move window to the left of the player position if the player position is > 1/2 of the screen width, right otherwise.
        // move window to top-align with player position, if player is >1/2 of screen height, bottom-align otherwise.
    }
}
