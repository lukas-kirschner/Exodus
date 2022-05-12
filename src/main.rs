use bevy::prelude::*;
use bevy::window::WindowMode;

#[derive(Component)]
struct TileSolid;

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Exodus".to_string(),
            resizable: false,
            width: 1000.,
            height: 500.,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_startup_system(setup_camera)
        .add_plugins(DefaultPlugins)
        .run();
}
