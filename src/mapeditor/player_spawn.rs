use bevy::prelude::*;
use libexodus::player::Player;
use crate::game::constants::{PLAYER_Z, TILE_SIZE};
use crate::game::tilewrapper::MapWrapper;
use crate::{CurrentPlayerTextureAtlasHandle, TEXTURE_SIZE};

#[derive(Component)]
pub struct PlayerSpawnComponent {
    pub player: Player,
}

pub fn init_player_spawn(
    mut commands: Commands,
    current_texture_atlas: Res<CurrentPlayerTextureAtlasHandle>,
    worldwrapper: ResMut<MapWrapper>,
) {
    // Code Duplication from player.rs - but we need to change things later
    let player: PlayerSpawnComponent = PlayerSpawnComponent { player: Player::new() };
    let (map_player_position_x, map_player_position_y) = worldwrapper.world.player_spawn();
    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(player.player.atlas_index()),
            texture_atlas: current_texture_atlas.handle.clone(),
            transform: Transform {
                translation: Vec3::new(map_player_position_x as f32, map_player_position_y as f32, PLAYER_Z),
                scale: Vec3::splat(TILE_SIZE as f32 / TEXTURE_SIZE as f32),
                ..default()
            },
            ..Default::default()
        })
        .insert(player);
}

pub fn destroy_player_spawn(
    mut commands: Commands,
    player_query: Query<Entity, With<PlayerSpawnComponent>>,
) {
    commands.entity(player_query.single()).despawn_recursive();
}