use crate::game::pickup_item::PickupItemPlugin;
use crate::AppState;
use bevy::prelude::*;

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
            .add_system_set(
                SystemSet::on_update(AppState::Playing).with_system(back_to_main_menu_controls),
            )
            .add_system_set(SystemSet::on_exit(AppState::Playing).with_system(cleanup))
            .add_system_set(
                SystemSet::on_enter(AppState::Playing)
                    .with_system(reset_score)
                    .label("reset_score"),
            );
    }
}

fn back_to_main_menu_controls(
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if *app_state.current() == AppState::Playing && keys.just_pressed(KeyCode::Escape) {
        app_state
            .set(AppState::MainMenu)
            .expect("Could not go back to Main Menu");
        keys.reset(KeyCode::Escape);
    }
}

/// Cleanup every entity that is present in the stage. Used for State Changes
fn cleanup(mut commands: Commands, query: Query<Entity>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
