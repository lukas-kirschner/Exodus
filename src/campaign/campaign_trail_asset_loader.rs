use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use libexodus::campaign::graph::Graph;
use libexodus::exodus_serializable::ExodusSerializable;
use std::fmt::Error;

#[derive(Debug, TypeUuid, TypePath)]
#[uuid = "b1cec786-f177-4067-91b7-dc05dc869eb0"]
struct CampaignTrailAsset(pub Graph);
use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::__private::de::Content::ByteBuf;
use bevy::reflect::{TypePath, TypeUuid};

#[derive(Default)]
struct CampaignTrailLoader;

impl AssetLoader for CampaignTrailLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let mut graph = Graph::default();
            graph
                .parse(&mut bytes.clone())
                .map_err(|e| bevy::asset::Error::msg(e.to_string()))?;
            let asset = CampaignTrailAsset(graph);
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}
