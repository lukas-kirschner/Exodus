use bevy::asset::{AddAsset, AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::*;
use libexodus::campaign::graph::Graph;
use libexodus::exodus_serializable::ExodusSerializable;

pub struct CampaignTrailAssetPlugin;
impl Plugin for CampaignTrailAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<CampaignTrailAsset>()
            .init_asset_loader::<CampaignTrailLoader>();
    }
}

#[derive(Debug, TypeUuid, TypePath)]
#[uuid = "b1cec786-f177-4067-91b7-dc05dc869eb0"]
pub(crate) struct CampaignTrailAsset(pub Graph);
use bevy::reflect::{TypePath, TypeUuid};

#[derive(Default)]
pub(crate) struct CampaignTrailLoader;

impl AssetLoader for CampaignTrailLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let mut graph = Graph::default();
            // Bug in Clippy: https://github.com/rust-lang/rust-clippy/issues/8566
            #[allow(noop_method_call)]
            graph
                .parse(&mut bytes.clone())
                .map_err(|e| bevy::asset::Error::msg(e.to_string()))?;
            let asset = CampaignTrailAsset(graph);
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tgf"]
    }
}
