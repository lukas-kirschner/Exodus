use crate::game::constants::MENU_SQUARE_BUTTON_SIZE;
use crate::ui::configscreen::ConfigScreen;
use crate::ui::creditsscreen::CreditsScreen;
use crate::ui::mainmenu::MainMenu;
use crate::ui::mapselectionscreen::MapSelectionScreenPlugin;
use crate::WindowUiOverlayInfo;
use bevy::prelude::*;

mod configscreen;
mod creditsscreen;
pub mod egui_textures;
pub mod mainmenu;
mod mapselectionscreen;
pub mod uicontrols;

/// The button height of main menu buttons
pub const BUTTON_HEIGHT: f32 = MENU_SQUARE_BUTTON_SIZE;
/// The margin of UI elements that must not touch each other
pub const UIMARGIN: f32 = 4.0;
/// The text used for the Navbar Back Button
pub const NAVBAR_BACK_TEXT: &str = "\u{300a}";

pub struct Ui;

impl Plugin for Ui {
    fn build(&self, app: &mut App) {
        app.add_plugin(MainMenu)
            .add_plugin(MapSelectionScreenPlugin)
            .add_plugin(CreditsScreen)
            .add_plugin(ConfigScreen);
    }
}

pub struct UiSizeChangedEvent;

pub fn check_ui_size_changed(
    new_size: &WindowUiOverlayInfo,
    mut current_size: ResMut<WindowUiOverlayInfo>,
    event_writer: &mut EventWriter<UiSizeChangedEvent>,
) {
    if *new_size != *current_size {
        *current_size = *new_size;
        event_writer.send(UiSizeChangedEvent);
        debug!(
            "Changed UI Overlay to T {:?} B {:?} L {:?} R{:?}",
            new_size.top, new_size.bottom, new_size.left, new_size.right
        );
    }
}
