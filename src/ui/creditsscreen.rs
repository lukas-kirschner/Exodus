use crate::textures::egui_textures::EguiButtonTextures;
use crate::ui::uicontrols::{add_navbar, menu_esc_control};
use crate::ui::UIMARGIN;
use crate::{get_buildnr, AppState};
use bevy::prelude::*;
use bevy_egui::egui::{Frame, RichText};
use bevy_egui::{egui, EguiContext};
use indoc::formatdoc;

pub struct CreditsScreen;

fn credits() -> String {
    let buildnr = get_buildnr();
    let mut ret: String = formatdoc! {"
        {program_name} Version {version}{buildnr}
        based on the \"Space Exodus\" Psion EPOC game
        by David Sansome (2001)

        This program is licensed under a {license}.
        The Tiny Platform Quest Sprites were created by dancramp (CC BY 4.0)
         https://opengameart.org/content/tiny-platform-quest-sprites
         Changes were made to the original sprites.
        ",
            program_name = env!("CARGO_PKG_NAME"),
            version = env!("CARGO_PKG_VERSION"),
            license = "MIT License",
            buildnr = buildnr,
    };
    if cfg!(debug_assertions) {
        ret.push_str(
            format!(
                "\nDebug Build {build} ({date})",
                build = env!("GIT_SHORTHASH"),
                date = env!("GIT_SHORTDATE")
            )
            .as_str(),
        );
    }
    ret
}

/// Main Menu main routine
fn credits_screen_ui(
    mut egui_ctx: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    egui_textures: Res<EguiButtonTextures>,
) {
    add_navbar(&mut egui_ctx, &mut state, &egui_textures);

    egui::CentralPanel::default()
        .frame(Frame::none())
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                ui.vertical_centered(|ui| {
                    ui.scope(|ui| {
                        ui.set_width(600.0);
                        ui.vertical_centered_justified(|ui| {
                            ui.scope(|ui| {
                                ui.set_height(UIMARGIN);
                            });
                            ui.heading(
                                format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
                                    .as_str(),
                            );
                            ui.separator();
                            ui.label(RichText::new(credits()).text_style(egui::TextStyle::Small));
                        });
                    });
                });
            });
        });
}

impl Plugin for CreditsScreen {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::CreditsScreen).with_system(credits_screen_ui),
        )
        .add_system_set(
            SystemSet::on_update(AppState::CreditsScreen).with_system(menu_esc_control),
        );
    }
}
