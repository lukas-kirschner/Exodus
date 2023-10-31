use bevy::asset::{AddAsset, AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use libexodus::exodus_serializable::ExodusSerializable;
use libexodus::world::GameWorld;

pub struct CampaignMapAssetPlugin;
impl Plugin for CampaignMapAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<CampaignMapAsset>()
            .init_asset_loader::<CampaignMapLoader>();
    }
}

#[derive(TypeUuid, TypePath)]
#[uuid = "09617fc8-031c-4a4e-ae9f-3cc028c64692"]
pub(crate) struct CampaignMapAsset(pub GameWorld);

#[derive(Default)]
pub(crate) struct CampaignMapLoader;

impl AssetLoader for CampaignMapLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let mut map = GameWorld::default();
            // Bug in Clippy: https://github.com/rust-lang/rust-clippy/issues/8566
            #[allow(noop_method_call)]
            map.parse(&mut bytes.clone())
                .map_err(|e| bevy::asset::Error::msg(e.to_string()))?;
            map.set_filename(load_context.path().to_path_buf());
            let asset = CampaignMapAsset(map);
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["exm"]
    }
}
