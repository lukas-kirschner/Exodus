use crate::game::pickup_item::PickupItemPlugin;
use crate::{AppLabels, AppState, GameConfig};
use bevy::prelude::*;
use libexodus::highscores::highscores_database::HighscoresDatabase;
use std::path::PathBuf;

pub mod camera;
pub mod constants;
mod pickup_item;
pub mod player;
pub mod scoreboard;
pub mod tilewrapper;
mod ui;
mod vending_machine;
pub(crate) mod world;

use crate::game::player::{PlayerPlugin, ReturnTo};
use crate::game::tilewrapper::{reset_score, MapWrapper};
use crate::game::ui::GameUIPlugin;
use crate::game::vending_machine::VendingMachinePlugin;
use crate::game::world::WorldPlugin;
use crate::textures::egui_textures::atlas_to_egui_textures;
use crate::textures::tileset_manager::TilesetManager;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapWrapper>()
            .add_plugins(WorldPlugin)
            .add_plugins(GameUIPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(PickupItemPlugin)
            .add_plugins(VendingMachinePlugin)
            .add_systems(
                Update,
                back_with_esc_controls.run_if(in_state(AppState::Playing)),
            )
            .add_systems(
                OnEnter(AppState::Playing),
                reset_score.in_set(AppLabels::ResetScore),
            )
            .add_systems(
                OnTransition {
                    from: AppState::MapSelectionScreen,
                    to: AppState::Playing,
                },
                (load_texture_pack, atlas_to_egui_textures)
                    .chain()
                    .in_set(AppLabels::PrepareData),
            )
            .add_systems(
                OnTransition {
                    from: AppState::CampaignTrailScreen,
                    to: AppState::Playing,
                },
                (load_texture_pack, atlas_to_egui_textures)
                    .chain()
                    .in_set(AppLabels::PrepareData),
            )
            .add_systems(
                OnTransition {
                    from: AppState::GameOverScreen,
                    to: AppState::Playing,
                },
                (load_texture_pack, atlas_to_egui_textures)
                    .chain()
                    .in_set(AppLabels::PrepareData),
            )
            .add_systems(
                OnExit(AppState::Playing),
                (load_texture_pack_from_config, atlas_to_egui_textures).chain(),
            )
            .add_systems(
                OnExit(AppState::ConfigScreen),
                (load_texture_pack_from_config, atlas_to_egui_textures)
                    .chain()
                    .in_set(AppLabels::PrepareData),
            );
    }
}

/// Set the loaded texture pack to the map-specific texture pack, if there is a forced texture pack set in the map
fn load_texture_pack(
    res_config: Res<GameConfig>,
    mut res_tileset: ResMut<TilesetManager>,
    mapwrapper: Res<MapWrapper>,
) {
    if let Some(map_texture_pack) = mapwrapper.world.forced_tileset() {
        info!(
            "Found a map-specific texture pack: {}. Setting the texture pack",
            map_texture_pack
        );
        res_tileset.current_tileset = map_texture_pack;
    } else {
        info!(
            "There was no map-specific texture pack configured inside the map. Keeping {}.",
            &res_config.config.tile_set
        );
        res_tileset.current_tileset = res_config.config.tile_set;
    }
}

/// Set the loaded texture pack to the texture pack set in the config
pub fn load_texture_pack_from_config(
    res_config: Res<GameConfig>,
    mut res_tileset: ResMut<TilesetManager>,
) {
    info!(
        "Re-setting texture pack to {}.",
        &res_config.config.tile_set
    );
    res_tileset.current_tileset = res_config.config.tile_set;
}

#[derive(Resource)]
pub struct HighscoresDatabaseWrapper {
    pub highscores: HighscoresDatabase,
    pub file: PathBuf,
}

fn back_with_esc_controls(
    keys: ResMut<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
    return_to: Res<ReturnTo>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        app_state.set(return_to.0);
    }
}
