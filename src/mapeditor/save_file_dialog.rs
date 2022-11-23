use std::borrow::BorrowMut;
use bevy_egui::egui;
use bevy_egui::egui::{Align, Ui};
use libexodus::world::GameWorld;
use crate::mapeditor::mapeditor_ui::EguiButtonTextures;
use crate::World;

pub trait UIDialog {
    fn dialog_title(&self) -> &str;
    fn draw(&mut self,
            ui: &mut Ui,
            egui_textures: &EguiButtonTextures,
    );
    fn is_done(&self) -> bool;
}

#[derive(Eq, PartialEq)]
enum SaveFileDialogState {
    CHOOSING,
    OVERWRITE,
    DONE,
}

pub struct SaveFileDialog {
    file_name: String,
    world_to_save: GameWorld,
    state: SaveFileDialogState,

}

impl SaveFileDialog {
    /// Instantiate a new SaveFileDialog from the given world
    pub fn new(world: &GameWorld) -> Self {
        SaveFileDialog {
            file_name: Default::default(),
            world_to_save: world.clone(),
            state: SaveFileDialogState::CHOOSING,
        }
    }
}

impl UIDialog for SaveFileDialog {
    fn dialog_title(&self) -> &str {
        "Edit Properties and Save Map"
    }

    fn draw(&mut self,
            ui: &mut Ui,
            egui_textures: &EguiButtonTextures,
    ) {
        ui.vertical_centered_justified(|ui| {
            // File Name and Save Button
            ui.scope(|ui| {
                ui.with_layout(egui::Layout::right_to_left(Align::TOP), |ui| {
                    ui.add_enabled_ui(self.state == SaveFileDialogState::CHOOSING, |ui| {
                        let save = ui.button("Save");
                        if save.clicked() {
                            todo!()
                        }
                        ui.horizontal_centered(|ui| {
                            ui.text_edit_singleline(&mut self.file_name).on_hover_text("Type a file name here.");
                        });
                    });
                })
            });
            // Map Properties
            ui.separator();
            ui.label("Map Name:");
            ui.label("Map Author:");
            ui.label("Map UUID:");
        });
    }

    fn is_done(&self) -> bool {
        self.state == SaveFileDialogState::DONE
    }
}