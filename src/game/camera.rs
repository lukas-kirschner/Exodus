use crate::game::tilewrapper::MapWrapper;
use crate::ui::uicontrols::WindowUiOverlayInfo;
use crate::ui::UiSizeChangedEvent;
use crate::{TilesetManager, LAYER_ID};
use bevy::prelude::*;
use bevy::render::camera::{RenderTarget, ScalingMode};
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::view::RenderLayers;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct LayerCamera;

#[derive(Component)]
pub struct LayerImage;

pub fn handle_ui_resize(
    mut event: EventReader<UiSizeChangedEvent>,
    window: Res<Windows>,
    map: Res<MapWrapper>,
    ui_info: Res<WindowUiOverlayInfo>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<LayerCamera>>,
    tileset: Res<TilesetManager>,
) {
    for _ in event.iter() {
        let (mut camera_transform, mut camera_projection) = camera_query.single_mut();
        rescale_camera(
            window.get_primary().unwrap(),
            &map,
            &mut camera_transform,
            &mut camera_projection,
            &ui_info,
            tileset.current_tileset().texture_size(),
        );
    }
}

pub fn rescale_camera(
    window: &Window,
    map: &MapWrapper,
    mut camera_transform: &mut Transform,
    mut camera_projection: &mut OrthographicProjection,
    ui_margins: &WindowUiOverlayInfo,
    texture_size: usize,
) {
    // Scale the camera, such that the world exactly fits into the viewport.
    let map_width_px: usize = texture_size * map.world.width();
    let map_height_px: usize = texture_size * (map.world.height());
    let window_space_height_pixels: f32 = window.height() - (ui_margins.top + ui_margins.bottom);
    let window_space_width_pixels: f32 = window.width() - (ui_margins.left + ui_margins.right);
    let window_ratio: f32 = window_space_width_pixels / window_space_height_pixels;
    let map_ratio: f32 = map_width_px as f32 / map_height_px as f32;
    let camera_scale = if window_ratio < map_ratio {
        window_space_width_pixels / (map_width_px as f32)
    } else {
        window_space_height_pixels / (map_height_px as f32)
    };
    camera_projection.scale = 1. / (camera_scale * texture_size as f32);

    // Translate the camera, such that the center of the game board is shifted up or down, according to the UI margins
    // Shift the world to the middle of the screen
    let mut shift_x = (map.world.width() * texture_size) as f32 / 2.;
    let mut shift_y = (map.world.height() * texture_size) as f32 / 2.;
    // Shift the UI down to match the viewport with UI margins
    shift_x += (ui_margins.left - ui_margins.right) / 2.;
    shift_y += (ui_margins.top - ui_margins.bottom) / 2.;
    // Convert pixels to world coordinates
    shift_x /= texture_size as f32;
    shift_y /= texture_size as f32;
    // We need to subtract 0.5 to take account for the fact that tiles are placed in the middle of each coordinate instead of the corner
    shift_x -= 0.5;
    shift_y -= 0.5;

    camera_transform.translation = Vec3::new(shift_x, shift_y, 0.);
}

pub fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let size = Extent3d {
        width: 1920,
        height: 1080,
        ..default()
    };
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            // view_formats: &[],
        },
        ..default()
    };
    image.resize(size);
    let image_handle = images.add(image);
    let mut layer_camera = Camera2dBundle::new_with_far(1000.);
    layer_camera.camera.target = RenderTarget::Image(image_handle.clone());
    layer_camera.camera.priority = -1;

    let main_camera = Camera2dBundle::new_with_far(1000.);
    let layer = RenderLayers::layer(LAYER_ID);
    commands.insert_resource::<WindowUiOverlayInfo>(WindowUiOverlayInfo::default());
    commands.spawn((main_camera, MainCamera));
    commands.spawn((layer_camera, LayerCamera, layer));
    commands.spawn((
        SpriteBundle {
            texture: image_handle,
            ..default()
        },
        LayerImage,
    ));
}

pub fn destroy_camera(
    mut commands: Commands,
    q_layer_camera: Query<Entity, With<MainCamera>>,
    q_main_camera: Query<Entity, With<LayerCamera>>,
    q_layer_image: Query<Entity, With<LayerImage>>,
) {
    let main_camera_entity = q_main_camera.single();
    commands.entity(main_camera_entity).despawn_recursive();
    let layer_camera_entity = q_layer_camera.single();
    commands.entity(layer_camera_entity).despawn_recursive();
    let q_layer_image_entity = q_layer_image.single();
    commands.entity(q_layer_image_entity).despawn_recursive();
}
