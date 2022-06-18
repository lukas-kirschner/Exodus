use bevy::prelude::*;
use bevy::window::WindowMode;
use libexodus::directories::GameDirectories;
use crate::creditsscreen::CreditsScreen;
use crate::game::GamePlugin;
use crate::mainmenu::MainMenu;
use crate::mapselectionscreen::MapSelectionScreenPlugin;
use crate::uicontrols::UiControlsPlugin;

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
    fn from_world(world: &mut World) -> Self {
        GameDirectoriesWrapper {
            game_directories: GameDirectories::from_system_config().map_err(|err| format!("Invalid system configuration! Error: {}", err)).unwrap()
        }
    }
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
        .add_state(AppState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(UiControlsPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(MainMenu)
        .add_plugin(MapSelectionScreenPlugin)
        .add_plugin(CreditsScreen)
        .run();
}
