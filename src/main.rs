use std::borrow::BorrowMut;
use std::fs;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_egui::{EguiContext, EguiPlugin};
use indoc::printdoc;
use libexodus::directories::GameDirectories;
use crate::creditsscreen::CreditsScreen;
use crate::game::GamePlugin;
use crate::mainmenu::MainMenu;
use crate::mapselectionscreen::MapSelectionScreenPlugin;
use crate::uicontrols::{egui_fonts, UiControlsPlugin};

mod game;
mod mainmenu;
mod creditsscreen;
mod uicontrols;
mod mapselectionscreen;

// We use https://opengameart.org/content/tiny-platform-quest-sprites free textures
// TODO !!! Textures are CC-BY-SA 3.0
// TODO There is a bug in Bevy that causes adjacent textures from the atlas to leak through due to precision errors: https://github.com/bevyengine/bevy/issues/1949


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    MapSelectionScreen,
    CreditsScreen,
    Playing,
}

struct GameDirectoriesWrapper {
    pub game_directories: GameDirectories,
}

impl FromWorld for GameDirectoriesWrapper {
    fn from_world(_: &mut World) -> Self {
        GameDirectoriesWrapper {
            game_directories: GameDirectories::from_system_config().map_err(|err| format!("Invalid system configuration! Error: {}", err)).unwrap()
        }
    }
}

/// Main init method for the game.
/// This method ensures that all necessary directories actually exist and are writable.
fn game_init(
    directories: Res<GameDirectoriesWrapper>,
    mut ctx: ResMut<EguiContext>,
) {
    if !directories.game_directories.maps_dir.as_path().exists() {
        fs::create_dir_all(&directories.game_directories.maps_dir)
            .expect(format!("Could not create the map directory at {}!", directories.game_directories.maps_dir.as_path().to_str().unwrap_or("<Invalid>")).as_str());
    }
    if !directories.game_directories.config_dir.as_path().exists() {
        fs::create_dir_all(&directories.game_directories.config_dir)
            .expect(format!("Could not create the config directory at {}!", directories.game_directories.config_dir.as_path().to_str().unwrap_or("<Invalid>")).as_str());
    }
    printdoc! {"
    Using directory structure:
        Maps directory: {maps_dir}
        Config Directory: {config_dir}
    ",
        maps_dir = &directories.game_directories.maps_dir.as_path().to_str().unwrap_or("<Invalid Path>"),
        config_dir = &directories.game_directories.config_dir.as_path().to_str().unwrap_or("<Invalid Path>")
    }
    // Initialize Styling and fonts for egui
    egui_fonts(ctx.ctx_mut());
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Exodus".to_string(),
            resizable: false,
            width: 1001.,
            height: 501.,
            cursor_visible: true,
            decorations: true,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .init_resource::<GameDirectoriesWrapper>()
        .add_startup_system(game_init)
        .add_state(AppState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(UiControlsPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(MainMenu)
        .add_plugin(MapSelectionScreenPlugin)
        .add_plugin(CreditsScreen)
        .run();
}
