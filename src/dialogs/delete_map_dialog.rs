use crate::dialogs::create_new_map_dialog::CreateNewMapDialog;
use crate::dialogs::edit_message_dialog::EditMessageDialog;
use crate::dialogs::save_file_dialog::SaveFileDialog;
use crate::dialogs::unsaved_changes_dialog::UnsavedChangesDialog;
use crate::dialogs::UIDialog;
use crate::game::scoreboard::egui_highscore_label;
use crate::game::tilewrapper::MapWrapper;
use crate::textures::egui_textures::EguiButtonTextures;
use bevy::prelude::Commands;
use bevy_egui::egui;
use bevy_egui::egui::{RichText, Ui};
use libexodus::directories::GameDirectories;

#[derive(Eq, PartialEq)]
enum DeleteMapDialogState {
    Choosing,
    Yes,
    Cancelled,
}

pub struct DeleteMapDialog {
    /// The message that is shown to the user
    map: MapWrapper,
    state: DeleteMapDialogState,
}

impl DeleteMapDialog {
    /// Instantiate a new SaveFileDialog from the given world
    pub fn new(map: MapWrapper) -> Self {
        DeleteMapDialog {
            map,
            state: DeleteMapDialogState::Choosing,
        }
    }
}

impl UIDialog for DeleteMapDialog {
    fn dialog_title(&self) -> String {
        t!("map_selection_screen.dialog.delete_map_dialog_title").to_string()
    }

    fn draw(
        &mut self,
        ui: &mut Ui,
        egui_textures: &EguiButtonTextures,
        _: &GameDirectories,
        _commands: &mut Commands,
    ) {
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new(t!("map_selection_screen.dialog.delete_map_dialog_question"))
                    .text_style(egui::TextStyle::Name("DialogText".into())),
            );
            ui.separator();
            crate::ui::mapselectionscreen::labels_name_author(ui, &self.map.world);
            egui_highscore_label(ui, &self.map.previous_best, &egui_textures);
            ui.separator();
            ui.label(
                RichText::new(t!("map_selection_screen.dialog.delete_map_dialog_hint"))
                    .text_style(egui::TextStyle::Name("DialogText".into())),
            );
            ui.scope(|ui| {
                ui.horizontal_top(|ui| {
                    let yes_btn = ui.button(t!("common_buttons.yes"));
                    let cancel_btn = ui.button(t!("common_buttons.cancel"));
                    if yes_btn.clicked() {
                        self.state = DeleteMapDialogState::Yes;
                    }
                    if cancel_btn.clicked() {
                        self.state = DeleteMapDialogState::Cancelled;
                    }
                })
            });
        });
    }

    fn is_done(&self) -> bool {
        self.state == DeleteMapDialogState::Yes
    }

    fn is_cancelled(&self) -> bool {
        self.state == DeleteMapDialogState::Cancelled
    }

    fn as_save_file_dialog(&mut self) -> Option<&mut SaveFileDialog> {
        None
    }

    fn as_unsaved_changes_dialog(&mut self) -> Option<&mut UnsavedChangesDialog> {
        None
    }

    fn as_edit_message_dialog(&mut self) -> Option<&mut EditMessageDialog> {
        None
    }

    fn as_create_new_map_dialog(&mut self) -> Option<&mut CreateNewMapDialog> {
        None
    }

    fn as_delete_map_dialog(&mut self) -> Option<&mut DeleteMapDialog> {
        Some(self)
    }
}
