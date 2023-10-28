use crate::campaign::campaign_map_asset_loader::*;
use crate::game::tilewrapper::MapWrapper;
use crate::textures::load_asset_folder_or_panic;
use crate::{AllAssetHandles, AppState};
use bevy::app::AppLabel;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use libexodus::world::GameWorld;
use std::path::Path;

pub struct CampaignMapsLoadingPlugin;
impl Plugin for CampaignMapsLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CampaignMapHandles>()
            .init_resource::<CampaignMaps>()
            .add_plugins(CampaignMapAssetPlugin)
            .add_systems(OnEnter(AppState::Loading), load_maps)
            .add_systems(OnEnter(AppState::Process), generate_campaign_maps_resource);
    }
}
#[derive(Resource, Default)]
pub struct CampaignMaps {
    pub maps: HashMap<String, GameWorld>,
}

#[derive(Resource)]
/// A struct containing all loaded handles from the maps folder
pub struct CampaignMapHandles {
    pub handles: Vec<HandleUntyped>,
}
impl FromWorld for CampaignMapHandles {
    fn from_world(_: &mut World) -> Self {
        CampaignMapHandles { handles: vec![] }
    }
}
/// Queue loading all map files in the AssetServer and add handles to both the AllAssetHandles
/// and CampaignMapHandles collection to make sure all maps are loaded before transitioning to the
/// Process state.
fn load_maps(
    mut map_handles: ResMut<CampaignMapHandles>,
    asset_server: Res<AssetServer>,
    mut all_assets: ResMut<AllAssetHandles>,
) {
    map_handles.handles = load_asset_folder_or_panic(&asset_server, "maps");
    all_assets.handles.append(&mut map_handles.handles.clone());
}
fn generate_campaign_maps_resource(
    mut assets: ResMut<Assets<CampaignMapAsset>>,
    map_handles: ResMut<CampaignMapHandles>,
    mut maps: ResMut<CampaignMaps>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    match asset_server.get_group_load_state(map_handles.handles.iter().map(|handle| handle.id())) {
        LoadState::Loaded => {
            for handle in &map_handles.handles {
                let map = assets
                    .remove(handle)
                    .expect("Error removing a map asset from asset manager!")
                    .0;
                let name = map
                    .get_filename()
                    .expect("Could not get file name from map!")
                    .file_name()
                    .expect("Illegal Path in Map")
                    .to_str()
                    .expect("Could not encode file name of map!")
                    .to_string();
                debug!("Successfully loaded campaign map {}", &name);
                maps.maps.insert(name, map);
            }
            commands.remove_resource::<CampaignMapHandles>();
        },
        LoadState::Failed => panic!("Failed to load the campaign maps!"),
        _ => {},
    }
}
