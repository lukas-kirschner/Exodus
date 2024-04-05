use crate::dialogs::edit_message_dialog::EditMessageDialog;
use crate::dialogs::save_file_dialog::SaveFileDialog;
use crate::dialogs::unsaved_changes_dialog::UnsavedChangesDialog;
use crate::dialogs::{DialogResource, UIDialog};
use crate::game::tilewrapper::MapWrapper;
use crate::game::world::{despawn_world, setup_game_world, WorldTile};
use crate::textures::egui_textures::EguiButtonTextures;
use crate::textures::tileset_manager::TilesetManager;
use crate::ui::UIPANELCBWIDTH;
use bevy::ecs::system::{CommandQueue, SystemState};
use bevy::prelude::*;
use bevy::render::MainWorld;
use bevy_egui::egui;
use bevy_egui::egui::{Align, InnerResponse, Layout, Response, RichText, TextBuffer, Ui};
use libexodus::directories::GameDirectories;
use libexodus::tiles::Tile;
use libexodus::world::GameWorld;
use libexodus::worldgeneration::{
    build_generator, WorldGenerationError, WorldGenerationKind, WorldSize,
};
use std::borrow::Cow;
use std::cmp::min;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::thread;
use std::thread::JoinHandle;
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
fn KindToString(size: &WorldGenerationKind) -> Cow<str> {
    match size {
        WorldGenerationKind::Empty => {
            t!("map_selection_screen.dialog.create_new_map_dialog_kind_empty")
        },
        WorldGenerationKind::Border { .. } => {
            t!("map_selection_screen.dialog.create_new_map_dialog_kind_border")
        },
        WorldGenerationKind::Filled { .. } => {
            t!("map_selection_screen.dialog.create_new_map_dialog_kind_filled")
        },
        WorldGenerationKind::Labyrinth { .. } => {
            t!("map_selection_screen.dialog.create_new_map_dialog_kind_labyrinth")
        },
    }
}
#[derive(Resource)]
pub struct CreateMapBackgroundWorkerThread {
    job: JoinHandle<Result<GameWorld, WorldGenerationError>>,
}
/// Query all unfinished jobs and update the map generator dialog, if one job has finished.
/// The generated preview will be displayed in full screen mode behind the map dialog,
/// if the generation has been successful.
/// Shall ONLY be run if a CreateMapBackgroundWorkerThread exists!
pub fn bevy_job_handler(world: &mut World) {
    if matches!(
        world
            .get_resource::<CreateMapBackgroundWorkerThread>()
            .map(|r| r.job.is_finished()),
        Some(true)
    ) {
        if let Some(thread) = world.remove_resource::<CreateMapBackgroundWorkerThread>() {
            let thread_result = thread.job.join(); //How to move?
            match thread_result {
                Ok(map_result) => match map_result {
                    Ok(map) => {
                        // Generate the preview of the new map and emit all tiles onto the game board
                        let mut state: SystemState<(
                            Commands,
                            Res<TilesetManager>,
                            Query<Entity, With<WorldTile>>,
                        )> = SystemState::new(world);
                        let (mut commands, atlas_handle, tiles_query) = state.get_mut(world);
                        generate_preview(&mut commands, &map, atlas_handle, &tiles_query);
                        state.apply(world);
                        // Update the Dialog
                        if let Some(generate_map_dialog) = world
                            .resource_mut::<DialogResource>()
                            .ui_dialog
                            .as_create_new_map_dialog()
                        {
                            generate_map_dialog.state = CreateNewMapDialogState::Choosing;
                            generate_map_dialog.preview = Some(map);
                        } else {
                            error!(
                                "The current dialog resource did not contain a CreateNewMapDialog!"
                            );
                        }
                    },
                    Err(e) => {
                        error!("Error while generating map! {:?}", e);
                    },
                },
                Err(thread_error) => {
                    error!(
                        "Error while joining map generator thread: {:?}",
                        thread_error
                    );
                    return;
                },
            }
        }
    }
}
/// Generate a preview of the given map and show it in the background
fn generate_preview(
    commands: &mut Commands,
    map: &GameWorld,
    atlas_handle: Res<TilesetManager>,
    tiles_query: &Query<Entity, With<WorldTile>>,
) {
    despawn_world(commands, tiles_query);
    setup_game_world(commands, map, &*atlas_handle);
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
    pub fn generate_map(&mut self, mut commands: &mut Commands) -> Option<GameWorld> {
        let ret = self.preview.take();
        match ret {
            None => {
                self.trigger_generation(commands);
                None
            },
            Some(map) => Some(map),
        }
    }

    /// Trigger a generation
    fn trigger_generation(&mut self, commands: &mut Commands) {
        self.state = CreateNewMapDialogState::GeneratingMap;
        let algo = build_generator(self.map_kind.clone(), self.get_width(), self.get_height());
        commands.insert_resource(CreateMapBackgroundWorkerThread {
            job: thread::spawn(move || algo.generate()),
        });
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
        commands: &mut Commands,
    ) {
        ui.vertical_centered_justified(|ui| {
            ui.add_enabled_ui(self.state == CreateNewMapDialogState::Choosing, |ui| {
                // Map Size
                ui.scope(|ui| {
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
                    // Algorithm Selection
                    heading_label(ui,t!("map_selection_screen.dialog.create_new_map_dialog_kind_selector"));
                    ui.scope(|ui| {
                        ui.set_width(UIPANELCBWIDTH);
                        let selected_kind = KindToString(&self.map_kind);
                        egui::ComboBox::from_id_source("kind_box").width(UIPANELCBWIDTH).selected_text(selected_kind).show_ui(ui, |ui| {
                            for kind in WorldGenerationKind::iter() {
                                ui.selectable_value(
                                    &mut self.map_kind,
                                    kind.clone(),
                                    KindToString(&kind),
                                );
                            }
                        }).response.on_hover_text(t!("map_selection_screen.dialog.create_new_map_dialog_kind_tooltip"));
                    });
                    // UI for the individual generation parameters, different for each generation kind
                    ui.separator();
                    ui_for_generation_kind(&mut self.map_kind,ui);
                    ui.separator();
                    // Seed (if applicable)
                    ui.add_enabled_ui(self.map_kind.is_seeded(), |ui| {
                        heading_label(ui,t!("map_selection_screen.dialog.create_new_map_dialog_seed"));
                        ui.add_sized((UIPANELCBWIDTH,0.), egui::DragValue::new(&mut self.seed).clamp_range(u32::MIN..=u32::MAX).speed(0.1))
                            .on_hover_text(t!("map_selection_screen.dialog.create_new_map_dialog_seed_tooltip"))
                            .on_disabled_hover_text(t!("map_selection_screen.dialog.create_new_map_dialog_no_seed_tooltip"));
                        // TODO add randomize button
                    });
                    // Buttons
                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                        ui.add_enabled_ui(self.preview.is_some(), |ui| {
                            let res = ui.button(t!("map_selection_screen.dialog.create_new_map_dialog_accept")).on_hover_text(t!("map_selection_screen.dialog.create_new_map_dialog_accept_tooltip")).on_disabled_hover_text(t!("map_selection_screen.dialog.create_new_map_dialog_cant_accept_tooltip"));
                            if res.clicked() {
                                self.state = CreateNewMapDialogState::Done;
                                // The caller is now responsible for taking the generated map and changing the state
                            }
                        });
                        let res = ui.button(t!("map_selection_screen.dialog.create_new_map_dialog_generate")).on_hover_text(t!("map_selection_screen.dialog.create_new_map_dialog_generate_tooltip"));
                        if res.clicked() {
                            self.trigger_generation(commands);
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

fn ui_for_generation_kind(kind: &mut WorldGenerationKind, ui: &mut Ui) {
    match kind {
        WorldGenerationKind::Empty => {
            // No parameters at all
        },
        WorldGenerationKind::Border {
            ref mut width,
            ref mut color,
        } => {
            one_line_label_and_slider(
                ui,
                t!("map_selection_screen.dialog.create_new_map_dialog_border_width"),
                width,
                0..=32,
                t!("map_selection_screen.dialog.create_new_map_dialog_border_width_tooltip"),
            );
            heading_label(
                ui,
                t!("map_selection_screen.dialog.create_new_map_dialog_border_color"),
            );
            algorithm_color_selector(
                ui,
                color,
                t!("map_selection_screen.dialog.create_new_map_dialog_border_color_tooltip"),
            );
        },
        WorldGenerationKind::Filled { ref mut color } => {
            heading_label(
                ui,
                t!("map_selection_screen.dialog.create_new_map_dialog_filled_color"),
            );
            algorithm_color_selector(
                ui,
                color,
                t!("map_selection_screen.dialog.create_new_map_dialog_filled_color_tooltip"),
            );
        },
        WorldGenerationKind::Labyrinth { ref mut color } => {
            heading_label(
                ui,
                t!("map_selection_screen.dialog.create_new_map_dialog_labyrinth_color"),
            );
            algorithm_color_selector(
                ui,
                color,
                t!("map_selection_screen.dialog.create_new_map_dialog_labyrinth_color_tooltip"),
            );
        },
    }
}

/// Spawn a subheading label
fn heading_label(ui: &mut Ui, translated_string: Cow<str>) -> InnerResponse<Response> {
    ui.scope(|ui| {
        ui.set_width(UIPANELCBWIDTH);
        ui.label(
            RichText::new(format!("{}:", translated_string))
                .text_style(egui::TextStyle::Name("Subheading".into())),
        )
    })
}
/// All selectable colors for the color selection dropdowns
const ALL_COLORS: [Tile; 5] = [
    Tile::WALL,
    Tile::WALLCOBBLE,
    Tile::WALLSMOOTH,
    Tile::WALLNATURE,
    Tile::WALLCHISELED,
];
/// Spawn a Color Selector ComboBox
fn algorithm_color_selector(ui: &mut Ui, color: &mut Tile, tooltip: Cow<str>) {
    ui.scope(|ui| {
        ui.set_width(UIPANELCBWIDTH);
        let selected_kind = TileToString(color);
        egui::ComboBox::from_id_source(tooltip.to_string())
            .width(UIPANELCBWIDTH)
            .selected_text(selected_kind)
            .show_ui(ui, |ui| {
                for tile in ALL_COLORS {
                    ui.selectable_value(color, tile.clone(), TileToString(&tile));
                }
            })
            .response
            .on_hover_text(tooltip);
    });
}
/// Format the given tile using the current locale
fn TileToString(tile: &Tile) -> String {
    t!(format!("tile.{}", tile.str_id()).as_str()).to_string()
}
/// Add one label and one slider, with the label taking up two thirds of the available space
/// and the numeric slider one third
fn one_line_label_and_slider(
    ui: &mut Ui,
    heading: Cow<str>,
    num: &mut u32,
    range: RangeInclusive<u32>,
    tooltip: Cow<str>,
) -> InnerResponse<(Response, Response)> {
    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
        (
            ui.add_sized(
                (
                    UIPANELCBWIDTH / 3. * 2. - ui.style().spacing.item_spacing.x / 2.,
                    0.,
                ),
                egui::Label::new(
                    egui::RichText::new(format!("{}:", heading))
                        .text_style(egui::TextStyle::Name("Subheading".into())),
                ),
            ),
            ui.add_sized(
                (
                    UIPANELCBWIDTH / 3. - ui.style().spacing.item_spacing.x / 2.,
                    0.,
                ),
                egui::DragValue::new(num).clamp_range(range).speed(0.1),
            )
            .on_hover_text(tooltip),
        )
    })
}
