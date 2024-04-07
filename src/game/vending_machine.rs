use crate::game::player::ReturnTo;
use crate::game::scoreboard::GameOverEvent;
use crate::game::tilewrapper::MapWrapper;
use crate::mapeditor::SelectedTile;
use crate::textures::egui_textures::EguiButtonTextures;
use crate::ui::uicontrols::WindowUiOverlayInfo;
use crate::ui::{UiSizeChangedEvent, VENDINGMACHINEWIDTH};
use crate::{AppState, GameDirectoriesWrapper};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

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
            let close_response = ui.button(t!("game_ui.vending_machine.button_close"));
            if close_response.clicked() {
                commands.remove_resource::<HasVendingMachine>();
            }
        });
}
