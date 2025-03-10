use crate::game::camera::{LayerCamera, MainCamera};
use crate::game::constants::{
    MAPEDITOR_PREVIEWTILE_AIR_ATLAS_INDEX, MAPEDITOR_PREVIEWTILE_ALPHA, MAPEDITOR_PREVIEWTILE_Z,
};
use crate::game::tilewrapper::MapWrapper;
use crate::mapeditor::{compute_cursor_position_in_world, SelectedTile};
use crate::{App, AppState, GameConfig, TilesetManager, LAYER_ID};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::PrimaryWindow;
use libexodus::player::Player;
use libexodus::tiles::Tile;

#[derive(Component)]
pub struct PreviewTile {
    current_tile: Tile,
}

pub struct MapEditorPreviewTilePlugin;

impl Plugin for MapEditorPreviewTilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MapEditor), setup_preview_tile)
            .add_systems(OnExit(AppState::MapEditor), destroy_preview_tile)
            .add_systems(
                Update,
                update_preview_tile.run_if(in_state(AppState::MapEditor)),
            );
    }
}

fn destroy_preview_tile(mut commands: Commands, preview_tile_q: Query<Entity, With<PreviewTile>>) {
    let ent = preview_tile_q.single();
    commands.entity(ent).despawn();
}

/// Spawn a WALL PreviewTile at an invisible position
pub fn setup_preview_tile(mut commands: Commands, current_texture_atlas: Res<TilesetManager>) {
    let previewtile: PreviewTile = PreviewTile {
        current_tile: Tile::WALL,
    };
    let layer = RenderLayers::layer(LAYER_ID);
    let mut sprite = Sprite::from_atlas_image(
        current_texture_atlas.current_texture_handle(),
        TextureAtlas {
            layout: current_texture_atlas.current_atlas_handle(),
            index: previewtile.current_tile.atlas_index().unwrap(),
        },
    );
    sprite.color = Color::Srgba(Srgba {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
        alpha: MAPEDITOR_PREVIEWTILE_ALPHA,
    });
    commands.spawn((
        sprite,
        Transform::from_translation(Vec3::new(-1f32, -1f32, MAPEDITOR_PREVIEWTILE_Z)),
        previewtile,
        layer,
    ));
}

fn set_preview_tile_texture(
    new_tile: &Tile,
    sprite: &mut Sprite,
    preview_tile: &mut PreviewTile,
    current_texture_atlas: &TilesetManager,
) {
    if let Some(ref mut a) = sprite.texture_atlas {
        match *new_tile {
            Tile::PLAYERSPAWN => {
                a.layout = current_texture_atlas.current_atlas_handle();
                a.index = Player::new().atlas_index();
            },
            _ => {
                if let Some(atlas_index) = new_tile.atlas_index() {
                    a.layout = current_texture_atlas.current_atlas_handle();
                    a.index = atlas_index;
                } else {
                    a.layout = current_texture_atlas.current_atlas_handle();
                    a.index = MAPEDITOR_PREVIEWTILE_AIR_ATLAS_INDEX;
                }
            },
        }
    }
    sprite.image = current_texture_atlas.current_texture_handle();
    preview_tile.current_tile = new_tile.clone();
}

/// System to show a transparent preview tile on the map
fn update_preview_tile(
    wnds: Query<&Window, With<PrimaryWindow>>,
    q_layer_camera: Query<(&Camera, &GlobalTransform), With<LayerCamera>>,
    q_main_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    map: Res<MapWrapper>,
    current_tile: Res<SelectedTile>,
    mut preview_tile_q: Query<(&mut PreviewTile, &mut Sprite, &mut Transform)>,
    current_texture_atlas: Res<TilesetManager>,
    config: Res<GameConfig>,
) {
    let (mut preview_tile, mut sprite, mut transform) = preview_tile_q.single_mut();
    if current_tile.tile != preview_tile.current_tile {
        set_preview_tile_texture(
            &current_tile.tile,
            &mut sprite,
            preview_tile.as_mut(),
            &current_texture_atlas,
        );
    }
    let (layer_camera, layer_camera_transform) = q_layer_camera.single();
    let (main_camera, main_camera_transform) = q_main_camera.single();
    if let Some((world_x_coord, world_y_coord)) = compute_cursor_position_in_world(
        &wnds,
        main_camera,
        main_camera_transform,
        layer_camera,
        layer_camera_transform,
        config.texture_size(),
    ) {
        // The cursor is inside the window
        if world_x_coord >= 0
            && world_y_coord >= 0
            && world_x_coord < map.world.width() as i32
            && world_y_coord < map.world.height() as i32
        {
            transform.translation.x = world_x_coord as f32 * config.texture_size();
            transform.translation.y = world_y_coord as f32 * config.texture_size();
        } else {
            // The cursor is not in the window. We need to move the preview out of sight
            transform.translation.x = -10000.0;
            transform.translation.y = -10000.0;
        }
    } else {
        // The cursor is not in the window. We need to move the preview out of sight
        transform.translation.x = -10000.0;
        transform.translation.y = -10000.0;
    }
}
