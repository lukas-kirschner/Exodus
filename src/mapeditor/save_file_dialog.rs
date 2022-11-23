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
    fn as_save_file_dialog(&mut self) -> Option<&mut SaveFileDialog>;
}

#[derive(Eq, PartialEq)]
enum SaveFileDialogState {
    CHOOSING,
    OVERWRITE,
    DONE,
}

pub struct SaveFileDialog {
    file_name: String,
    map_title: String,
    map_author: String,
    uuid: String,
    world_to_save: GameWorld,
    state: SaveFileDialogState,

}

impl SaveFileDialog {
    /// Instantiate a new SaveFileDialog from the given world
    pub fn new(world: &GameWorld, filename: &str, mapname: &str, mapauthor: &str, uuid: &str) -> Self {
        SaveFileDialog {
            file_name: String::from(filename),
            map_title: String::from(mapname),
            map_author: String::from(mapauthor),
            uuid: String::from(uuid),
            world_to_save: world.clone(),
            state: SaveFileDialogState::CHOOSING,
        }
    }

    pub fn get_filename(&self) -> &str {
        self.file_name.as_str()
    }
    pub fn get_map_title(&self) -> &str {
        self.map_title.as_str()
    }
    pub fn get_map_author(&self) -> &str {
        self.map_author.as_str()
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
            ui.add_enabled_ui(self.state == SaveFileDialogState::CHOOSING, |ui| {
                // File Name and Save Button
                ui.scope(|ui| {
                    ui.horizontal(|ui| {
                        ui.scope(|ui| {
                            ui.set_max_width(300.);// TODO hard-coded numbers
                            ui.centered_and_justified(|ui| {
                                ui.text_edit_singleline(&mut self.file_name).on_hover_text("Type a file name here.");
                            });
                        });
                        let save = ui.button("Save");
                        if save.clicked() {
                            //TODO Save map
                            self.state = SaveFileDialogState::DONE;
                        }
                    });
                });
                // Map Properties
                ui.separator();
                ui.scope(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Map Name:");
                        ui.text_edit_singleline(&mut self.map_title).on_hover_text("Type a map title here")
                    });
                });
                ui.scope(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Map Author:");
                        ui.text_edit_singleline(&mut self.map_author).on_hover_text("Type an author name here")
                    });
                });
                ui.scope(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Map UUID:");
                        ui.label(self.uuid.as_str());
                    });
                });
            });
        });
    }

    fn is_done(&self) -> bool {
        self.state == SaveFileDialogState::DONE
    }

    fn as_save_file_dialog(&mut self) -> Option<&mut SaveFileDialog> {
        Some(self)
    }
}