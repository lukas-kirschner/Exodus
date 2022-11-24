use bevy::prelude::*;
use crate::AppState;
use crate::game::pickup_item::PickupItemPlugin;

pub mod constants;
mod player;
pub mod tilewrapper;
pub mod scoreboard;
mod ui;
pub(crate) mod world;
pub mod camera;
mod pickup_item;

use crate::game::player::PlayerPlugin;
use crate::game::tilewrapper::{MapWrapper, reset_score};
use crate::game::ui::GameUIPlugin;
use crate::game::world::WorldPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MapWrapper>()
            .add_plugin(WorldPlugin)
            .add_plugin(GameUIPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(PickupItemPlugin)

            .add_system_set(SystemSet::on_update(AppState::Playing)
                .with_system(back_to_main_menu_controls)
            )

            .add_system_set(SystemSet::on_exit(AppState::Playing)
                .with_system(cleanup)
            )
            .add_system_set(SystemSet::on_enter(AppState::Playing)
                .with_system(reset_score).label("reset_score")
            )
        ;
    }
}

fn back_to_main_menu_controls(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if *app_state.current() == AppState::Playing {
        if keys.just_pressed(KeyCode::Escape) {
            app_state.set(AppState::MainMenu).expect("Could not go back to Main Menu");
            keys.reset(KeyCode::Escape);
        }
    }
}

/// Cleanup every entity that is present in the stage. Used for State Changes
fn cleanup(mut commands: Commands, query: Query<Entity>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
