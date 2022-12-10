use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_egui::egui::Frame;
use libexodus::config::Language;
use strum::IntoEnumIterator;
use crate::{AppState, GameConfig};
use crate::ui::uicontrols::{add_navbar, menu_esc_control};
use crate::ui::UIMARGIN;

pub struct ConfigScreen;

impl Plugin for ConfigScreen {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(AppState::ConfigScreen)
                .with_system(config_screen_ui)
            )
            .add_system_set(SystemSet::on_update(AppState::ConfigScreen)
                .with_system(menu_esc_control)
            )
            .add_system_set(SystemSet::on_exit(AppState::ConfigScreen)
                .with_system(save_config)
            )
        ;
    }
}

fn config_screen_ui(
    mut egui_ctx: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    mut res_config: ResMut<GameConfig>,
) {
    add_navbar(&mut egui_ctx, &mut state);

    egui::CentralPanel::default()
        .frame(Frame::none())
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                ui.vertical_centered(|ui| {
                    ui.scope(|ui| {
                        ui.set_width(600.0);
                        ui.vertical_centered_justified(|ui| {
                            ui.set_width(550.0);
                            ui.scope(|ui| {
                                ui.set_height(UIMARGIN);
                            });
                            ui.label(format!("{}:", t!("config_screen.language_label")));
                            let selected_lang = res_config.config.game_language.to_string();
                            egui::ComboBox::from_id_source("lang_box")
                                .selected_text(format!("{}", &selected_lang))
                                .show_ui(ui, |ui| {
                                    ui.set_width(400.);
                                    for lang in Language::iter() {
                                        ui.selectable_value(&mut res_config.config.game_language, lang, &lang.to_string());
                                    }
                                });
                            ui.separator();
                        });
                    });
                });
            });
        });
}

fn save_config(
    res_config: Res<GameConfig>,
) {
    res_config.config.save_to_file(res_config.file.as_path())
        .map_err(|err| {
            error!("Could not save config file {} - {}",res_config.file.as_path().to_str().unwrap_or("<Invalid>"),err.to_string())
        })
        .map(|_| {
            debug!("Saved Config File to {}",res_config.file.as_path().to_str().unwrap());
        }).unwrap_or(());
    rust_i18n::set_locale(res_config.config.game_language.locale());
}