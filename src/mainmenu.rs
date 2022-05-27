use bevy::prelude::*;
use bevy::app::AppExit;
use crate::{AppState};

struct MenuMaterials {
    root: UiColor,
    border: UiColor,
    menu: UiColor,
    button: UiColor,
    button_hovered: UiColor,
    button_pressed: UiColor,
    button_text: Color,
}

struct MainMenuData {
    camera_entity: Entity,
    ui_root: Entity,
}

impl FromWorld for MenuMaterials {
    fn from_world(_: &mut World) -> Self {
        MenuMaterials {
            root: Color::NONE.into(),
            border: Color::rgb(0.65, 0.65, 0.65).into(),
            menu: Color::rgb(0.15, 0.15, 0.15).into(),
            button: Color::rgb(0.15, 0.15, 0.15).into(),
            button_hovered: Color::rgb(0.25, 0.25, 0.25).into(),
            button_pressed: Color::rgb(0.35, 0.75, 0.35).into(),
            button_text: Color::WHITE,
        }
    }
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

fn button(materials: &Res<MenuMaterials>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: materials.button.clone(),
        ..Default::default()
    }
}

fn button_text(asset_server: &Res<AssetServer>, materials: &Res<MenuMaterials>, label: &str) -> TextBundle {
    return TextBundle {
        style: Style {
            margin: Rect::all(Val::Px(10.0)),
            ..Default::default()
        },
        text: Text::with_section(
            label,
            TextStyle {
                font: asset_server.load("fonts/PublicPixel.ttf"),
                font_size: 20.0,
                color: materials.button_text.clone(),
            },
            Default::default(),
        ),
        ..Default::default()
    };
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
                                    parent.spawn_bundle(button_text(&asset_server, &materials, "New Game"));
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

fn button_system(
    materials: Res<MenuMaterials>,
    mut buttons: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut uicolor) in buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => *uicolor = materials.button_pressed.into(),
            Interaction::Hovered => *uicolor = materials.button_hovered.into(),
            Interaction::None => *uicolor = materials.button.into(),
        }
    }
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
                    .set(AppState::Playing)
                    .expect("Could not switch state to Playing"),
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
        app.init_resource::<MenuMaterials>()
            .add_system(button_system)
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu)
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