use bevy::prelude::*;
use bevy::app::AppExit;
use crate::{AppState};
use crate::uicontrols::{button, button_text, MenuMaterials};

struct MainMenuData {
    camera_entity: Entity,
    ui_root: Entity,
}

fn root(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: materials.root.clone(),
        ..Default::default()
    }
}

fn border(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(400.0), Val::Auto),
            border: Rect::all(Val::Px(8.0)),
            ..Default::default()
        },
        color: materials.border.clone(),
        ..Default::default()
    }
}

fn menu_background(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            padding: Rect::all(Val::Px(5.0)),
            ..Default::default()
        },
        color: materials.menu.clone(),
        ..Default::default()
    }
}

#[derive(Component)]
enum MainMenuButton {
    Play,
    Quit,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: Res<MenuMaterials>,
) {
    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();

    let ui_root = commands
        .spawn_bundle(root(&materials))
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn_bundle(border(&materials))
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn_bundle(menu_background(&materials))
                        .with_children(|parent| {
                            parent.spawn_bundle(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(&asset_server, &materials, "Maps"));
                                })
                                .insert(MainMenuButton::Play);
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