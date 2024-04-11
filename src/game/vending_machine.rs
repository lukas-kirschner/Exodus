use crate::game::constants::MENU_SQUARE_BUTTON_SIZE;
use crate::game::player::{PlayerComponent, ReturnTo};
use crate::game::scoreboard::{GameOverEvent, Scoreboard};
use crate::game::tilewrapper::MapWrapper;
use crate::mapeditor::SelectedTile;
use crate::textures::egui_textures::EguiButtonTextures;
use crate::ui::uicontrols::WindowUiOverlayInfo;
use crate::ui::{UiSizeChangedEvent, VENDINGMACHINEWIDTH};
use crate::{AppState, GameConfig, GameDirectoriesWrapper};
use bevy::prelude::*;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::WidgetText;
use bevy_egui::{egui, EguiContexts};
use libexodus::tiles::UITiles;
use std::borrow::Cow;
use std::cmp::max;
use std::sync::Arc;

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
        _ => panic!("Unsupported number of vending machine items!"),
    }
}
fn vending_machine_key_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut scoreboard: ResMut<Scoreboard>,
    items: Res<VendingMachineItems>,
    mut commands: Commands,
) {
    for (i, item) in items.items.iter().enumerate() {
        let keycode = index_to_keycode(i + 1);
        if keyboard_input.just_pressed(keycode) {
            if scoreboard.crystals >= item.cost() {
                item.purchase(&mut commands, &mut scoreboard);
                click_close_button(&mut commands);
            }
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
    if let Some(_) = reader.read().next() {
        reader.clear();
        debug!("A vending machine has been triggered!");
        commands.insert_resource(HasVendingMachine);
    }
}

fn vending_machine_triggered_event_clearer(mut events: ResMut<Events<VendingMachineTriggered>>) {
    events.clear();
}

fn vending_machine_ui(
    mut commands: Commands,
    mut egui_ctx: EguiContexts,
    mut selected_tile: ResMut<SelectedTile>,
    egui_textures: Res<EguiButtonTextures>,
    mut state: ResMut<NextState<AppState>>,
    mut worldwrapper: ResMut<MapWrapper>,
    mut scoreboard: ResMut<Scoreboard>,
    items: Res<VendingMachineItems>,
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
            for (i, item) in items.items.iter().enumerate() {
                let response = vending_machine_button(
                    ui,
                    item.button_text(i + 1),
                    item.button_tooltip(i + 1),
                    item.cost(),
                    scoreboard.crystals,
                );
                if response.clicked() {
                    item.purchase(&mut commands, &mut scoreboard);
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
) -> egui::Response {
    ui.add_enabled_ui(cost <= balance, |ui| {
        ui.add_sized(
            [VENDINGMACHINEWIDTH, MENU_SQUARE_BUTTON_SIZE],
            egui::Button::new(text),
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
    fn purchase(&self, commands: &mut Commands, scoreboard: &mut Scoreboard);
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

    fn purchase(&self, _commands: &mut Commands, scoreboard: &mut Scoreboard) {
        scoreboard.crystals = max(0i32, scoreboard.crystals as i32 - COST_KEY as i32) as usize;
        scoreboard.keys = scoreboard.keys + 1; //TODO Animation
    }
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
            "game_ui.vending_machine.button_purchase_five_coins",
            key = index,
            cost = self.cost()
        )
    }

    fn purchase(&self, _commands: &mut Commands, scoreboard: &mut Scoreboard) {
        scoreboard.crystals = max(0i32, scoreboard.crystals as i32 - COST_COINS as i32) as usize;
        scoreboard.coins = scoreboard.coins + 5; //TODO Animation
    }
}
