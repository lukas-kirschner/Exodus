use crate::game::player::{
    despawn_exited_player, despawn_players, keyboard_controls, player_movement, setup_player,
    GameOverEvent,
};
use crate::game::tilewrapper::MapWrapper;
use crate::textures::egui_textures::EguiButtonTextures;
use crate::ui::uicontrols::{add_navbar, menu_esc_control, WindowUiOverlayInfo};
use crate::ui::{check_ui_size_changed, UiSizeChangedEvent};
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
use libexodus::campaign::graph::{Coord, Graph, NodeKind};
use libexodus::tiles::{InteractionKind, Tile};
use libexodus::world::GameWorld;
use std::cmp::{max, min};

/// A struct that holds all maps that may be played in the campaign trail
#[derive(Resource)]
struct CampaignMaps {
    maps: Vec<MapWrapper>,
}

#[derive(Component)]
pub struct CampaignTrail {
    pub trail: Graph,
    pub last_player_position: (Coord, Coord),
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
            Update,
            menu_esc_control
                .run_if(in_state(AppState::CampaignTrailScreen))
                .in_set(AppLabels::GameUI),
        )
        .add_systems(
            OnEnter(AppState::CampaignTrailScreen),
            reset_trail
                .in_set(AppLabels::PrepareData)
                .before(AppLabels::World),
            //TODO Player Movement
            //TODO Key controls to play a map
            //TODO UI that shows a previous highscore to the player and that lets the player enter a map, if they are on an appropriate tile
        )
        .add_systems(
            Update,
            keyboard_controls.run_if(in_state(AppState::CampaignTrailScreen)),
        )
        .add_systems(
            OnEnter(AppState::CampaignTrailScreen),
            setup_player
                .after(AppLabels::World)
                .after(AppLabels::ResetScore)
                .in_set(AppLabels::Player),
        )
        .add_systems(
            Update,
            player_movement
                .run_if(in_state(AppState::CampaignTrailScreen))
                .in_set(AppLabels::PlayerMovement),
        )
        .add_systems(
            Update,
            despawn_exited_player
                .run_if(in_state(AppState::CampaignTrailScreen))
                .in_set(AppLabels::GameOverTrigger),
        )
        .add_systems(
            Update,
            play_map_event_listener
                .run_if(in_state(AppState::Playing))
                .in_set(AppLabels::GameOverTrigger),
        )
        .add_systems(OnExit(AppState::CampaignTrailScreen), despawn_players);
    }
}
fn play_map_event_listener(
    mut reader: EventReader<GameOverEvent>,
    mut state: ResMut<NextState<AppState>>,
    mut commands: Commands,
) {
    if let Some(event) = reader.iter().next() {
        // commands.insert_resource(event.state.clone());
        // state.set(AppState::GameOverScreen);
    }
}

/// Load the current campaign trail as "map" (MapWrapper) and place the player spawn at the last position.
/// This is executed in the PrepareData set, and the map is loaded and displayed immediately after loading the trail in world.rs.
/// All maps that are 'behind' non-finished maps are automatically 'locked', therefore
/// each time a map has been newly unlocked, this function must be called to unlock those maps
/// in the campaign trail.
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
    world.fill(&Tile::CAMPAIGNTRAILBORDER);
    world.set(
        (trail.last_player_position.0 + offset_x) as usize,
        (trail.last_player_position.1 + offset_y) as usize,
        Tile::PLAYERSPAWN,
    );
    for node in trail_graph.nodes() {
        world.set(
            (node.coord.0 + offset_x) as usize,
            (node.coord.1 + offset_y) as usize,
            match &node.kind {
                NodeKind::Empty => Tile::CAMPAIGNTRAILWALKWAY,
                NodeKind::MapFilename { map } => Tile::CAMPAIGNTRAILMAPENTRYPOINT {
                    interaction: InteractionKind::LaunchMap {
                        map_name: map.clone(),
                    },
                },
            },
        );
    }
    for (edge_from, edges_to) in trail_graph.edges() {
        for edge_to in edges_to {
            let (from_x, from_y) = trail_graph.get_node(edge_from).unwrap().coord;
            let (to_x, to_y) = trail_graph.get_node(edge_to).unwrap().coord;
            if from_x == to_x {
                for y in (min(from_y, to_y) + 1)..max(from_y, to_y) {
                    world.set(
                        (from_x + offset_x) as usize,
                        (y + offset_y) as usize,
                        Tile::CAMPAIGNTRAILWALKWAY,
                    );
                }
            } else {
                for x in (min(from_x, to_x) + 1)..max(from_x, to_x) {
                    world.set(
                        (x + offset_x) as usize,
                        (from_y + offset_y) as usize,
                        Tile::CAMPAIGNTRAILWALKWAY,
                    );
                }
            }
        }
    }
    // TODO Traverse the graph using a breadth-first search to unlock all maps that are already won, or reachable from a won map
    // TODO Load all campaign maps as asset and assign them into the CampaignMaps resource
    debug!(
        "Loaded a campaign trail with size {0}x{1}, Offset {2}x{3} and player spawn at {4},{5} in a world size of {6}x{7}",
        trail.trail.width(),
        trail.trail.height(),
        offset_x,
        offset_y,
        trail.last_player_position.0,
        trail.last_player_position.1,
        world.width(),
        world.height()
    );
    commands.insert_resource(MapWrapper {
        world,
        previous_best: None,
    });
}
fn campaign_screen_ui(
    mut egui_ctx: EguiContexts,
    mut state: ResMut<NextState<AppState>>,
    current_window_size: ResMut<WindowUiOverlayInfo>,
    egui_textures: Res<EguiButtonTextures>,
    mut window_size_event_writer: EventWriter<UiSizeChangedEvent>,
) {
    let navbar_response = add_navbar(&mut egui_ctx, &mut state, &egui_textures);
    let ui_height = navbar_response.response.rect.height();
    check_ui_size_changed(
        &WindowUiOverlayInfo {
            top: ui_height,
            ..default()
        },
        current_window_size,
        &mut window_size_event_writer,
    );
}
