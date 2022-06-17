use bevy::prelude::*;
use bevy::app::AppExit;
use crate::{AppState};
use crate::uicontrols::{button, button_text, floating_border, full_screen_menu_root_node, fullscreen_menu_background, MenuMaterials};

struct MainMenuData {
    camera_entity: Entity,
    ui_root: Entity,
}

#[derive(Component)]
enum MainMenuButton {
    Play,
    Credits,
    Quit,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: Res<MenuMaterials>,
) {
    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();

    let ui_root = commands
        .spawn_bundle(full_screen_menu_root_node(&materials))
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn_bundle(floating_border(&materials, 400))
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn_bundle(fullscreen_menu_background(&materials))
                        .with_children(|parent| {
                            parent.spawn_bundle(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(&asset_server, &materials, "Maps"));
                                })
                                .insert(MainMenuButton::Play);
                            parent.spawn_bundle(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(&asset_server, &materials, "Credits"));
                                })
                                .insert(MainMenuButton::Credits);
                            parent.spawn_bundle(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(&asset_server, &materials, "Quit"));
                                })
                                .insert(MainMenuButton::Quit);
                        });
                });
        }).id();
    commands.insert_resource(
        MainMenuData {
            camera_entity,
            ui_root,
        });
}

fn button_press_system(
    buttons: Query<(&Interaction, &MainMenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MainMenuButton::Play => state
                    .set(AppState::MapSelectionScreen)
                    .expect("Could not switch state to Map Selection Screen"),
                MainMenuButton::Credits => state
                    .set(AppState::CreditsScreen)
                    .expect("Could not switch state to Credits Screen"),
                MainMenuButton::Quit => exit.send(AppExit),
            };
        }
    }
}

fn cleanup(mut commands: Commands, menu_data: Res<MainMenuData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();
}

/// The Main Menu Plugin
pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::MainMenu)
                                .with_system(setup),
            )
            .add_system_set(SystemSet::on_update(AppState::MainMenu)
                .with_system(button_press_system)
            )
            .add_system_set(SystemSet::on_exit(AppState::MainMenu)
                .with_system(cleanup)
            )
        ;
    }
}