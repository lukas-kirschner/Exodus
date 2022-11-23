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
    fn is_cancelled(&self) -> bool;
    fn as_save_file_dialog(&mut self) -> Option<&mut SaveFileDialog>;
}

#[derive(Eq, PartialEq)]
enum SaveFileDialogState {
    CHOOSING,
    OVERWRITE,
    DONE,
    ERROR,
    CANCELLED,
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
    /// Error text that is shown when an error occurs
    error_text: String,

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
            error_text: "".to_string(),
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
                                self.state = if self.file_path.as_ref().unwrap().exists() {
                                    SaveFileDialogState::OVERWRITE
                                } else {
                                    SaveFileDialogState::DONE
                                };
                            } else {
                                self.error_text = map_dir.unwrap_err().to_string();
                                self.state = SaveFileDialogState::ERROR;
                            }
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
            ui.scope(|ui| {
                ui.horizontal(|ui| {
                    let res = ui.button("Cancel");
                    if res.clicked() {
                        self.state = SaveFileDialogState::CANCELLED;
                    }
                });
            });
            ui.add_visible_ui(self.state == SaveFileDialogState::ERROR || self.state == SaveFileDialogState::OVERWRITE, |ui| {
                ui.scope(|ui| {
                    ui.horizontal(|ui| {
                        let etext = if self.state == SaveFileDialogState::ERROR {
                            format!("Error: {}", self.error_text.as_str())
                        } else {
                            "Do you want to overwrite the map?".to_string()
                        };
                        ui.label(etext.as_str());
                        if self.state == SaveFileDialogState::ERROR {
                            let res = ui.button("Ok");
                            if res.clicked() {
                                self.state = SaveFileDialogState::CHOOSING;
                            }
                        } else {
                            let res = ui.button("Yes");
                            if res.clicked() {
                                // We do not save the file here, but rely on the caller to save the file for us.
                                self.state = SaveFileDialogState::DONE;
                            }
                            let res = ui.button("No");
                            if res.clicked() {
                                self.state = SaveFileDialogState::CHOOSING;
                            }
                        }
                    });
                });
            });
        });
    }

    fn is_done(&self) -> bool {
        self.state == SaveFileDialogState::DONE
    }

    fn is_cancelled(&self) -> bool {
        self.state == SaveFileDialogState::CANCELLED
    }

    fn as_save_file_dialog(&mut self) -> Option<&mut SaveFileDialog> {
        Some(self)
    }
}