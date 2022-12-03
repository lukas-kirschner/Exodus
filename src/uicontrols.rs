/// This module contains UI elements and styles that are reusable throughout the program
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_egui::egui::{Align, FontId};
use bevy_egui::egui::FontFamily::Proportional;
use crate::AppState;
use crate::game::constants::MAPEDITOR_BUTTON_SIZE;

/// The height of the Navbar
pub const NAVBAR_HEIGHT: f32 = 32.0;
/// The height of the Map Editor Controls Bar
pub const MAPEDITOR_CONTROLS_HEIGHT: f32 = (MAPEDITOR_BUTTON_SIZE + 2.) * 2.;
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

#[derive(Resource, PartialEq, Copy, Clone)]
pub struct WindowUiOverlayInfo {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Default for WindowUiOverlayInfo {
    fn default() -> Self {
        WindowUiOverlayInfo {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: 0.0,
        }
    }
}

pub struct UiSizeChangedEvent;

pub fn check_ui_size_changed(
    new_size: &WindowUiOverlayInfo,
    mut current_size: ResMut<WindowUiOverlayInfo>,
    event_writer: &mut EventWriter<UiSizeChangedEvent>,
) {
    if *new_size != *current_size {
        *current_size = (*new_size).clone();
        event_writer.send(UiSizeChangedEvent);
        println!("Changed UI Overlay to T {:?} B {:?} L {:?} R{:?}", new_size.top, new_size.bottom, new_size.left, new_size.right);
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
    egui_ctx: &mut ResMut<EguiContext>,
    state: &mut ResMut<State<AppState>>,
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