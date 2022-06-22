use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use libexodus::world::{GameWorld, presets};
use crate::{AppState, GameDirectoriesWrapper};
use crate::uicontrols::{button, button_text, DELETE_TEXT, EDIT_TEXT, full_screen_menu_root_node, menu_esc_control, MenuMaterials, NAVBAR_BACK_TEXT, navbar_button_container, NAVBAR_HEIGHT, PLAY_TEXT, top_menu_container};
use crate::game::tilewrapper::MapWrapper;

struct MapSelectionScreenData {
    camera_entity: Entity,
    ui_root: Entity,
}

/// The possible Map Selection Screen buttons, which are embedded in the List View component
#[derive(Component)]
enum MapSelectionScreenButton {
    /// Play a map
    Play { map_uuid: String },
    /// Edit a map
    Edit { map_uuid: String },
    /// Delete a map
    Delete { map_uuid: String, entity_id: Entity },
    /// Back to Main Menu
    Back,
}

struct Maps {
    maps: Vec<MapWrapper>,
}

impl FromWorld for Maps {
    fn from_world(world: &mut World) -> Self {
        Maps {
            maps: vec![MapWrapper::from_world(world)],
        }
    }
}

/// This list is inspired from the official Bevy tutorial example at https://bevyengine.org/examples/ui/ui/
#[derive(Component, Default)]
struct MapSelectionList {
    position: f32,
}

fn listview_container(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            ..Default::default()
        },
        color: materials.root.clone(),
        ..Default::default()
    }
}

/// Create the ListView Overflow Node
fn listview_list_overflownode(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::ColumnReverse,
            align_self: AlignSelf::Center,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            overflow: Overflow::Hidden,
            ..default()
        },
        color: materials.root.clone(),
        ..default()
    }
}

/// Create the moving node of the ListView
fn listview_list_movingnode(_materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::ColumnReverse,
            flex_grow: 1.0,
            max_size: Size::new(Val::Percent(100.0), Val::Undefined),
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    }
}

/// Initialize a new map list button
pub fn map_list_button(materials: &Res<MenuMaterials>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(NAVBAR_HEIGHT), Val::Px(NAVBAR_HEIGHT)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: materials.button.clone(),
        ..Default::default()
    }
}

fn spawn_list_item(
    asset_server: &Res<AssetServer>,
    materials: &Res<MenuMaterials>,
    parent: &mut ChildBuilder,
    map_name: &str,
    map_author: &str,
    map_uuid: String) {
    let mut rootnode = parent

        // Panel that contains all the contents for one row in the ListView
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_shrink: 0.,
                size: Size::new(Val::Percent(100.0), Val::Px(NAVBAR_HEIGHT)),
                flex_direction: FlexDirection::RowReverse,
                margin: Rect {
                    left: Val::Auto,
                    right: Val::Auto,
                    ..default()
                },
                ..default()
            },
            color: materials.menu.into(),
            ..default()
        });
    let row_id = rootnode.id();

    rootnode
        // Play Button
        .with_children(|parent| {
            parent
                .spawn_bundle(map_list_button(materials))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(button_text(asset_server, materials, PLAY_TEXT))
                    ;
                })
                .insert(MapSelectionScreenButton::Play { map_uuid: map_uuid.clone() })
            ;
        })

        // Edit Button
        .with_children(|parent| {
            parent
                .spawn_bundle(map_list_button(materials))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(button_text(asset_server, materials, EDIT_TEXT))
                    ;
                })
                .insert(MapSelectionScreenButton::Edit { map_uuid: map_uuid.clone() })
            ;
        })

        // Delete Button
        .with_children(|parent| {
            parent
                .spawn_bundle(map_list_button(materials))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(button_text(asset_server, materials, DELETE_TEXT))
                    ;
                })
                .insert(MapSelectionScreenButton::Delete { map_uuid: map_uuid.clone(), entity_id: row_id })
            ;
        })

        // Empty Panel for alignment
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(NAVBAR_HEIGHT)),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
            ;
        })

        // Map Name
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        max_size: Size::new(Val::Auto, Val::Px(NAVBAR_HEIGHT)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::FlexStart,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(10.0)),
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::FlexStart,
                                ..Default::default()
                            },
                            text:
                            Text {
                                sections: vec![
                                    TextSection {
                                        value: map_name.into(),
                                        style: TextStyle {
                                            font: asset_server.load("fonts/PublicPixel.ttf"),
                                            font_size: 20.0,
                                            color: materials.button_text.clone(),
                                        },
                                    },
                                    TextSection {
                                        value: "  ".into(),
                                        style: TextStyle {
                                            font: asset_server.load("fonts/PublicPixel.ttf"),
                                            font_size: 20.0,
                                            color: materials.button_text.clone(),
                                        },
                                    },
                                    TextSection {
                                        value: if map_author.is_empty() { "" } else { "Author: " }.into(),
                                        style: TextStyle {
                                            font: asset_server.load("fonts/PublicPixel.ttf"),
                                            font_size: 10.0,
                                            color: materials.button_text.clone(),
                                        },
                                    },
                                    TextSection {
                                        value: map_author.into(),
                                        style: TextStyle {
                                            font: asset_server.load("fonts/PublicPixel.ttf"),
                                            font_size: 10.0,
                                            color: materials.button_text.clone(),
                                        },
                                    },
                                ],
                                ..Default::default()
                            }
                            ,
                            ..Default::default()
                        })
                    ;
                })
            ;
        })

    ;
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: Res<MenuMaterials>,
    maps: Res<Maps>,
) {
    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();

    let ui_root = commands
        .spawn_bundle(full_screen_menu_root_node(&materials))
        // NAVBAR
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
                                .insert(MapSelectionScreenButton::Back);
                        })
                    ;
                })
            ;
        })

        // LISTVIEW
        .with_children(|parent| {
            parent
                .spawn_bundle(listview_container(&materials))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(listview_list_overflownode(&materials))
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(listview_list_movingnode(&materials))
                                .insert(MapSelectionList::default())
                                .with_children(|parent| {
                                    for map in &maps.maps {
                                        // Spawn all maps that are in the resource. The resource MUST be initialized beforehand
                                        spawn_list_item(&asset_server, &materials, parent, map.name(), map.author(), map.uuid());
                                    }
                                })
                            ;
                        })
                    ;
                })
            ;
        }).id();

    commands.insert_resource(
        MapSelectionScreenData {
            camera_entity,
            ui_root,
        });
}

/// Load all maps from the Map Directory. This might take a while, depending on how many maps there are in the maps folder
fn load_maps(
    mut maps: ResMut<Maps>,
    directories: Res<GameDirectoriesWrapper>,
) {
    // Delete all maps
    maps.maps = Vec::new();

    // Load all maps from the game's map directory and all subdirectories
    directories.game_directories.iter_maps()
        .for_each(|map_file| {
            if let Ok(map) = GameWorld::load_from_file(map_file.path())
                .map_err(|err| eprintln!("Could not load map file at {}! Error: {}", map_file.path().to_str().unwrap_or("<Invalid Path>"), err))
                .map(|map| {
                    println!("Successfully loaded map file {}", map_file.path().to_str().unwrap_or("<Invalid Path>"));
                    map
                }) {
                maps.maps.push(MapWrapper {
                    world: map,
                })
            }
        });

    //If we are in debug mode, insert the debug map exampleworld()!
    if cfg!(debug_assertions) {
        maps.maps.push(MapWrapper {
            world: GameWorld::exampleworld(),
        });
        // Fill the list to test scrolling
        for i in 1..=20 {
            let mut map = presets::map_with_border(24 + i, i + 3);
            map.set_name(format!("Empty {}x{} world", 24 + i, i + 3).as_str());
            maps.maps.push(MapWrapper {
                world: map,
            })
        }
    }
}

/// Button Click System
fn button_press_system(
    mut commands: Commands,
    buttons: Query<(&Interaction, &MapSelectionScreenButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
    mut maps: ResMut<Maps>,
    directories: Res<GameDirectoriesWrapper>,
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MapSelectionScreenButton::Back => state
                    .set(AppState::MainMenu)
                    .expect("Could not switch back to Main Menu"),
                MapSelectionScreenButton::Play { map_uuid } => {
                    //load map and move it out of the maps vector
                    let map_ind = maps.maps.iter().position(|mw| mw.uuid().eq(&*map_uuid)).expect(&*format!("The map with UUID {} was not found in maps.", map_uuid));
                    let mapwrapper = maps.maps.remove(map_ind);
                    commands.insert_resource(mapwrapper);

                    state.set(AppState::Playing)
                        .expect("Could not start game");
                }
                MapSelectionScreenButton::Delete { map_uuid, entity_id } => {
                    //TODO Delete Map
                    //Delete the ListView Item from the ListView:
                    commands.entity(*entity_id).despawn_recursive();
                }
                MapSelectionScreenButton::Edit { map_uuid } => {
                    let map_ind = maps.maps.iter().position(|mw| mw.uuid().eq(&*map_uuid)).expect(&*format!("The map with UUID {} was not found in maps.", map_uuid));
                    maps.maps[map_ind].world.save_to_file(directories.game_directories.path_from_mapname(maps.maps[map_ind].world.get_name()).unwrap().as_path()).unwrap(); // TODO
                }
            };
        }
    }
}

/// Scroll the map list with the mouse wheel.
/// Taken from https://bevyengine.org/examples/ui/ui/
fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut MapSelectionList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in query_list.iter_mut() {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size.y)
                .sum();
            let panel_height = uinode.size.y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}

/// Cleanup the Map Selection Screen
fn cleanup(mut commands: Commands, menu_data: Res<MapSelectionScreenData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();
}

pub struct MapSelectionScreenPlugin;

impl Plugin for MapSelectionScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Maps>()
            .add_system_set(
                SystemSet::on_enter(AppState::MapSelectionScreen)
                    .with_system(load_maps).label("load_maps"),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::MapSelectionScreen)
                    .with_system(setup).after("load_maps"),
            )
            .add_system_set(SystemSet::on_exit(AppState::MapSelectionScreen)
                .with_system(cleanup)
            )
            .add_system_set(SystemSet::on_update(AppState::MapSelectionScreen)
                .with_system(button_press_system)
            )
            .add_system_set(SystemSet::on_update(AppState::MapSelectionScreen)
                .with_system(mouse_scroll)
            )
            .add_system_set(SystemSet::on_update(AppState::MapSelectionScreen)
                .with_system(menu_esc_control)
            )
        ;
    }
}