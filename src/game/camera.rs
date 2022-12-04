use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use crate::game::constants::TILE_SIZE;
use crate::game::tilewrapper::MapWrapper;
use crate::{TEXTURE_SIZE, UiSizeChangedEvent};
use crate::uicontrols::WindowUiOverlayInfo;


pub fn handle_ui_resize(
    mut event: EventReader<UiSizeChangedEvent>,
    window: Res<Windows>,
    map: Res<MapWrapper>,
    ui_info: Res<WindowUiOverlayInfo>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    for _ in event.iter() {
        let mut camera_transform = camera_query.single_mut();
        rescale_camera(&window.get_primary().unwrap(), &*map, &mut camera_transform, &*ui_info);
    }
}

pub fn rescale_camera(
    window: &Window,
    map: &MapWrapper,
    mut camera_transform: &mut Transform,
    ui_margins: &WindowUiOverlayInfo,
) {
    // Scale the camera, such that the world exactly fits into the viewport.
    let map_width_px: usize = TEXTURE_SIZE * map.world.width();
    let map_height_px: usize = TEXTURE_SIZE * (map.world.height());
    let window_space_height_pixels: f32 = window.height() - (ui_margins.top + ui_margins.bottom);
    let window_space_width_pixels: f32 = window.width() - (ui_margins.left + ui_margins.right);
    let window_ratio: f32 = window_space_width_pixels / window_space_height_pixels;
    let map_ratio: f32 = map_width_px as f32 / map_height_px as f32;
    let camera_scale = if window_ratio < map_ratio {
        window_space_width_pixels / (map_width_px as f32)
    } else {
        window_space_height_pixels / (map_height_px as f32)
    };
    *camera_transform = Transform::from_scale(Vec3::splat(1. / (camera_scale * TEXTURE_SIZE as f32)));

    // Translate the camera, such that the center of the game board is shifted up or down, according to the UI margins
    // Shift the world to the middle of the screen
    let mut shift_x = ((map.world.width() * TEXTURE_SIZE) as f32 / 2.);
    let mut shift_y = ((map.world.height() * TEXTURE_SIZE) as f32 / 2.);
    // Shift the UI down to match the viewport with UI margins
    shift_x = shift_x + (ui_margins.left - ui_margins.right) / 2.;
    shift_y = shift_y + (ui_margins.top - ui_margins.bottom) / 2.;
    // Convert pixels to world coordinates
    shift_x /= TEXTURE_SIZE as f32;
    shift_y /= TEXTURE_SIZE as f32;
    // We need to subtract 0.5 to take account for the fact that tiles are placed in the middle of each coordinate instead of the corner
    shift_x = shift_x - 0.5;
    shift_y = shift_y - 0.5;

    camera_transform.translation = Vec3::new(shift_x, shift_y, 0.);
}

pub fn setup_camera(
    mut commands: Commands,
    window: Res<Windows>,
    map: Res<MapWrapper>,
) {
    let mut camera = Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.0,
            scaling_mode: ScalingMode::WindowSize,
            ..default()
        }.into(),
        transform: Transform::default(),
        ..default()
    };
    let new_size = WindowUiOverlayInfo {
        top: TILE_SIZE,
        bottom: TILE_SIZE,
        ..default()
    };
    commands.insert_resource::<WindowUiOverlayInfo>(new_size.clone());
    rescale_camera(&window.get_primary().unwrap(), &map, &mut camera.transform, &new_size);
    commands.spawn(camera);
}

pub fn destroy_camera(
    mut commands: Commands,
    q_camera: Query<(&Camera, Entity)>,
) {
    let (_, entity) = q_camera.single();
    commands.entity(entity).despawn_recursive();
}
