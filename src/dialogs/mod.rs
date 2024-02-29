use crate::dialogs::edit_message_dialog::EditMessageDialog;
use crate::dialogs::save_file_dialog::SaveFileDialog;
use crate::dialogs::unsaved_changes_dialog::UnsavedChangesDialog;
use crate::textures::egui_textures::EguiButtonTextures;
use bevy::prelude::Resource;
use bevy_egui::egui::Ui;
use libexodus::directories::GameDirectories;

pub mod edit_message_dialog;
pub mod save_file_dialog;
pub mod unsaved_changes_dialog;

pub trait UIDialog {
    fn dialog_title(&self) -> String;
    fn draw(
        &mut self,
        ui: &mut Ui,
        egui_textures: &EguiButtonTextures,
        directories: &GameDirectories,
    );
    fn is_done(&self) -> bool;
    fn is_cancelled(&self) -> bool;
    fn as_save_file_dialog(&mut self) -> Option<&mut SaveFileDialog>;
    fn as_unsaved_changes_dialog(&mut self) -> Option<&mut UnsavedChangesDialog>;
    fn as_edit_message_dialog(&mut self) -> Option<&mut EditMessageDialog>;
}
#[derive(Resource)]
pub struct DialogResource {
    pub ui_dialog: Box<dyn UIDialog + Send + Sync>,
}
