use crate::game::camera::{LayerCamera, MainCamera, compute_world_to_viewport};
use crate::game::player::PlayerComponent;
use crate::game::scoreboard::Scoreboard;
use crate::game::tilewrapper::MapWrapper;
use crate::textures::egui_textures::EguiButtonTextures;
use crate::ui::uicontrols::WindowUiOverlayInfo;
use crate::ui::{UIMARGIN, UiSizeChangedEvent, check_ui_size_changed};
use crate::{AppLabels, AppState, GameConfig};
use bevy::prelude::*;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Align, Align2, Layout};
use bevy_egui::{EguiContexts, egui};
use libexodus::player::Player;
use libexodus::tiles::Tile;
use regex::Regex;

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

fn game_ui_system(
    mut egui_ctx: EguiContexts,
    scoreboard: Res<Scoreboard>,
    current_size: ResMut<WindowUiOverlayInfo>,
    mut event_writer: EventWriter<UiSizeChangedEvent>,
    textures: Res<EguiButtonTextures>,
) {
    let bot_panel =
        egui::TopBottomPanel::bottom("")
            .resizable(false)
            .show(egui_ctx.ctx_mut().unwrap(), |ui| {
                ui.vertical(|ui| {
                    ui.add_space(UIMARGIN / 2.);
                    ui.scope(|ui| {
                        ui.set_height(16.);
                        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                            let h = ui.label(t!("game_ui.moves")).rect.height();
                            ui.image(SizedTexture::new(
                                textures.textures[&Player::atlas_index_right()].0,
                                (h, h),
                            ));
                            ui.label(format!("{}", scoreboard.moves));

                            ui.separator();
                            ui.label(t!("game_ui.coins"));
                            ui.image(SizedTexture::new(
                                textures.textures[&Tile::COIN.atlas_index().unwrap()].0,
                                (h, h),
                            ));
                            ui.label(format!("{}", scoreboard.coins));

                            if scoreboard.keys > 0 {
                                ui.separator();
                            }
                            // ui.label(t!("game_ui.keys"));
                            // ui.image(SizedTexture::new(
                            //     textures.textures[&Tile::KEY.atlas_index().unwrap()].0,
                            //     (h, h),
                            // ));
                            // ui.label(format!("{}", scoreboard.keys));
                            for _ in 0..scoreboard.keys {
                                ui.image(SizedTexture::new(
                                    textures.textures[&Tile::KEY.atlas_index().unwrap()].0,
                                    (h, h),
                                ));
                            }
                            if scoreboard.crystals > 0 {
                                ui.separator();
                            }
                            for _ in 0..scoreboard.crystals {
                                ui.image(SizedTexture::new(
                                    textures.textures[&Tile::STARCRYSTAL.atlas_index().unwrap()].0,
                                    (h, h),
                                ));
                            }
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
#[derive(Debug)]
enum WindowAlignTo {
    TopLeft,
    BotLeft,
    TopRight,
    BotRight,
}
impl From<WindowAlignTo> for Align2 {
    fn from(value: WindowAlignTo) -> Self {
        match value {
            WindowAlignTo::TopLeft => Align2::RIGHT_BOTTOM,
            WindowAlignTo::BotLeft => Align2::RIGHT_TOP,
            WindowAlignTo::TopRight => Align2::LEFT_BOTTOM,
            WindowAlignTo::BotRight => Align2::LEFT_TOP,
        }
    }
}

/// Calculate the placement coordinates for message subwindows
fn message_window_position(
    ctx: &egui::Context,
    player_position: &Transform,
    main_camera: &Camera,
    main_camera_transform: &GlobalTransform,
    layer_camera: &Camera,
    layer_camera_transform: &GlobalTransform,
    texture_size: f32,
) -> (f32, f32, WindowAlignTo) {
    let available_rect = ctx.available_rect();
    let (screen_x, screen_y) = available_rect.size().into();
    // Bevy coordinates have an inverted y axis
    let player_screen = compute_world_to_viewport(
        &player_position.translation,
        main_camera,
        main_camera_transform,
        layer_camera,
        layer_camera_transform,
        texture_size,
    )
    .unwrap();
    let player_ui_x = player_screen.x + available_rect.min.x;
    let player_ui_y = player_screen.y + available_rect.min.y;
    let player_left_half = player_ui_x > (available_rect.min.x + (screen_x / 2.));
    let player_bottom_half = player_ui_y < (available_rect.min.y + (screen_y / 2.));
    let align_to = match (player_left_half, player_bottom_half) {
        (true, true) => WindowAlignTo::BotLeft,
        (false, true) => WindowAlignTo::BotRight,
        (true, false) => WindowAlignTo::TopLeft,
        (false, false) => WindowAlignTo::TopRight,
    };
    /// Number of tiles that a floating window is offset from the player's center
    const OFFSET: f32 = 3.75;
    let offset_player_pos_x = if player_left_half {
        player_ui_x - OFFSET * texture_size
    } else {
        player_ui_x + OFFSET * texture_size
    };
    let offset_player_pos_y = if player_bottom_half {
        player_ui_y + OFFSET * texture_size
    } else {
        player_ui_y - OFFSET * texture_size
    };
    (offset_player_pos_x, offset_player_pos_y, align_to)
}
/// System that shows a message to the user when they enter a message sign
fn sign_message_system(
    mut egui_ctx: EguiContexts,
    player_positions: Query<&Transform, With<PlayerComponent>>,
    worldwrapper: Res<MapWrapper>,
    config: Res<GameConfig>,
    q_layer_camera: Query<(&Camera, &GlobalTransform), With<LayerCamera>>,
    q_main_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (layer_camera, layer_camera_transform) = q_layer_camera
        .get_single()
        .expect("There were multiple layer cameras spawned");
    let (main_camera, main_camera_transform) = q_main_camera
        .get_single()
        .expect("There were multiple main cameras spawned");
    let mut messages_to_show: Vec<&str> = vec![];
    let mut first_player_pos: Option<&Transform> = None;
    for player_position in player_positions.iter() {
        let player_map_position = player_position.translation / Vec3::splat(config.texture_size());
        if let Some(Tile::MESSAGE { message_id }) = worldwrapper
            .world
            .get(player_map_position.x as i32, player_map_position.y as i32)
        {
            if let Some(message) = worldwrapper.world.get_message(*message_id) {
                messages_to_show.push(message);
                first_player_pos = Some(player_position);
            }
        }
    }
    if let Some(player_position) = first_player_pos {
        let (xpos, ypos, message_alignment) = message_window_position(
            egui_ctx.ctx_mut().unwrap(),
            player_position,
            main_camera,
            main_camera_transform,
            layer_camera,
            layer_camera_transform,
            config.texture_size(),
        );
        let _sign_floating_window = egui::Window::new("Message Window")
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .title_bar(false)
            .interactable(false)
            // Set max width to 1/3 of available screen size
            .fixed_size(egui_ctx.ctx_mut().unwrap().available_rect().size() / 3.)
            // move window to the left of the player position if the player position is > 1/2 of the screen width, right otherwise.
            // move window to top-align with player position, if player is >1/2 of screen height, bottom-align otherwise.
            .pivot(message_alignment.into())
            .fixed_pos((xpos, ypos))
            // Show the actual message inside a label. If there are multiple players triggering
            // messages simultaneously, show all messages concatenated with a " / ".
            .show(egui_ctx.ctx_mut().unwrap(), |ui| {
            ui.with_layout(Layout::top_down(Align::TOP),|ui| {
                ui.label(messages_to_show.iter().map(|m| parse_text(m)).collect::<Vec<String>>().join(" / "));
            });
            });
    }
}
/// Parse all special text occurrences inside the given text, e.g. Translations
fn parse_text(text: &str) -> String {
    let mut ret = text.to_string();
    let re_translations = Regex::new(r"t!\(([^)]+)\)").unwrap();
    while let Some(caps) = re_translations.captures(ret.as_str()) {
        let cap_start = caps.get(1).unwrap().start() - 3;
        let cap_end = caps.get(1).unwrap().end() + 1;
        let cap_inner = caps.get(1).unwrap().as_str();
        ret = format!("{}{}{}", &ret[0..cap_start], t!(cap_inner), &ret[cap_end..]);
    }
    ret
}
