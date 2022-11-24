use bevy_egui::egui::Ui;
use libexodus::directories::GameDirectories;
use crate::dialogs::save_file_dialog::SaveFileDialog;
use crate::dialogs::UIDialog;
use crate::egui_textures::EguiButtonTextures;

#[derive(Eq, PartialEq)]
enum UnsavedChangesDialogState {
    CHOOSING,
    YES,
    NO,
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
            state: UnsavedChangesDialogState::CHOOSING,
        }
    }
}

impl UIDialog for UnsavedChangesDialog {
    fn dialog_title(&self) -> &str {
        "There are unsaved changes!"
    }

    fn draw(&mut self,
            ui: &mut Ui,
            _egui_textures: &EguiButtonTextures,
            _: &GameDirectories,
    ) {
        ui.vertical_centered(|ui| {
            ui.label(self.message.as_str());
            ui.scope(|ui| {
                ui.horizontal_top(|ui| {
                    let yes_btn = ui.button("Yes");
                    let no_btn = ui.button("No");
                    if yes_btn.clicked() {
                        self.state = UnsavedChangesDialogState::YES;
                    }
                    if no_btn.clicked() {
                        self.state = UnsavedChangesDialogState::NO;
                    }
                })
            });
        });
    }

    fn is_done(&self) -> bool {
        self.state == UnsavedChangesDialogState::YES
    }

    fn is_cancelled(&self) -> bool {
        self.state == UnsavedChangesDialogState::NO
    }

    fn as_save_file_dialog(&mut self) -> Option<&mut SaveFileDialog> {
        None
    }

    fn as_unsaved_changes_dialog(&mut self) -> Option<&mut UnsavedChangesDialog> {
        Some(self)
    }
}