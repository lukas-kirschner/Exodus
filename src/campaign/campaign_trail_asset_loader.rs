use bevy::asset::io::Reader;
use bevy::asset::AsyncReadExt;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use libexodus::campaign::graph::{Graph, GraphParseError};
use libexodus::exodus_serializable::ExodusSerializable;

pub struct CampaignTrailAssetPlugin;

impl Plugin for CampaignTrailAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<CampaignTrailAsset>()
            .register_asset_loader(CampaignTrailLoader);
    }
}

#[derive(Debug, TypePath, Asset)]
pub(crate) struct CampaignTrailAsset(pub Graph);

#[derive(Default)]
pub(crate) struct CampaignTrailLoader;

impl AssetLoader for CampaignTrailLoader {
    type Asset = CampaignTrailAsset;
    type Settings = ();
    type Error = GraphParseError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let mut graph = Graph::default();
        // Bug in Clippy: https://github.com/rust-lang/rust-clippy/issues/8566
        #[allow(noop_method_call)]
        graph.parse(&mut bytes.as_slice().clone())?;
        let asset = CampaignTrailAsset(graph);
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["tgf"]
    }
}
