use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use libexodus::exodus_serializable::ExodusSerializable;
use libexodus::world::GameWorld;
use libexodus::world::io_error::GameWorldParseError;

pub struct CampaignMapAssetPlugin;

impl Plugin for CampaignMapAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<CampaignMapAsset>()
            .register_asset_loader(CampaignMapLoader);
    }
}

#[derive(TypePath, Asset)]
pub(crate) struct CampaignMapAsset(pub GameWorld);

#[derive(Default)]
pub(crate) struct CampaignMapLoader;

impl AssetLoader for CampaignMapLoader {
    type Asset = CampaignMapAsset;
    type Settings = ();
    type Error = GameWorldParseError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let mut map = GameWorld::default();
        // Bug in Clippy: https://github.com/rust-lang/rust-clippy/issues/8566
        #[allow(noop_method_call)]
        map.parse(&mut bytes.as_slice().clone())?;
        map.set_filename(load_context.path().to_path_buf());
        let asset = CampaignMapAsset(map);
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["exm"]
    }
}
