use crate::animation::animated_action_sprite::{AnimatedActionSprite, AnimatedSpriteAction};
use crate::game::constants::{
    MENU_SQUARE_BUTTON_SIZE, PICKUP_ITEM_ASCEND_SPEED, PICKUP_ITEM_DECAY_SPEED,
    PICKUP_ITEM_ZOOM_SPEED, PLAYER_Z,
};
use crate::game::player::PlayerComponent;
use crate::game::scoreboard::Scoreboard;
use crate::textures::egui_textures::EguiButtonTextures;
use crate::textures::tileset_manager::TilesetManager;
use crate::ui::VENDINGMACHINEWIDTH;
use crate::{AppState, LAYER_ID};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{RichText, WidgetText};
use bevy_egui::{egui, EguiContexts};
use egui::Label;
use libexodus::tiles::Tile;
use std::borrow::Cow;
use std::cmp::max;

const COST_COINS: usize = 1;
const COST_KEY: usize = 3;

#[derive(Resource)]
struct VendingMachineItems {
    items: Vec<Box<dyn VendingMachineItem>>,
}

#[derive(Event)]
pub struct VendingMachineTriggered;
#[derive(Resource)]
struct HasVendingMachine;

pub struct VendingMachinePlugin;
impl Plugin for VendingMachinePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<VendingMachineTriggered>()
            .add_systems(
                Update,
                vending_machine_triggered_event_listener.run_if(in_state(AppState::Playing)),
            )
            .add_systems(
                OnEnter(AppState::Playing),
                vending_machine_triggered_event_clearer,
            )
            .add_systems(
                Update,
                vending_machine_ui.run_if(
                    in_state(AppState::Playing).and_then(resource_exists::<HasVendingMachine>),
                ),
            )
            .add_systems(
                Update,
                vending_machine_key_handler.run_if(
                    in_state(AppState::Playing).and_then(resource_exists::<HasVendingMachine>),
                ),
            )
            .insert_resource(VendingMachineItems {
                items: vec![Box::new(CoinsItem), Box::new(KeysItem)],
            });
    }
}
fn index_to_keycode(index: usize) -> KeyCode {
    match index {
        1 => KeyCode::Digit1,
        2 => KeyCode::Digit2,
        3 => KeyCode::Digit3,
        4 => KeyCode::Digit4,
        5 => KeyCode::Digit5,
        6 => KeyCode::Digit6,
        7 => KeyCode::Digit7,
        8 => KeyCode::Digit8,
        9 => KeyCode::Digit9,
        _ => panic!("Unsupported number of vending machine items: {}!", index),
    }
}
fn vending_machine_key_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut scoreboard: ResMut<Scoreboard>,
    items: Res<VendingMachineItems>,
    mut commands: Commands,
    player_positions: Query<&Transform, With<PlayerComponent>>,
    atlas_handle: Res<TilesetManager>,
) {
    let player_pos = player_positions.single();
    for (i, item) in items.items.iter().enumerate() {
        let keycode = index_to_keycode(i + 1);
        if keyboard_input.just_pressed(keycode) && scoreboard.crystals >= item.cost() {
            item.purchase(
                &mut commands,
                &mut scoreboard,
                &atlas_handle,
                (player_pos.translation.x, player_pos.translation.y),
            );
            click_close_button(&mut commands);
        } else if keyboard_input.just_pressed(KeyCode::ArrowDown)
            || keyboard_input.just_pressed(KeyCode::ArrowUp)
            || keyboard_input.just_pressed(KeyCode::ArrowLeft)
            || keyboard_input.just_pressed(KeyCode::ArrowRight)
            || keyboard_input.just_pressed(KeyCode::KeyQ)
            || keyboard_input.just_pressed(KeyCode::KeyW)
        {
            // Close the interface if any other key has been pressed
            click_close_button(&mut commands);
        }
    }
    let exit_key = index_to_keycode(items.items.len() + 1);
    if keyboard_input.just_pressed(exit_key) {
        click_close_button(&mut commands);
    }
}
fn vending_machine_triggered_event_listener(
    mut reader: EventReader<VendingMachineTriggered>,
    mut commands: Commands,
) {
    if reader.read().next().is_some() {
        reader.clear();
        // debug!("A vending machine has been triggered!");
        commands.insert_resource(HasVendingMachine);
    }
}

fn vending_machine_triggered_event_clearer(mut events: ResMut<Events<VendingMachineTriggered>>) {
    events.clear();
}

fn vending_machine_ui(
    mut commands: Commands,
    mut egui_ctx: EguiContexts,
    mut scoreboard: ResMut<Scoreboard>,
    items: Res<VendingMachineItems>,
    player_positions: Query<&Transform, With<PlayerComponent>>,
    atlas_handle: Res<TilesetManager>,
    egui_textures: Res<EguiButtonTextures>,
) {
    // TODO Idea: If the player is at the left of the map center, put the window on the right.
    // If the player is at the right, put the window left
    // Accomplish this by putting an optional pos into the marker resource
    egui::Window::new(t!("game_ui.vending_machine.dialog_title"))
        .resizable(false)
        .collapsible(false)
        .min_width(VENDINGMACHINEWIDTH)
        .max_width(VENDINGMACHINEWIDTH)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.set_width(VENDINGMACHINEWIDTH);
            ui.add_sized(
                (VENDINGMACHINEWIDTH, 0.0),
                Label::new(
                    RichText::new(t!("game_ui.vending_machine.dialog_greeting"))
                        .text_style(egui::TextStyle::Name("Description".into())),
                )
                .wrap(true),
            );
            for (i, item) in items.items.iter().enumerate() {
                let response = vending_machine_button(
                    ui,
                    item.button_text(i + 1),
                    item.button_tooltip(i + 1),
                    item.cost(),
                    scoreboard.crystals,
                    &egui_textures,
                    Some(Tile::STARCRYSTAL),
                );
                if response.clicked() {
                    let player_pos = player_positions.single();
                    item.purchase(
                        &mut commands,
                        &mut scoreboard,
                        &atlas_handle,
                        (player_pos.translation.x, player_pos.translation.y),
                    );
                    click_close_button(&mut commands);
                }
            }
            let exit_key = items.items.len() + 1;
            let exit_response = vending_machine_button(
                ui,
                t!("game_ui.vending_machine.button_close", key = exit_key),
                t!(
                    "game_ui.vending_machine.button_close_tooltip",
                    key = exit_key
                ),
                0,
                0,
                &egui_textures,
                None,
            );
            if exit_response.clicked() {
                click_close_button(&mut commands);
            }
        });
}
fn click_close_button(commands: &mut Commands) {
    commands.remove_resource::<HasVendingMachine>();
}
/// Create an image button to display in the UI
fn vending_machine_button(
    ui: &mut egui::Ui,
    text: impl Into<WidgetText>,
    tooltip: impl Into<WidgetText> + std::fmt::Display + Clone,
    cost: usize,
    balance: usize,
    egui_textures: &EguiButtonTextures,
    thumbnail_tile: Option<Tile>,
) -> egui::Response {
    ui.add_enabled_ui(cost <= balance, |ui| {
        ui.add_sized(
            [VENDINGMACHINEWIDTH, MENU_SQUARE_BUTTON_SIZE],
            egui::Button::opt_image_and_text(
                thumbnail_tile
                    .map(|tile| {
                        let (id, size, _) = egui_textures
                            .textures
                            .get(&tile.atlas_index().unwrap())
                            .unwrap_or_else(|| {
                                panic!("Textures for {:?} were not loaded as Egui textures!", tile)
                            });
                        SizedTexture::new(*id, *size)
                    })
                    .map(|i| i.into()),
                Some(text.into()),
            )
            .shortcut_text(if cost > 0 {
                t!("game_ui.vending_machine.currency", cost = cost)
            } else {
                Cow::from(" ")
            })
            .wrap(false),
        )
    })
    .inner
    .on_hover_text(tooltip.clone())
    .on_disabled_hover_text(format!(
        "{} ({})",
        tooltip,
        t!("game_ui.vending_machine.not_enough_funds")
    ))
}

trait VendingMachineItem: Sync + Send {
    fn cost(&self) -> usize;
    fn button_text(&self, index: usize) -> Cow<str>;
    fn button_tooltip(&self, index: usize) -> Cow<str>;
    fn purchase(
        &self,
        commands: &mut Commands,
        scoreboard: &mut Scoreboard,
        atlas_handle: &TilesetManager,
        player_pos_px: (f32, f32),
    );
}

struct KeysItem;

impl VendingMachineItem for KeysItem {
    fn cost(&self) -> usize {
        COST_KEY
    }

    fn button_text(&self, index: usize) -> Cow<str> {
        t!(
            "game_ui.vending_machine.button_purchase_key",
            key = index,
            cost = self.cost()
        )
    }

    fn button_tooltip(&self, index: usize) -> Cow<str> {
        t!(
            "game_ui.vending_machine.button_purchase_key_tooltip",
            key = index,
            cost = self.cost()
        )
    }

    fn purchase(
        &self,
        commands: &mut Commands,
        scoreboard: &mut Scoreboard,
        atlas_handle: &TilesetManager,
        player_pos_px: (f32, f32),
    ) {
        scoreboard.crystals = max(0i32, scoreboard.crystals as i32 - COST_KEY as i32) as usize;
        scoreboard.keys += 1;
        // Animate the purchase:
        let mut action = AnimatedActionSprite::from_ascend_and_zoom(
            PICKUP_ITEM_DECAY_SPEED,
            -PICKUP_ITEM_ASCEND_SPEED,
            PICKUP_ITEM_ZOOM_SPEED,
            AnimatedSpriteAction::None,
        );
        action.set_repeat(2, player_pos_px);
        spawn_animation(
            commands,
            atlas_handle,
            player_pos_px,
            action,
            &Tile::STARCRYSTAL,
        );
        spawn_animation(
            commands,
            atlas_handle,
            player_pos_px,
            AnimatedActionSprite::from_ascend_and_zoom(
                PICKUP_ITEM_DECAY_SPEED,
                PICKUP_ITEM_ASCEND_SPEED,
                PICKUP_ITEM_ZOOM_SPEED,
                AnimatedSpriteAction::None,
            ),
            &Tile::KEY,
        );
    }
}

fn spawn_animation(
    commands: &mut Commands,
    atlas_handle: &TilesetManager,
    player_pos_px: (f32, f32),
    animation: AnimatedActionSprite,
    tile: &Tile,
) {
    commands.spawn((
        SpriteSheetBundle {
            sprite: Sprite::default(),
            atlas: TextureAtlas {
                layout: atlas_handle.current_atlas_handle(),
                index: tile.atlas_index().unwrap(),
            },
            texture: atlas_handle.current_texture_handle().clone(),
            transform: Transform {
                translation: (player_pos_px.0, player_pos_px.1, PLAYER_Z - 0.1).into(),
                ..default()
            },
            ..Default::default()
        },
        animation,
        RenderLayers::layer(LAYER_ID),
    ));
}

struct CoinsItem;

impl VendingMachineItem for CoinsItem {
    fn cost(&self) -> usize {
        COST_COINS
    }

    fn button_text(&self, index: usize) -> Cow<str> {
        t!(
            "game_ui.vending_machine.button_purchase_five_coins",
            key = index,
            cost = self.cost()
        )
    }

    fn button_tooltip(&self, index: usize) -> Cow<str> {
        t!(
            "game_ui.vending_machine.button_purchase_five_coins_tooltip",
            key = index,
            cost = self.cost()
        )
    }

    fn purchase(
        &self,
        commands: &mut Commands,
        scoreboard: &mut Scoreboard,
        atlas_handle: &TilesetManager,
        player_pos_px: (f32, f32),
    ) {
        scoreboard.crystals = max(0i32, scoreboard.crystals as i32 - COST_COINS as i32) as usize;
        scoreboard.coins += 5;
        // Animate Star Crystal
        spawn_animation(
            commands,
            atlas_handle,
            player_pos_px,
            AnimatedActionSprite::from_ascend_and_zoom(
                PICKUP_ITEM_DECAY_SPEED,
                -PICKUP_ITEM_ASCEND_SPEED,
                PICKUP_ITEM_ZOOM_SPEED,
                AnimatedSpriteAction::None,
            ),
            &Tile::STARCRYSTAL,
        );
        // Animate five coins in different angles
        for angle in [-60., -30., 0., 30., 60.] {
            spawn_animation(
                commands,
                atlas_handle,
                player_pos_px,
                AnimatedActionSprite::from_ascend_angle_and_zoom(
                    PICKUP_ITEM_DECAY_SPEED,
                    PICKUP_ITEM_ASCEND_SPEED,
                    angle,
                    PICKUP_ITEM_ZOOM_SPEED,
                    AnimatedSpriteAction::None,
                ),
                &Tile::COIN,
            );
        }
    }
}
