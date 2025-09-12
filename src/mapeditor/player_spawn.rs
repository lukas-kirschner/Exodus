use crate::game::constants::PLAYER_Z;
use crate::game::tilewrapper::MapWrapper;
use crate::{LAYER_ID, TilesetManager};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use libexodus::player::Player;

#[derive(Component)]
pub struct PlayerSpawnComponent {
    pub player: Player,
}

pub fn init_player_spawn(
    mut commands: Commands,
    tileset: Res<TilesetManager>,
    world_wrapper: ResMut<MapWrapper>,
) {
    // Code Duplication from player.rs - but we need to change things later
    let player: PlayerSpawnComponent = PlayerSpawnComponent {
        player: Player::new(),
    };
    let (map_player_position_x, map_player_position_y) = world_wrapper.world.player_spawn();
    let layer = RenderLayers::layer(LAYER_ID);
    commands.spawn((
        Sprite::from_atlas_image(
            tileset.current_texture_handle(),
            TextureAtlas {
                layout: tileset.current_atlas_handle(),
                index: player.player.atlas_index(),
            },
        ),
        Transform::from_translation(Vec3::new(
            (map_player_position_x as u32 * tileset.current_tileset.texture_size()) as f32,
            (map_player_position_y as u32 * tileset.current_tileset.texture_size()) as f32,
            PLAYER_Z,
        )),
        player,
        layer,
    ));
}

pub fn destroy_player_spawn(
    mut commands: Commands,
    player_query: Query<Entity, With<PlayerSpawnComponent>>,
) {
    commands.entity(player_query.single().unwrap()).despawn();
}
