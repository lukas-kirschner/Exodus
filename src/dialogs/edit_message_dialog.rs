use crate::dialogs::create_new_map_dialog::CreateNewMapDialog;
use crate::dialogs::save_file_dialog::SaveFileDialog;
use crate::dialogs::unsaved_changes_dialog::UnsavedChangesDialog;
use crate::dialogs::UIDialog;
use crate::textures::egui_textures::EguiButtonTextures;
use bevy_egui::egui::Ui;
use libexodus::directories::GameDirectories;

#[derive(Eq, PartialEq)]
enum EditMessageDialogState {
    Typing,
    Done,
}

pub struct EditMessageDialog {
    /// The message content
    message: String,
    /// The message to edit
    message_id: usize,
    /// The current state of the dialog
    state: EditMessageDialogState,
}

impl EditMessageDialog {
    pub fn new(message_id: usize, message: String) -> Self {
        EditMessageDialog {
            message,
            message_id,
            state: EditMessageDialogState::Typing,
        }
    }
    pub fn get_message(&self) -> &str {
        self.message.as_str()
    }
    pub fn get_message_id(&self) -> usize {
        self.message_id
    }
}

impl UIDialog for EditMessageDialog {
    fn dialog_title(&self) -> String {
        t!("map_editor.dialog.edit_message_dialog_title").to_string()
    }

    fn draw(
        &mut self,
        ui: &mut Ui,
        _egui_textures: &EguiButtonTextures,
        _directories: &GameDirectories,
    ) {
        ui.vertical_centered_justified(|ui| {
            ui.add_enabled_ui(self.state == EditMessageDialogState::Typing, |ui| {
                ui.text_edit_multiline(&mut self.message);
                let ok = ui.button(t!("common_buttons.ok"));
                if ok.clicked() {
                    self.state = EditMessageDialogState::Done;
                }
            });
        });
    }

    fn is_done(&self) -> bool {
        self.state == EditMessageDialogState::Done
    }

    fn is_cancelled(&self) -> bool {
        false
    }

    fn as_save_file_dialog(&mut self) -> Option<&mut SaveFileDialog> {
        None
    }

    fn as_unsaved_changes_dialog(&mut self) -> Option<&mut UnsavedChangesDialog> {
        None
    }

    fn as_edit_message_dialog(&mut self) -> Option<&mut EditMessageDialog> {
        Some(self)
    }

    fn as_create_new_map_dialog(&mut self) -> Option<&mut CreateNewMapDialog> {
        None
    }
}
