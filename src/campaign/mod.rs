use crate::campaign::campaign_maps::CampaignMapsLoadingPlugin;
use crate::campaign::campaign_trail::{CampaignTrail, MainCampaignTrail, SelectedCampaignTrail};
use crate::campaign::campaign_trail_asset_loader::CampaignTrailAsset;
use crate::{AllAssetHandles, AppState};
use bevy::app::{App, Plugin};
use bevy::asset::{AssetServer, LoadState};
use bevy::prelude::*;

pub mod campaign_map_asset_loader;
pub mod campaign_maps;
pub mod campaign_trail;
pub mod campaign_trail_asset_loader;

pub struct MainCampaignLoader;
/// Plugin that loads the main campaign trail from the campaign.tgf file
impl Plugin for MainCampaignLoader {
    fn build(&self, app: &mut App) {
        app.add_plugins(CampaignMapsLoadingPlugin)
            .add_systems(Startup, load_main_campaign);
        app.add_systems(
            Update,
            insert_main_campaign.run_if(in_state(AppState::Process)),
        );
    }
}
#[derive(Component)]
struct MainCampaign {
    handle: Handle<CampaignTrailAsset>,
}

fn load_main_campaign(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut all_assets: ResMut<AllAssetHandles>,
) {
    debug!("Loading Main Campaign Trail Asset from {}", "campaign.tgf");
    let handle = asset_server.load("campaign.tgf");
    all_assets.file_handles.push(handle.clone().untyped());
    commands
        .spawn(MainCampaign { handle })
        .insert(MainCampaignTrail);
}

/// Removes all loaded Graph Assets that are marked with the MainCampaignTrail marker struct
/// and loads them into CampaignTrails
fn insert_main_campaign(
    mut state: ResMut<NextState<AppState>>,
    mut assets: ResMut<Assets<CampaignTrailAsset>>,
    main_asset: Query<(Entity, &MainCampaign), With<MainCampaignTrail>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (entity, asset) in &main_asset {
        match asset_server.get_load_state(&asset.handle) {
            Some(LoadState::Loaded) => {
                let graph = assets.remove(&asset.handle).expect(
                    "The Campaign Trail was removed from the asset manager before loading!",
                );
                commands.entity(entity).despawn_recursive();
                commands
                    .spawn(CampaignTrail {
                        trail: graph.0,
                        ..default()
                    })
                    .insert(MainCampaignTrail)
                    .insert(SelectedCampaignTrail);
                // TODO Change this line as soon as multiple campaign trails are supported:.insert(SelectedCampaignTrail);
                // TODO Change this to support multiple campaign trails:
                state.set(AppState::MainMenu);
            },
            Some(LoadState::Failed) => panic!(
                "Failed to load the Campaign Trail from {}",
                asset_server
                    .get_path(&asset.handle)
                    .expect("Could not get path from handle!")
                    .path()
                    .to_str()
                    .unwrap()
            ),
            _ => {},
        }
    }
}
