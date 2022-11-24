use bevy::prelude::*;
use bevy::render::camera::{DepthCalculation, ScalingMode};
use crate::game::constants::TILE_SIZE;
use crate::game::tilewrapper::MapWrapper;
use crate::TEXTURE_SIZE;

pub type UIMargins = (usize, usize, usize, usize);

pub fn rescale_camera(
    window: &WindowDescriptor,
    map: &MapWrapper,
    mut camera_transform: &mut Transform,
    ui_margins: &UIMargins,
) {
    // Scale the camera, such that the world exactly fits into the viewport. At the top and bottom,
    // we leave at least one world tile of space free for UI elements, which we also scale
    // exactly to the height of one tile.
    let (left, top, right, bottom) = ui_margins;
    let map_width_pixels_plus_ui: usize = TEXTURE_SIZE * map.world.width() + left + right;
    let map_height_pixels_plus_ui: usize = TEXTURE_SIZE * (map.world.height()) + top + bottom; // 2 tiles for UI elements
    let window_height_pixels: usize = window.height as usize;
    let window_width_pixels: usize = window.width as usize;
    let window_ratio: f32 = window_width_pixels as f32 / window_height_pixels as f32;
    let map_ratio: f32 = map_width_pixels_plus_ui as f32 / map_height_pixels_plus_ui as f32;
    let camera_scale = if window_ratio < map_ratio {
        window_width_pixels as f32 / map_width_pixels_plus_ui as f32
    } else {
        window_height_pixels as f32 / map_height_pixels_plus_ui as f32
    };
    *camera_transform = Transform::from_scale(Vec3::splat(1. / (camera_scale * TEXTURE_SIZE as f32)));
    camera_transform.translation = Vec3::new((map.world.width() as f32 / 2.) - 0.5, (map.world.height() as f32 / 2.) - 0.5, 0.);
}

pub fn setup_camera(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    map: Res<MapWrapper>,
) {
    let mut camera = Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.0,
            depth_calculation: DepthCalculation::ZDifference,
            scaling_mode: ScalingMode::WindowSize,
            ..default()
        }.into(),
        transform: Transform::default(),
        ..default()
    };
    // We need to subtract 0.5 to account for the fact that tiles are placed in the middle of each coordinate
    rescale_camera(&window, &map, &mut camera.transform, &(0, TILE_SIZE as usize, 0, TILE_SIZE as usize));
    commands.spawn_bundle(camera);
}

pub fn destroy_camera(
    mut commands: Commands,
    q_camera: Query<(&Camera, Entity)>,
) {
    let (_, entity) = q_camera.single();
    commands.entity(entity).despawn_recursive();
}
