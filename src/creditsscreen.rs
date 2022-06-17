use bevy::prelude::*;
use indoc::formatdoc;
use crate::AppState;
use crate::uicontrols::{button, button_text, floating_border, full_screen_menu_root_node, fullscreen_menu_background, menu_esc_control, MenuMaterials, NAVBAR_BACK_TEXT, navbar_button_container, top_menu_container};

pub struct CreditsScreen;

struct CreditsScreenData {
    camera_entity: Entity,
    ui_root: Entity,
}

#[derive(Component)]
enum CreditsScreenButton {
    Quit,
}

fn credits() -> String {
    formatdoc! {"
        {program_name} Version {version}
        ",
            program_name = env!("CARGO_PKG_NAME"),
            version = env!("CARGO_PKG_VERSION"),
    }
}

fn credits_text(asset_server: &Res<AssetServer>, materials: &Res<MenuMaterials>) -> TextBundle {
    TextBundle {
        style: Style {
            margin: Rect::all(Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        text: Text::with_section(
            credits(),
            TextStyle {
                font: asset_server.load("fonts/PublicPixel.ttf"),
                font_size: 20.0,
                color: materials.button_text.clone(),
            },
            Default::default(),
        ),
        ..Default::default()
    }
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
            parent
                .spawn_bundle(top_menu_container(&materials))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(navbar_button_container(&materials))
                        .with_children(|parent| {
                            parent.spawn_bundle(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(&asset_server, &materials, NAVBAR_BACK_TEXT));
                                })
                                .insert(CreditsScreenButton::Quit);
                        })
                    ;
                })
            ;
        })

        // Credits in border and centered
        .with_children(|parent| {
            parent
                .spawn_bundle(fullscreen_menu_background(&materials))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(floating_border(&materials, 600))
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(fullscreen_menu_background(&materials))
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(credits_text(&asset_server, &materials))
                                    ;
                                })
                            ;
                        })
                    ;
                })
            ;
        })
        .id()
        ;
    commands.insert_resource(
        CreditsScreenData {
            camera_entity,
            ui_root,
        });
}

/// Return to Main Menu Button
fn button_press_system(
    buttons: Query<(&Interaction, &CreditsScreenButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                CreditsScreenButton::Quit => state
                    .set(AppState::MainMenu)
                    .expect("Could not switch back to Main Menu"),
            };
        }
    }
}

fn cleanup(mut commands: Commands, menu_data: Res<CreditsScreenData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();
}

impl Plugin for CreditsScreen {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::CreditsScreen)
                    .with_system(setup),
            )
            .add_system_set(SystemSet::on_update(AppState::CreditsScreen)
                .with_system(button_press_system)
            )
            .add_system_set(SystemSet::on_update(AppState::CreditsScreen)
                .with_system(menu_esc_control)
            )
            .add_system_set(SystemSet::on_exit(AppState::CreditsScreen)
                .with_system(cleanup)
            )
        ;
    }
}