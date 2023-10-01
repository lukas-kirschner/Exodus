use crate::game::tilewrapper::MapWrapper;
use crate::textures::egui_textures::EguiButtonTextures;
use crate::ui::uicontrols::add_navbar;
/// This file contains all required UI and logic structs that are required to show the user a
/// campaign trail where they can choose a map to play and save their progress while doing so.
/// Since in the future, multiple campaign trails may be supported, we derive the campaign trail
/// storage struct from Component instead of Resource.
///
/// A campaign trail is shown as world in the same way as an usual "Game World" is shown to the
/// user when they play the game, therefore we try to re-use as many functions from the game
/// world here as possible.
/// Especially the movement, camera and tile placement functions are exactly the same, except in the
/// campaign screen, the player is not affected by gravity and may move upwards or downwards.
use crate::{AppLabels, AppState};
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use libexodus::campaign::graph::Graph;
use libexodus::tiles::Tile;
use libexodus::world::GameWorld;

/// A struct that holds all maps that may be played in the campaign trail
#[derive(Resource)]
struct CampaignMaps {
    maps: Vec<MapWrapper>,
}

#[derive(Component)]
pub struct CampaignTrail {
    pub trail: Graph,
    pub last_player_position: (usize, usize),
}

impl Default for CampaignTrail {
    fn default() -> Self {
        CampaignTrail {
            trail: Graph::default(),
            last_player_position: (0, 0),
        }
    }
}

/// Marker Struct that marks the main campaign trail entry point
#[derive(Component)]
pub struct MainCampaignTrail;
/// Marker Struct that marks the currently selected campaign trail that should be displayed in the
/// Campaign Trail Screen
#[derive(Component)]
pub struct SelectedCampaignTrail;
pub struct CampaignTrailPlugin;
impl Plugin for CampaignTrailPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            campaign_screen_ui
                .run_if(in_state(AppState::CampaignTrailScreen))
                .in_set(AppLabels::GameUI),
        )
        .add_systems(
            OnEnter(AppState::CampaignTrailScreen),
            reset_trail.in_set(AppLabels::PrepareData),
            //TODO Player Movement
            //TODO Key controls to play a map
            //TODO UI that shows a previous highscore to the player and that lets the player enter a map, if they are on an appropriate tile
        );
    }
}
/// Load the current campaign trail as "map" (MapWrapper) and place the player spawn at the last position.
/// This is executed in the PrepareData set, and the map is loaded and displayed immediately after loading the trail in world.rs.
fn reset_trail(
    trail_query: Query<&CampaignTrail, With<SelectedCampaignTrail>>,
    mut commands: Commands,
) {
    let trail: &CampaignTrail = match trail_query.get_single() {
        Ok(trail) => trail,
        Err(e) => {
            error!("{}", e);
            return;
        },
    };
    let trail_graph = &trail.trail;
    let offset_x = -trail_graph.min_x();
    let offset_y = -trail_graph.min_y();
    let mut world = GameWorld::new(trail_graph.width(), trail_graph.height());
    for node in trail_graph.nodes() {
        world.set(
            (node.coord.0 + offset_x) as usize,
            (node.coord.1 + offset_y) as usize,
            Tile::ARROWDOWN,
        );
    }
    commands.insert_resource(MapWrapper {
        world,
        previous_best: None,
    });
}
fn campaign_screen_ui(
    mut commands: Commands,
    mut egui_ctx: EguiContexts,
    mut state: ResMut<NextState<AppState>>,
    egui_textures: Res<EguiButtonTextures>,
    maps: Res<CampaignMaps>,
) {
    add_navbar(&mut egui_ctx, &mut state, &egui_textures);
}
