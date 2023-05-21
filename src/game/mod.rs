use crate::game::pickup_item::PickupItemPlugin;
use crate::{AppLabels, AppState};
use bevy::prelude::*;
use libexodus::highscores::highscores_database::HighscoresDatabase;
use std::path::PathBuf;

pub mod camera;
pub mod constants;
mod pickup_item;
mod player;
pub mod scoreboard;
pub mod tilewrapper;
mod ui;
pub(crate) mod world;

use crate::game::player::PlayerPlugin;
use crate::game::tilewrapper::{reset_score, MapWrapper};
use crate::game::ui::GameUIPlugin;
use crate::game::world::WorldPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapWrapper>()
            .add_plugin(WorldPlugin)
            .add_plugin(GameUIPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(PickupItemPlugin)
            .add_system(back_to_main_menu_controls.in_set(OnUpdate(AppState::Playing)))
            .add_system(
                reset_score
                    .in_schedule(OnEnter(AppState::Playing))
                    .in_set(AppLabels::ResetScore),
            );
    }
}

#[derive(Resource)]
pub struct HighscoresDatabaseWrapper {
    pub highscores: HighscoresDatabase,
    pub file: PathBuf,
}

fn back_to_main_menu_controls(
    mut keys: ResMut<Input<KeyCode>>,
    current_app_state: ResMut<State<AppState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if current_app_state.0 == AppState::Playing && keys.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::MainMenu);
        keys.reset(KeyCode::Escape);
    }
}
