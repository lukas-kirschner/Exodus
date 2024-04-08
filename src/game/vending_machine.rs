use crate::game::constants::MENU_SQUARE_BUTTON_SIZE;
use crate::game::player::ReturnTo;
use crate::game::scoreboard::{GameOverEvent, Scoreboard};
use crate::game::tilewrapper::MapWrapper;
use crate::mapeditor::SelectedTile;
use crate::textures::egui_textures::EguiButtonTextures;
use crate::ui::uicontrols::WindowUiOverlayInfo;
use crate::ui::{UiSizeChangedEvent, VENDINGMACHINEWIDTH};
use crate::{AppState, GameDirectoriesWrapper};
use bevy::prelude::*;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::WidgetText;
use bevy_egui::{egui, EguiContexts};
use libexodus::tiles::UITiles;
use std::cmp::max;

const COST_COINS: usize = 1;
const COST_KEY: usize = 3;

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
            );
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
            let coins_response = vending_machine_button(
                ui,
                t!(
                    "game_ui.vending_machine.button_purchase_five_coins",
                    key = "1"
                ),
                t!(
                    "game_ui.vending_machine.button_purchase_five_coins_tooltip",
                    key = "1"
                ),
                COST_COINS,
                scoreboard.crystals,
            );
            if coins_response.clicked() {
                click_coins_button(&mut commands, &mut scoreboard);
            }
            let key_response = vending_machine_button(
                ui,
                t!("game_ui.vending_machine.button_purchase_key", key = "2"),
                t!(
                    "game_ui.vending_machine.button_purchase_key_tooltip",
                    key = "2"
                ),
                COST_KEY,
                scoreboard.crystals,
            );
            if key_response.clicked() {
                click_key_button(&mut commands, &mut scoreboard);
            }
            let exit_response = vending_machine_button(
                ui,
                t!("game_ui.vending_machine.button_close", key = "2"),
                t!("game_ui.vending_machine.button_close_tooltip", key = "2"),
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
fn click_coins_button(commands: &mut Commands, scoreboard: &mut Scoreboard) {
    scoreboard.crystals = max(0i32, scoreboard.crystals as i32 - COST_COINS as i32) as usize;
    scoreboard.coins = scoreboard.coins + 5; //TODO Animation
    click_close_button(commands);
}
fn click_key_button(commands: &mut Commands, scoreboard: &mut Scoreboard) {
    scoreboard.crystals = max(0i32, scoreboard.crystals as i32 - COST_KEY as i32) as usize;
    scoreboard.keys = scoreboard.keys + 1; //TODO Animation
    click_close_button(commands);
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
