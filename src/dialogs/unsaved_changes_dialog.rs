use crate::dialogs::create_new_map_dialog::CreateNewMapDialog;
use crate::dialogs::delete_map_dialog::DeleteMapDialog;
use crate::dialogs::edit_message_dialog::EditMessageDialog;
use crate::dialogs::save_file_dialog::SaveFileDialog;
use crate::dialogs::UIDialog;
use crate::textures::egui_textures::EguiButtonTextures;
use bevy::prelude::Commands;
use bevy_egui::egui;
use bevy_egui::egui::{RichText, Ui};
use libexodus::directories::GameDirectories;

#[derive(Eq, PartialEq)]
enum UnsavedChangesDialogState {
    Choosing,
    Yes,
    No,
}

pub struct UnsavedChangesDialog {
    /// The message that is shown to the user
    message: String,
    state: UnsavedChangesDialogState,
}

impl UnsavedChangesDialog {
    /// Instantiate a new SaveFileDialog from the given world
    pub fn new(message: &str) -> Self {
        UnsavedChangesDialog {
            message: String::from(message),
            state: UnsavedChangesDialogState::Choosing,
        }
    }
}

impl UIDialog for UnsavedChangesDialog {
    fn dialog_title(&self) -> String {
        t!("map_editor.dialog.unsaved_changes_dialog_title").to_string()
    }

    fn draw(
        &mut self,
        ui: &mut Ui,
        _egui_textures: &EguiButtonTextures,
        _: &GameDirectories,
        _commands: &mut Commands,
    ) {
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new(self.message.as_str())
                    .text_style(egui::TextStyle::Name("DialogText".into())),
            );
            ui.scope(|ui| {
                ui.horizontal_top(|ui| {
                    let yes_btn = ui.button(t!("common_buttons.yes"));
                    let no_btn = ui.button(t!("common_buttons.no"));
                    if yes_btn.clicked() {
                        self.state = UnsavedChangesDialogState::Yes;
                    }
                    if no_btn.clicked() {
                        self.state = UnsavedChangesDialogState::No;
                    }
                })
            });
        });
    }

    fn is_done(&self) -> bool {
        self.state == UnsavedChangesDialogState::Yes
    }

    fn is_cancelled(&self) -> bool {
        self.state == UnsavedChangesDialogState::No
    }

    fn as_save_file_dialog(&mut self) -> Option<&mut SaveFileDialog> {
        None
    }

    fn as_unsaved_changes_dialog(&mut self) -> Option<&mut UnsavedChangesDialog> {
        Some(self)
    }

    fn as_edit_message_dialog(&mut self) -> Option<&mut EditMessageDialog> {
        None
    }

    fn as_create_new_map_dialog(&mut self) -> Option<&mut CreateNewMapDialog> {
        None
    }

    fn as_delete_map_dialog(&mut self) -> Option<&mut DeleteMapDialog> {
        None
    }
}
