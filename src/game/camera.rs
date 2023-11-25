use crate::game::constants::RENDER_PLANE_Z;
use crate::game::tilewrapper::MapWrapper;
use crate::ui::uicontrols::WindowUiOverlayInfo;
use crate::ui::UiSizeChangedEvent;
use crate::{GameConfig, TilesetManager, LAYER_ID};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::view::RenderLayers;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct LayerCamera;

#[derive(Component)]
pub struct LayerImage;

pub fn handle_ui_resize(
    mut event: EventReader<UiSizeChangedEvent>,
    window: Query<&Window, With<PrimaryWindow>>,
    map: Res<MapWrapper>,
    ui_info: Res<WindowUiOverlayInfo>,
    mut main_camera_query: Query<
        (&mut Transform, &mut OrthographicProjection),
        (With<MainCamera>, Without<LayerCamera>),
    >,
    mut layer_camera_query: Query<&mut Transform, (With<LayerCamera>, Without<MainCamera>)>,
    tileset: Res<TilesetManager>,
) {
    let Ok(primary) = window.get_single() else {
        return;
    };
    for _ in event.read() {
        let (mut main_camera_transform, mut main_camera_projection) =
            main_camera_query.single_mut();
        let mut layer_camera_transform = layer_camera_query.single_mut();
        rescale_main_camera(
            primary,
            &map,
            &mut layer_camera_transform,
            &mut main_camera_transform,
            &mut main_camera_projection,
            &ui_info,
            tileset.current_tileset().texture_size() as f32,
        );
    }
}

/// Rescale the Main Camera and translate the layer camera,
/// such that the game world fits exactly into the viewport
pub fn rescale_main_camera(
    window: &Window,
    map: &MapWrapper,
    layer_camera_transform: &mut Transform,
    main_camera_transform: &mut Transform,
    main_camera_projection: &mut OrthographicProjection,
    ui_margins: &WindowUiOverlayInfo,
    texture_size: f32,
) {
    // Scale the camera, such that the world exactly fits into the viewport.
    let map_width_px: f32 = texture_size * (map.world.width() as f32);
    let map_height_px: f32 = texture_size * (map.world.height() as f32);
    let viewport_height_pixels: f32 = window.height() - (ui_margins.top + ui_margins.bottom);
    let viewport_width_pixels: f32 = window.width() - (ui_margins.left + ui_margins.right);
    let viewport_ratio: f32 = viewport_width_pixels / viewport_height_pixels;
    let map_ratio: f32 = map_width_px / map_height_px;
    let camera_scale = if viewport_ratio < map_ratio {
        viewport_width_pixels / (map_width_px)
    } else {
        viewport_height_pixels / (map_height_px)
    };
    main_camera_projection.scale = 1. / (camera_scale * texture_size);

    // Translate the layer camera, such that the world is centered on screen.
    // This should cause the world to be rendered perfectly centered on the render layer.
    // Shift the world to the middle of the screen
    let mut shift_x = map_width_px / 2.;
    let mut shift_y = map_height_px / 2.;
    // We need to subtract 0.5 to take account for the fact that tiles are placed in the middle of each coordinate instead of the corner
    shift_x -= 0.5 * texture_size;
    shift_y -= 0.5 * texture_size;

    layer_camera_transform.translation = Vec3::new(shift_x, shift_y, 0.);

    // Shift the main camera by the UI margin sizes to fit the world into the viewport

    main_camera_transform.translation = Vec3::new(
        ((ui_margins.right - ui_margins.left) * main_camera_projection.scale) / 2.,
        ((ui_margins.top - ui_margins.bottom) * main_camera_projection.scale) / 2.,
        0.,
    )
}

pub fn setup_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    config: Res<GameConfig>,
) {
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
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);
    let image_handle = images.add(image);
    let mut layer_camera = Camera2dBundle::default();
    layer_camera.camera.target = RenderTarget::Image(image_handle.clone());
    layer_camera.camera.order = -1;

    let main_camera = Camera2dBundle::default();
    let layer = RenderLayers::layer(LAYER_ID);
    commands.insert_resource::<WindowUiOverlayInfo>(WindowUiOverlayInfo::default());
    commands.spawn((main_camera, MainCamera));
    commands.spawn((layer_camera, LayerCamera, layer));
    commands.spawn((
        SpriteBundle {
            texture: image_handle,
            transform: Transform {
                // Rescale the world, such that 1 world unit = 1 tile
                scale: Vec3::splat(1. / (config.texture_size())),
                translation: Vec3::new(0., 0., RENDER_PLANE_Z),
                ..default()
            },
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

/// Compute the given Viewport Coordinates to Bevy World Coordinates
/// (in Pixels of the world that is rendered onto the layer camera).
///
/// This must transform the given coordinates twice (once for transformation from the viewport
/// to the viewport of the render plane and once for transformation of the rendered viewport
/// to the actual world).
pub fn compute_viewport_to_world(
    screen_pos: Vec2,
    main_camera: &Camera,
    main_camera_transform: &GlobalTransform,
    _layer_camera: &Camera,
    layer_camera_transform: &GlobalTransform,
    texture_size: f32,
) -> Option<(f32, f32)> {
    if let Some(world_coord) = main_camera.viewport_to_world(main_camera_transform, screen_pos) {
        let mut ret = ((world_coord.origin.x), (world_coord.origin.y));
        ret.0 += layer_camera_transform.translation().x / texture_size;
        ret.1 += layer_camera_transform.translation().y / texture_size;
        return Some((ret.0 + 0.5, ret.1 + 0.5));
    }
    None
}
/// Convert the given Bevy World Coordinates into Viewport Coordinates,
/// considering both render layers.
pub fn compute_world_to_viewport(
    world_position: &Vec3,
    main_camera: &Camera,
    main_camera_transform: &GlobalTransform,
    _layer_camera: &Camera,
    layer_camera_transform: &GlobalTransform,
    texture_size: f32,
) -> Option<Vec2> {
    let main_x = ((world_position.x / texture_size) - 0.5)
        - (layer_camera_transform.translation().x / texture_size);
    let main_y = ((world_position.y / texture_size) - 0.5)
        - (layer_camera_transform.translation().y / texture_size);
    if let Some(screen_player) = main_camera.world_to_viewport(
        main_camera_transform,
        (main_x, main_y, world_position.z).into(),
    ) {
        return Some(screen_player);
    }
    None
}
