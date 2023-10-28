use crate::game::constants::MENU_SQUARE_BUTTON_SIZE;
use crate::textures::egui_textures::EguiButtonTextures;
use crate::ui::configscreen::ConfigScreen;
use crate::ui::creditsscreen::CreditsScreen;
use crate::ui::game_over_screen::GameOverScreen;
use crate::ui::mainmenu::MainMenu;
use crate::ui::mapselectionscreen::MapSelectionScreenPlugin;
use crate::WindowUiOverlayInfo;
use bevy::prelude::*;
use bevy_egui::egui;
use libexodus::tiles::UITiles;

mod configscreen;
mod creditsscreen;
pub mod game_over_screen;
pub mod mainmenu;
mod mapselectionscreen;
pub mod uicontrols;

/// The height of the bottom info panel in the campaign screen
pub const CAMPAIGN_MAPINFO_HEIGHT: f32 = 2. * MENU_SQUARE_BUTTON_SIZE;

/// The button height of main menu buttons
pub const BUTTON_HEIGHT: f32 = MENU_SQUARE_BUTTON_SIZE;
/// The margin of UI elements that must not touch each other
pub const UIMARGIN: f32 = 4.0;
/// Big Margins to use as outer margins for boxes and sub-windows
pub const UIBIGMARGIN: f32 = 50.0;
/// The default width of a centered UI panel
pub const UIPANELWIDTH: f32 = 600.0;

pub struct Ui;

impl Plugin for Ui {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenu)
            .add_plugins(MapSelectionScreenPlugin)
            .add_plugins(CreditsScreen)
            .add_plugins(GameOverScreen)
            .add_plugins(ConfigScreen);
    }
}
#[derive(Event)]
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
/// Create an image button to display in the UI
pub(crate) fn image_button(
    ui: &mut bevy_egui::egui::Ui,
    egui_textures: &EguiButtonTextures,
    tile: &UITiles,
    translationkey: &str,
) -> bevy_egui::egui::Response {
    let (id, size, uv) = egui_textures.textures.get(&tile.atlas_index().unwrap()) // Edit Button Texture
        .unwrap_or_else(|| panic!("Textures for Edit Button were not loaded as Egui textures!"));
    ui.add_sized(
        [MENU_SQUARE_BUTTON_SIZE, MENU_SQUARE_BUTTON_SIZE],
        egui::ImageButton::new(*id, *size).uv(*uv),
    )
    .on_hover_text(t!(translationkey))
}
