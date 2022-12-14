use bevy::prelude::*;
use libexodus::player::Player;
use libexodus::tiles::Tile;
use crate::{App, AppState, TilesetManager};
use crate::game::constants::{MAPEDITOR_PREVIEWTILE_AIR_ATLAS_INDEX, MAPEDITOR_PREVIEWTILE_ALPHA, MAPEDITOR_PREVIEWTILE_Z, TILE_SIZE};
use crate::game::tilewrapper::MapWrapper;
use crate::mapeditor::{compute_cursor_position_in_world, SelectedTile};

#[derive(Component)]
pub struct PreviewTile {
    current_tile: Tile,
}

pub struct MapEditorPreviewTilePlugin;

impl Plugin for MapEditorPreviewTilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::MapEditor)
                .with_system(setup_preview_tile)
            )
            .add_system_set(SystemSet::on_exit(AppState::MapEditor)
                .with_system(destroy_preview_tile)
            )
            .add_system_set(SystemSet::on_update(AppState::MapEditor)
                .with_system(update_preview_tile)
            )
        ;
    }
}

fn destroy_preview_tile(
    mut commands: Commands,
    preview_tile_q: Query<(&PreviewTile, Entity)>,
) {
    let (_, ent) = preview_tile_q.single();
    commands.entity(ent).despawn();
}

/// Spawn a WALL PreviewTile at an invisible position
pub fn setup_preview_tile(
    mut commands: Commands,
    current_texture_atlas: Res<TilesetManager>,
) {
    let previewtile: PreviewTile = PreviewTile { current_tile: Tile::WALL };
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: Color::Rgba {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                    alpha: MAPEDITOR_PREVIEWTILE_ALPHA,
                },
                index: Tile::WALL.atlas_index().unwrap(),
                ..default()
            },
            texture_atlas: current_texture_atlas.current_handle(),
            transform: Transform {
                translation: Vec3::new(-1 as f32, -1 as f32, MAPEDITOR_PREVIEWTILE_Z),
                scale: Vec3::splat(TILE_SIZE as f32 / current_texture_atlas.current_tileset().texture_size() as f32),
                ..default()
            },
            ..default()
        })
        .insert(previewtile);
}

fn set_preview_tile_texture(
    new_tile: &Tile,
    texture_atlas_handle: &mut Handle<TextureAtlas>,
    texture_atlas_sprite: &mut TextureAtlasSprite,
    preview_tile: &mut PreviewTile,
    current_texture_atlas: &TilesetManager,
) {
    match *new_tile {
        Tile::PLAYERSPAWN => {
            *texture_atlas_handle = current_texture_atlas.current_handle();
            texture_atlas_sprite.index = Player::new().atlas_index();
        }
        _ => {
            if let Some(atlas_index) = new_tile.atlas_index() {
                *texture_atlas_handle = current_texture_atlas.current_handle();
                texture_atlas_sprite.index = atlas_index;
            } else {
                *texture_atlas_handle = current_texture_atlas.current_handle();
                texture_atlas_sprite.index = MAPEDITOR_PREVIEWTILE_AIR_ATLAS_INDEX;
            }
        }
    }
    preview_tile.current_tile = new_tile.clone();
}

/// System to show a transparent preview tile on the map
fn update_preview_tile(
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    map: Res<MapWrapper>,
    current_tile: Res<SelectedTile>,
    mut preview_tile_q: Query<(&mut PreviewTile, &mut Handle<TextureAtlas>, &mut TextureAtlasSprite, &mut Transform)>,
    current_texture_atlas: Res<TilesetManager>,
) {
    let (mut preview_tile, mut texture_atlas_handle, mut texture_atlas_sprite, mut transform) = preview_tile_q.single_mut();
    if current_tile.tile != preview_tile.current_tile {
        set_preview_tile_texture(&current_tile.tile, &mut texture_atlas_handle, &mut texture_atlas_sprite, &mut preview_tile, &*current_texture_atlas);
    }
    let (camera, camera_transform) = q_camera.single(); // Will crash if there is more than one camera
    if let Some((world_x, world_y)) = compute_cursor_position_in_world(&*wnds, camera, camera_transform, &*map) {
        // The cursor is inside the window
        if transform.translation.x as i32 != world_x || transform.translation.y as i32 != world_y {
            transform.translation.x = world_x as f32;
            transform.translation.y = world_y as f32;
        }
        // eprintln!("World coords: {}/{}", world_x, world_y);
    } else {
        // The cursor is not in the window. We need to move the preview out of sight
        transform.translation.x = -10000.0;
        transform.translation.y = -10000.0;
    }
}