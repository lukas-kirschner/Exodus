use crate::dialogs::edit_message_dialog::EditMessageDialog;
use crate::dialogs::save_file_dialog::SaveFileDialog;
use crate::dialogs::unsaved_changes_dialog::UnsavedChangesDialog;
use crate::dialogs::UIDialog;
use crate::textures::egui_textures::EguiButtonTextures;
use crate::ui::UIPANELCBWIDTH;
use bevy::log::{debug, warn};
use bevy_egui::egui;
use bevy_egui::egui::{Align, Layout, TextBuffer, Ui};
use libexodus::config::Language;
use libexodus::directories::{GameDirectories, InvalidMapNameError};
use libexodus::tiles::Tile;
use libexodus::tilesets::Tileset;
use libexodus::world::GameWorld;
use libexodus::worldgeneration::{WorldGenerationKind, WorldSize};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;

#[derive(Eq, PartialEq, Default)]
enum CreateNewMapDialogState {
    #[default]
    Choosing,
    Done,
    Error,
    Cancelled,
    GeneratingMap,
}

#[derive(Default)]
pub struct CreateNewMapDialog {
    /// The Map Size
    size: WorldSize,
    /// The name of the map
    map_kind: WorldGenerationKind,
    /// The state of the dialog
    state: CreateNewMapDialogState,
    /// The temporary preview
    preview: Option<GameWorld>,
    /// The map seed
    seed: u32,
    /// The error message, if there was any error
    error_text: Option<String>,
}

fn SizeToString(size: &WorldSize) -> Cow<str> {
    match size {
        WorldSize::Classic5mx => t!("map_selection_screen.dialog.create_new_map_dialog_size_5mx"),
        WorldSize::Small => t!("map_selection_screen.dialog.create_new_map_dialog_size_small"),
        WorldSize::Medium => t!("map_selection_screen.dialog.create_new_map_dialog_size_medium"),
        WorldSize::Large => t!("map_selection_screen.dialog.create_new_map_dialog_size_large"),
        WorldSize::Huge => t!("map_selection_screen.dialog.create_new_map_dialog_size_huge"),
        WorldSize::Custom { .. } => {
            t!("map_selection_screen.dialog.create_new_map_dialog_size_custom")
        },
    }
}

impl CreateNewMapDialog {
    /// Get the width of the new map
    pub fn get_width(&self) -> u32 {
        self.size.width()
    }
    /// Get the height of the new map
    pub fn get_height(&self) -> u32 {
        self.size.height()
    }
    /// Generate the map and return it, moving it out of this dialog.
    /// If the map has not been generated yet, this will return None and trigger a map generation.
    /// In this case, this method must be called again to obtain the generated map once it is generated.
    pub fn generate_map(&mut self) -> Option<GameWorld> {
        let ret = self.preview.take();
        match ret {
            None => {
                self.trigger_generation();
                None
            },
            Some(map) => Some(map),
        }
    }

    /// Trigger a generation
    fn trigger_generation(&mut self) {
        self.state = CreateNewMapDialogState::GeneratingMap;
        //TODO
    }
}

impl UIDialog for CreateNewMapDialog {
    fn dialog_title(&self) -> String {
        t!("map_selection_screen.dialog.create_new_map_dialog_title").to_string()
    }

    fn draw(
        &mut self,
        ui: &mut Ui,
        _egui_textures: &EguiButtonTextures, // TODO include Save Button Icon etc.
        directories: &GameDirectories,
    ) {
        ui.vertical_centered_justified(|ui| {
            ui.add_enabled_ui(self.state == CreateNewMapDialogState::Choosing, |ui| {
                // Map Size
                ui.scope(|ui| {
                    ui.horizontal(|ui| {
                        ui.scope(|ui| {
                            ui.set_width(UIPANELCBWIDTH);
                            let selected_width_kind = SizeToString(&self.size);
                            egui::ComboBox::from_id_source("size_box").width(UIPANELCBWIDTH).selected_text(selected_width_kind).show_ui(ui, |ui| {
                                for size in WorldSize::iter() {
                                    ui.selectable_value(
                                        &mut self.size,
                                        size,
                                        SizeToString(&size),
                                    );
                                }
                            }).response.on_hover_text(t!("map_selection_screen.dialog.create_new_map_dialog_size_tooltip"));
                        });
                    });
                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                        ui.add_enabled_ui(matches!(self.size,WorldSize::Custom {..}), |ui| {
                            let (mut fix_width,mut fix_height) = (self.size.width(), self.size.height());
                            let (mut width, mut height) = match self.size {
                                WorldSize::Custom { ref mut width, ref mut height } => (width, height),
                                _ => (&mut fix_width,&mut fix_height)
                            };
                            ui.add_sized((UIPANELCBWIDTH / 2. - ui.style().spacing.item_spacing.x / 2.,0.), egui::DragValue::new(width).clamp_range(1..=64).speed(0.1))
                                .on_hover_text(t!("map_selection_screen.dialog.create_new_map_dialog_width_edit"))
                                .on_disabled_hover_text(t!("map_selection_screen.dialog.create_new_map_dialog_width_noedit"));
                            ui.add_sized((UIPANELCBWIDTH / 2.- ui.style().spacing.item_spacing.x / 2.,0.), egui::DragValue::new(height).clamp_range(1..=32).speed(0.1))
                                .on_hover_text(t!("map_selection_screen.dialog.create_new_map_dialog_height_edit"))
                                .on_disabled_hover_text(t!("map_selection_screen.dialog.create_new_map_dialog_height_noedit"));
                        });
                    });
                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                        ui.add_enabled_ui(self.preview.is_some(), |ui| {
                            let res = ui.button(t!("map_selection_screen.dialog.create_new_map_dialog_accept")).on_hover_text(t!("map_selection_screen.dialog.create_new_map_dialog_accept_tooltip")).on_disabled_hover_text(t!("map_selection_screen.dialog.create_new_map_dialog_cant_accept_tooltip"));
                            if res.clicked() {
                                self.trigger_generation();
                            }
                        });
                        let res = ui.button(t!("map_selection_screen.dialog.create_new_map_dialog_generate")).on_hover_text(t!("map_selection_screen.dialog.create_new_map_dialog_generate_tooltip"));
                        if res.clicked() {
                            self.trigger_generation();
                        }
                        let res = ui.button(t!("common_buttons.cancel")).on_hover_text(t!("map_selection_screen.dialog.create_new_map_dialog_cancel_tooltip"));
                        if res.clicked() {
                            self.state = CreateNewMapDialogState::Cancelled;
                        }
                    });
                });
            });
        });
    }

    fn is_done(&self) -> bool {
        self.state == CreateNewMapDialogState::Done
    }

    fn is_cancelled(&self) -> bool {
        self.state == CreateNewMapDialogState::Cancelled
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
        Some(self)
    }
}
