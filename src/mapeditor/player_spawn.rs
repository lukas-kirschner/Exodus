use crate::game::constants::PLAYER_Z;
use crate::game::tilewrapper::MapWrapper;
use crate::{TilesetManager, LAYER_ID};
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
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(player.player.atlas_index()),
            texture_atlas: tileset.current_handle(),
            transform: Transform {
                translation: Vec3::new(
                    (map_player_position_x * tileset.current_tileset.texture_size()) as f32,
                    (map_player_position_y * tileset.current_tileset.texture_size()) as f32,
                    PLAYER_Z,
                ),
                ..default()
            },
            ..Default::default()
        },
        player,
        layer,
    ));
}

pub fn destroy_player_spawn(
    mut commands: Commands,
    player_query: Query<Entity, With<PlayerSpawnComponent>>,
) {
    commands.entity(player_query.single()).despawn_recursive();
}
