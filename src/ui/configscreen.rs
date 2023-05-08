use crate::ui::egui_textures::EguiButtonTextures;
use crate::ui::uicontrols::{add_navbar, menu_esc_control};
use crate::ui::{UIBIGMARGIN, UIMARGIN, UIPANELWIDTH};
use crate::{AppState, GameConfig, TilesetManager};
use bevy::prelude::*;
use bevy_egui::egui::Frame;
use bevy_egui::{egui, EguiContext};
use libexodus::config::Language;
use libexodus::tilesets::Tileset;
use strum::IntoEnumIterator;

pub struct ConfigScreen;

impl Plugin for ConfigScreen {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::ConfigScreen).with_system(config_screen_ui),
        )
        .add_system_set(SystemSet::on_update(AppState::ConfigScreen).with_system(menu_esc_control))
        .add_system_set(
            SystemSet::on_exit(AppState::ConfigScreen).with_system(save_and_apply_config),
        );
    }
}

fn config_screen_ui(
    mut egui_ctx: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    mut res_config: ResMut<GameConfig>,
    egui_textures: Res<EguiButtonTextures>,
) {
    add_navbar(&mut egui_ctx, &mut state, &egui_textures);

    egui::CentralPanel::default()
        .frame(Frame::none())
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                ui.vertical_centered(|ui| {
                    ui.scope(|ui| {
                        ui.set_width(UIPANELWIDTH);
                        ui.vertical_centered_justified(|ui| {
                            ui.set_width(UIPANELWIDTH - UIBIGMARGIN);
                            ui.scope(|ui| {
                                ui.set_height(UIMARGIN);
                            });
                            ui.label(format!("{}:", t!("config_screen.language_label")));
                            let selected_lang = res_config.config.game_language.to_string();
                            egui::ComboBox::from_id_source("lang_box")
                                .selected_text(selected_lang)
                                .show_ui(ui, |ui| {
                                    ui.set_width(400.);
                                    for lang in Language::iter() {
                                        ui.selectable_value(
                                            &mut res_config.config.game_language,
                                            lang,
                                            &lang.to_string(),
                                        );
                                    }
                                });
                            ui.separator();
                            ui.label(format!("{}:", t!("config_screen.tileset_label")));
                            let selected_tileset = res_config.config.tile_set.to_string();
                            egui::ComboBox::from_id_source("tile_set_box")
                                .selected_text(selected_tileset)
                                .show_ui(ui, |ui| {
                                    ui.set_width(UIPANELWIDTH - UIBIGMARGIN - UIBIGMARGIN);
                                    for tileset in Tileset::iter() {
                                        ui.selectable_value(
                                            &mut res_config.config.tile_set,
                                            tileset,
                                            &tileset.to_string(),
                                        );
                                    }
                                });
                            ui.separator();
                            ui.label(format!("{}:", t!("config_screen.player_name_label")));
                            ui.text_edit_singleline(&mut res_config.config.player_id)
                                .on_hover_text(t!("config_screen.player_name_tooltip"));
                        });
                    });
                });
            });
        });
}

fn save_and_apply_config(res_config: Res<GameConfig>, mut res_tileset: ResMut<TilesetManager>) {
    res_config
        .config
        .save_to_file(res_config.file.as_path())
        .map_err(|err| {
            error!(
                "Could not save config file {} - {}",
                res_config.file.as_path().to_str().unwrap_or("<Invalid>"),
                err.to_string()
            )
        })
        .map(|_| {
            debug!(
                "Saved Config File to {}",
                res_config.file.as_path().to_str().unwrap()
            );
        })
        .unwrap_or(());
    // Set Locale
    rust_i18n::set_locale(res_config.config.game_language.locale());
    // Set Tile Set
    res_tileset.current_tileset = res_config.config.tile_set;
}
