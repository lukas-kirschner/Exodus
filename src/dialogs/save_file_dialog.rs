use crate::dialogs::unsaved_changes_dialog::UnsavedChangesDialog;
use crate::dialogs::UIDialog;
use crate::textures::egui_textures::EguiButtonTextures;
use bevy::log::{debug, warn};
use bevy_egui::egui::Ui;
use libexodus::directories::{GameDirectories, InvalidMapNameError};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[derive(Eq, PartialEq)]
enum SaveFileDialogState {
    Choosing,
    Overwrite,
    Done,
    Error,
    Cancelled,
}

pub struct SaveFileDialog {
    /// The file name to be saved
    file_name: String,
    /// The name of the map
    map_title: String,
    /// The name of the map author
    map_author: String,
    /// The hash of the map that is shown to the user
    hash: String,
    /// The current state of the dialog
    state: SaveFileDialogState,
    /// The finalized file path that is created as soon as the user presses the Save button
    file_path: Option<PathBuf>,
    /// Error text that is shown when an error occurs
    error_text: String,
}

impl SaveFileDialog {
    /// Instantiate a new SaveFileDialog from the given world
    pub fn new(
        filename: Option<&Path>,
        mapname: &str,
        mapauthor: &str,
        uuid: &str,
        directories: &GameDirectories,
    ) -> Self {
        SaveFileDialog {
            file_name: filename
                .map(|p| directories.relative_map_dir_from_path(p))
                .unwrap_or(Err(InvalidMapNameError::EmptyName))
                .unwrap_or_else(|e| {
                    warn!("Could not resolve map path: {}", e);
                    "".to_string()
                }),
            map_title: String::from(mapname),
            map_author: String::from(mapauthor),
            hash: String::from(uuid),
            state: SaveFileDialogState::Choosing,
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
    fn dialog_title(&self) -> String {
        t!("map_editor.dialog.save_dialog_title")
    }

    fn draw(
        &mut self,
        ui: &mut Ui,
        _egui_textures: &EguiButtonTextures, // TODO include Save Button Icon etc.
        directories: &GameDirectories,
    ) {
        let row_width = 500.; // TODO hard-coded numbers
        ui.vertical_centered_justified(|ui| {
            ui.add_enabled_ui(self.state == SaveFileDialogState::Choosing, |ui| {
                // File Name and Save Button
                ui.scope(|ui| {
                    ui.horizontal(|ui| {
                        ui.scope(|ui| {
                            ui.set_width(row_width);
                            ui.centered_and_justified(|ui| {
                                ui.text_edit_singleline(&mut self.file_name)
                                    .on_hover_text(t!(
                                        "map_editor.dialog.save_dialog_file_name_tooltip"
                                    ));
                            });
                        });
                        let save = ui.button(t!("common_buttons.save"));
                        if save.clicked() {
                            // TODO Support Subfolders like "campaign/map.exm" -> ~/.local/share/.../maps/campaign/map.exm
                            let map_dir = directories.path_from_userinput(self.file_name.as_str());
                            debug!("{:?}", map_dir);
                            match map_dir {
                                Ok(path) => {
                                    self.file_path = Some(path);
                                    self.state = if self.file_path.as_ref().unwrap().exists() {
                                        SaveFileDialogState::Overwrite
                                    } else {
                                        SaveFileDialogState::Done
                                    };
                                },
                                Err(err) => {
                                    self.error_text = err.to_string();
                                    self.state = SaveFileDialogState::Error;
                                },
                            }
                        }
                    });
                });
                // Map Properties
                ui.separator();
                ui.scope(|ui| {
                    ui.set_width(row_width);
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", t!("map_editor.dialog.save_dialog_map_name")));
                        ui.text_edit_singleline(&mut self.map_title)
                            .on_hover_text(t!("map_editor.dialog.save_dialog_map_title_tooltip"))
                    });
                });
                ui.scope(|ui| {
                    ui.set_width(row_width);
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "{}:",
                            t!("map_editor.dialog.save_dialog_map_author")
                        ));
                        ui.text_edit_singleline(&mut self.map_author)
                            .on_hover_text(t!("map_editor.dialog.save_dialog_map_author_tooltip"))
                    });
                });
                ui.scope(|ui| {
                    ui.set_width(row_width);
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", t!("map_editor.dialog.save_dialog_map_hash")));
                        ui.label(self.hash.as_str());
                    });
                });
            });
            ui.scope(|ui| {
                ui.horizontal(|ui| {
                    let res = ui.button(t!("common_buttons.cancel"));
                    if res.clicked() {
                        self.state = SaveFileDialogState::Cancelled;
                    }
                });
            });
            ui.add_visible_ui(
                self.state == SaveFileDialogState::Error
                    || self.state == SaveFileDialogState::Overwrite,
                |ui| {
                    ui.scope(|ui| {
                        ui.horizontal(|ui| {
                            let etext = if self.state == SaveFileDialogState::Error {
                                format!("Error: {}", self.error_text.as_str())
                            } else {
                                t!("map_editor.dialog.save_dialog_overwrite")
                            };
                            ui.label(etext.as_str());
                            if self.state == SaveFileDialogState::Error {
                                let res = ui.button(t!("common_buttons.ok"));
                                if res.clicked() {
                                    self.state = SaveFileDialogState::Choosing;
                                }
                            } else {
                                let res = ui.button(t!("common_buttons.yes"));
                                if res.clicked() {
                                    // We do not save the file here, but rely on the caller to save the file for us.
                                    self.state = SaveFileDialogState::Done;
                                }
                                let res = ui.button(t!("common_buttons.no"));
                                if res.clicked() {
                                    self.state = SaveFileDialogState::Choosing;
                                }
                            }
                        });
                    });
                },
            );
        });
    }

    fn is_done(&self) -> bool {
        self.state == SaveFileDialogState::Done
    }

    fn is_cancelled(&self) -> bool {
        self.state == SaveFileDialogState::Cancelled
    }

    fn as_save_file_dialog(&mut self) -> Option<&mut SaveFileDialog> {
        Some(self)
    }

    fn as_unsaved_changes_dialog(&mut self) -> Option<&mut UnsavedChangesDialog> {
        None
    }
}
