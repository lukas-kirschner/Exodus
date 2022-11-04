/// This module contains UI elements and styles that are reusable throughout the program
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_egui::egui::{Align, FontDefinitions, FontFamily, FontId};
use bevy_egui::egui::FontFamily::{Monospace, Proportional};
use crate::AppState;
use crate::game::constants::MENU_BORDER_WIDTH;

/// The height of the Navbar
pub const NAVBAR_HEIGHT: f32 = 32.0;
/// The margin of UI elements that must not touch each other
pub const UIMARGIN: f32 = 4.0;
/// The text used for the Navbar Back Button
pub const NAVBAR_BACK_TEXT: &str = "\u{300a}";
/// The text used for the Play Button
pub const PLAY_TEXT: &str = "\u{300b}";
/// The text used for the Delete Button
pub const DELETE_TEXT: &str = "\u{2020}";
/// The text used for the Delete Button
pub const EDIT_TEXT: &str = "E";

pub struct MenuMaterials {
    pub root: UiColor,
    pub border: UiColor,
    pub menu: UiColor,
    pub button: UiColor,
    pub button_hovered: UiColor,
    pub button_pressed: UiColor,
    pub navbar: UiColor,
    pub button_text: Color,
}

impl FromWorld for MenuMaterials {
    fn from_world(_: &mut World) -> Self {
        MenuMaterials {
            root: Color::NONE.into(),
            border: Color::rgb(0.65, 0.65, 0.65).into(),
            menu: Color::rgb(0.15, 0.15, 0.15).into(),
            button: Color::rgb(0.15, 0.15, 0.15).into(),
            button_hovered: Color::rgb(0.25, 0.25, 0.25).into(),
            button_pressed: Color::rgb(0.35, 0.75, 0.35).into(),
            navbar: Color::rgb(0.10, 0.10, 0.10).into(),
            button_text: Color::WHITE,
        }
    }
}

pub fn menu_esc_control(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if *app_state.current() != AppState::MainMenu {
        if keys.just_pressed(KeyCode::Escape) {
            app_state.set(AppState::MainMenu).expect("Could not go back to Main Menu");
            keys.reset(KeyCode::Escape);
        }
    }
}

pub fn egui_fonts(ctx: &egui::Context) -> () {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert("exodus".to_owned(),
                           egui::FontData::from_static(include_bytes!("../assets/fonts/PublicPixel.ttf")));
    fonts.families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "exodus".to_owned());
    fonts.families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("exodus".to_owned());
    ctx.set_fonts(fonts);

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (egui::TextStyle::Heading, FontId::new(30.0, Proportional)),
        (egui::TextStyle::Name("Heading2".into()), FontId::new(25.0, Proportional)),
        (egui::TextStyle::Name("Context".into()), FontId::new(23.0, Proportional)),
        (egui::TextStyle::Body, FontId::new(18.0, Proportional)),
        (egui::TextStyle::Monospace, FontId::new(16.0, Proportional)),
        (egui::TextStyle::Button, FontId::new(20.0, Proportional)),
        (egui::TextStyle::Small, FontId::new(10.0, Proportional)),
    ]
        .into();
    ctx.set_style(style);
}

pub fn add_navbar(
    mut egui_ctx: &mut ResMut<EguiContext>,
    mut state: &mut ResMut<State<AppState>>,
) {
    egui::TopBottomPanel::top("navbar").show(egui_ctx.ctx_mut(), |ui| {
        ui.set_height(NAVBAR_HEIGHT);
        ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
            ui.scope(|ui| {
                ui.set_width(NAVBAR_HEIGHT);
                ui.centered_and_justified(|ui| {
                    let back_button = ui.button(NAVBAR_BACK_TEXT);

                    if back_button.clicked() {
                        state
                            .set(AppState::MainMenu)
                            .expect("Could not switch back to Main Menu");
                    }
                });
            });
        });
    });
}

pub struct UiControlsPlugin;

impl Plugin for UiControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MenuMaterials>()
        ;
    }
}