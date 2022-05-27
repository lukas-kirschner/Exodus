use bevy::prelude::*;
use bevy::window::WindowMode;
use crate::game::GamePlugin;
use crate::mainmenu::MainMenu;

mod game;
mod mainmenu;

// We use https://opengameart.org/content/8x8-resource-pack and https://opengameart.org/content/tiny-platform-quest-sprites free textures
// TODO !!! Textures are CC-BY-SA 3.0
// TODO There is a bug in Bevy that causes adjacent textures from the atlas to leak through due to precision errors: https://github.com/bevyengine/bevy/issues/1949


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    Playing,
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
        .add_state(AppState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(MainMenu)
        .run();
}
