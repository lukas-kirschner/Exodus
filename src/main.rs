use bevy::prelude::*;
use bevy::window::WindowMode;

mod constants;

use crate::constants::*;
use crate::mainmenu::MainMenu;

mod player;

use crate::player::*;
use crate::scoreboard::Scoreboard;

mod tilewrapper;

use crate::tilewrapper::*;
use crate::ui::{GameUIPlugin, scoreboard_ui_system, setup_game_ui};

mod scoreboard;

mod ui;

pub mod world;

use crate::world::*;

mod mainmenu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    Playing,
}
// We use https://opengameart.org/content/8x8-resource-pack and https://opengameart.org/content/tiny-platform-quest-sprites free textures
// TODO !!! Textures are CC-BY-SA 3.0
// TODO There is a bug in Bevy that causes adjacent textures from the atlas to leak through due to precision errors: https://github.com/bevyengine/bevy/issues/1949


/// Cleanup every entity that is present in the stage. Used for State Changes
fn cleanup(mut commands: Commands, query: Query<Entity>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
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
        .add_state(AppState::MainMenu)
        .add_plugins(DefaultPlugins)
        .init_resource::<MapWrapper>()
        .add_plugin(WorldPlugin)
        .add_plugin(MainMenu)
        .add_plugin(GameUIPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CoinPlugin)
        .run();
}
