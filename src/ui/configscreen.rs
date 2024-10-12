use crate::textures::egui_textures::EguiButtonTextures;
use crate::ui::uicontrols::{add_navbar, menu_esc_control};
use crate::ui::{UIBIGMARGIN, UIMARGIN, UIPANELCBWIDTH, UIPANELWIDTH};
use crate::{AppState, GameConfig};
use bevy::prelude::*;
use bevy_egui::egui::{Align, Frame, Layout};
use bevy_egui::{egui, EguiContexts};
use libexodus::config::Language;
use libexodus::tilesets::Tileset;
use strum::IntoEnumIterator;

pub struct ConfigScreen;

impl Plugin for ConfigScreen {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            config_screen_ui.run_if(in_state(AppState::ConfigScreen)),
        )
        .add_systems(
            Update,
            menu_esc_control.run_if(in_state(AppState::ConfigScreen)),
        )
        .add_systems(OnExit(AppState::ConfigScreen), save_and_apply_config);
    }
}

fn config_screen_ui(
    mut egui_ctx: EguiContexts,
    mut state: ResMut<NextState<AppState>>,
    mut res_config: ResMut<GameConfig>,
    egui_textures: Res<EguiButtonTextures>,
) {
    add_navbar(
        egui_ctx.ctx_mut(),
        &mut state,
        &egui_textures,
        &t!("config_screen.title"),
    );

    egui::CentralPanel::default()
        .frame(Frame::none())
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.group(|ui| {
                    ui.set_width(UIPANELWIDTH);
                    ui.set_height(ui.available_height());
                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
                        ui.set_width(UIPANELWIDTH - UIBIGMARGIN);
                        ui.add_space(UIMARGIN);
                        ui.label(format!("{}:", t!("config_screen.language_label")));
                        ui.scope(|ui| {
                            ui.set_width(UIPANELCBWIDTH);
                            let selected_lang = res_config.config.game_language.to_string();
                            egui::ComboBox::from_id_salt("lang_box")
                                .width(UIPANELCBWIDTH)
                                .selected_text(selected_lang)
                                .show_ui(ui, |ui| {
                                    for lang in Language::iter() {
                                        ui.selectable_value(
                                            &mut res_config.config.game_language,
                                            lang,
                                            lang.to_string(),
                                        );
                                    }
                                })
                                .response
                                .on_hover_text(t!("config_screen.language_tooltip"));
                        });
                        ui.separator();
                        ui.add_space(UIMARGIN);
                        ui.label(format!("{}:", t!("config_screen.tileset_label")));
                        ui.scope(|ui| {
                            ui.set_width(UIPANELCBWIDTH);
                            let selected_tileset = res_config.config.tile_set.to_string();
                            egui::ComboBox::from_id_salt("tile_set_box")
                                .width(UIPANELCBWIDTH)
                                .selected_text(selected_tileset)
                                .show_ui(ui, |ui| {
                                    for tileset in Tileset::iter() {
                                        ui.selectable_value(
                                            &mut res_config.config.tile_set,
                                            tileset,
                                            tileset.to_string(),
                                        );
                                    }
                                })
                                .response
                                .on_hover_text(t!("config_screen.tileset_tooltip"));
                        });
                        ui.separator();
                        ui.add_space(UIMARGIN);
                        ui.label(format!("{}:", t!("config_screen.player_name_label")));
                        ui.scope(|ui| {
                            ui.set_width(UIPANELCBWIDTH);
                            ui.text_edit_singleline(&mut res_config.config.player_id)
                                .on_hover_text(t!("config_screen.player_name_tooltip"));
                        });
                    });
                });
            });
        });
}

fn save_and_apply_config(res_config: Res<GameConfig>) {
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
    // The tile set is reset in game/mod.rs.
}
