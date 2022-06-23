use std::borrow::BorrowMut;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_egui::egui::{Frame, RichText};
use indoc::formatdoc;
use crate::AppState;
use crate::uicontrols::{add_navbar, menu_esc_control, MenuMaterials, NAVBAR_BACK_TEXT};

pub struct CreditsScreen;

struct CreditsScreenData {
    camera_entity: Entity,
    ui_root: Entity,
}

#[derive(Component)]
enum CreditsScreenButton {
    Quit,
}

fn credits() -> String {
    formatdoc! {"
        {program_name} Version {version}

        This program is licensed under a {license}.
        The sprites were created by dancramp (CC BY 4.0)
         https://opengameart.org/content/tiny-platform-quest-sprites
        ",
            program_name = env!("CARGO_PKG_NAME"),
            version = env!("CARGO_PKG_VERSION"),
            license = "MIT License",
    }
}

/// Main Menu main routine
fn credits_screen_ui(
    mut egui_ctx: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
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
                            ui.heading(format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).as_str());
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
        app
            .add_system_set(SystemSet::on_update(AppState::CreditsScreen)
                .with_system(credits_screen_ui)
            )
            .add_system_set(SystemSet::on_update(AppState::CreditsScreen)
                .with_system(menu_esc_control)
            )
        ;
    }
}