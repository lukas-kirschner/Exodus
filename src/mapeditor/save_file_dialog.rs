use std::borrow::BorrowMut;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use bevy_egui::egui;
use bevy_egui::egui::{Align, Ui};
use libexodus::directories::GameDirectories;
use libexodus::world::GameWorld;
use crate::mapeditor::mapeditor_ui::EguiButtonTextures;
use crate::{GameDirectoriesWrapper, World};

pub trait UIDialog {
    fn dialog_title(&self) -> &str;
    fn draw(&mut self,
            ui: &mut Ui,
            egui_textures: &EguiButtonTextures,
            directories: &GameDirectories,
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
    /// The file name that is shown to the user
    file_name: String,
    map_title: String,
    map_author: String,
    uuid: String,
    world_to_save: GameWorld,
    state: SaveFileDialogState,
    /// The finalized file path that is created as soon as the user presses the Save button
    file_path: Option<PathBuf>,

}

impl SaveFileDialog {
    /// Instantiate a new SaveFileDialog from the given world
    pub fn new(world: &GameWorld, filename: Option<&Path>, mapname: &str, mapauthor: &str, uuid: &str) -> Self {
        SaveFileDialog {
            file_name: String::from(filename.map(|p| { p.file_name().unwrap_or(OsStr::new("")) }).unwrap_or(OsStr::new("")).to_str().unwrap_or("")),
            map_title: String::from(mapname),
            map_author: String::from(mapauthor),
            uuid: String::from(uuid),
            world_to_save: world.clone(),
            state: SaveFileDialogState::CHOOSING,
            file_path: None,
        }
    }
    /// Resolve the file name and return the full path
    pub fn get_filename(&self) -> Option<PathBuf> {
        self.file_path.clone()
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
            directories: &GameDirectories,
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
                            let map_dir = directories.path_from_userinput(self.file_name.as_str());
                            println!("{:?}", map_dir);
                            if let Ok(path) = map_dir {
                                self.file_path = Some(path);
                            }
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