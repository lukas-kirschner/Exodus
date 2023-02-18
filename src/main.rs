use std::fs;
use std::path::PathBuf;
use bevy::asset::LoadState;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::render::view::{Layer, RenderLayers};
use bevy::window::{WindowMode, WindowResized};
use bevy_egui::{EguiContext, EguiPlugin};
use libexodus::config::Config;
use libexodus::directories::GameDirectories;
use libexodus::tilesets::Tileset;
use strum::IntoEnumIterator;
use crate::game::GamePlugin;
use crate::mapeditor::MapEditorPlugin;
use crate::tileset_manager::{file_name_for_tileset, find_handle_with_path, RpgSpriteHandles, TilesetManager};
use crate::ui::egui_textures::egui_fonts;
use crate::ui::{Ui, UiSizeChangedEvent};
use crate::ui::uicontrols::WindowUiOverlayInfo;

#[macro_use]
extern crate rust_i18n;
i18n!("locales");

mod game;
mod mapeditor;
mod util;
mod dialogs;
mod ui;
mod tileset_manager;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    MapSelectionScreen,
    CreditsScreen,
    ConfigScreen,
    Playing,
    MapEditor,
    MapEditorDialog,
    Loading,
}

#[derive(Resource)]
pub struct GameDirectoriesWrapper {
    pub game_directories: GameDirectories,
}


#[derive(Resource)]
pub struct GameConfig {
    pub config: Config,
    pub file: PathBuf,
}

impl FromWorld for GameDirectoriesWrapper {
    fn from_world(_: &mut World) -> Self {
        GameDirectoriesWrapper {
            game_directories: GameDirectories::from_system_config().map_err(|err| format!("Invalid system configuration! Error: {}", err)).unwrap()
        }
    }
}

/// The layer to draw the world and players onto
pub const LAYER_ID: Layer = 1;

/// Main init method for the game.
/// This method ensures that all necessary directories actually exist and are writable.
fn game_init(
    mut commands: Commands,
    directories: Res<GameDirectoriesWrapper>,
    mut ctx: ResMut<EguiContext>,
    mut res_tileset: ResMut<TilesetManager>,
) {
    if !directories.game_directories.maps_dir.as_path().exists() {
        fs::create_dir_all(&directories.game_directories.maps_dir)
            .expect(format!("Could not create the map directory at {}!", directories.game_directories.maps_dir.as_path().to_str().unwrap_or("<Invalid>")).as_str());
    }
    if !directories.game_directories.config_dir.as_path().exists() {
        fs::create_dir_all(&directories.game_directories.config_dir)
            .expect(format!("Could not create the config directory at {}!", directories.game_directories.config_dir.as_path().to_str().unwrap_or("<Invalid>")).as_str());
    }
    info!("Set Maps Directory to {}",&directories.game_directories.maps_dir.as_path().to_str().unwrap_or("<Invalid Path>"));
    info!("Set Config Directory to {}",&directories.game_directories.config_dir.as_path().to_str().unwrap_or("<Invalid Path>"));
    let config_file = directories.game_directories.config_file();
    info!("Loading Config File {}",config_file.as_path().to_str().unwrap_or("<Invalid Path>"));
    let config = Config::load_from_file(config_file.as_path())
        .map_err(|e| {
            if config_file.exists() {
                error!("Could not load config file - resorting to default config! {}",e.to_string())
            } else {
                warn!("The config file does not exist")
            }
        })
        .unwrap_or(Config::default());
    debug!("Loaded Config with language {}",config.game_language.to_string());
    rust_i18n::set_locale(config.game_language.locale());
    res_tileset.current_tileset = config.tile_set;
    commands.insert_resource(GameConfig {
        config,
        file: config_file,
    });
    // Initialize Styling and fonts for egui
    egui_fonts(ctx.ctx_mut());
}

fn load_asset_folder_or_panic(
    asset_server: &AssetServer,
    path: &str,
) -> Vec<HandleUntyped> {
    asset_server.load_folder(path).expect(format!("Could not find asset folder at {}", path).as_str())
}

fn load_textures(
    mut rpg_sprite_handles: ResMut<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    // Load the textures - Bevy takes care of resolving the paths, see https://bevy-cheatbook.github.io/assets/assetserver.html
    rpg_sprite_handles.handles = load_asset_folder_or_panic(&asset_server, "textures/tilesets");
}

fn check_and_init_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut tileset_manager: ResMut<TilesetManager>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        // Load Tilesets
        for tileset in Tileset::iter() {
            let tileset: Tileset = tileset;
            let mut textures_folder = PathBuf::from("tilesets");
            textures_folder.push(file_name_for_tileset(&tileset));
            let handle = find_handle_with_path(textures_folder.as_path(), &*asset_server, &sprite_handles.handles);
            let texture_atlas = TextureAtlas::from_grid(
                handle.clone(),
                Vec2::splat(tileset.texture_size() as f32),
                16,
                16,
                None,
                None,
            );
            let atlas_handle = texture_atlases.add(texture_atlas);
            tileset_manager.set_handle(tileset, atlas_handle);
            debug!("Successfully loaded texture atlas {0} with tile size {1}x{1}",
                    asset_server.get_handle_path(handle).unwrap().path().to_str().unwrap(),
                    tileset.texture_size()
                );
        }
        // Finish loading and start the main menu
        state.set(AppState::MainMenu).unwrap();
    }
}

struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RpgSpriteHandles>()
            .init_resource::<TilesetManager>()
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_textures))
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_and_init_textures))
        ;
    }
}

fn resize_notificator(
    mut resize_event: EventReader<WindowResized>,
    mut ev_camera_writer: EventWriter<UiSizeChangedEvent>,
    window: Res<Windows>,
) {
    for e in resize_event.iter() {
        if e.id == window.get_primary().unwrap().id() {
            debug!("The main window was resized to a new size of {} x {}", e.width, e.height);
            ev_camera_writer.send(UiSizeChangedEvent);
        }
    }
}

pub(crate) fn get_buildnr() -> String {
    option_env!("BUILD_NUMBER").map(|b| format!(".{}", b)).unwrap_or("".to_string())
}

fn main() {
    let mut window_title: String = format!("{} {}{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), get_buildnr());
    if cfg!(debug_assertions) {
        window_title.push_str(format!(" Build {} ({})", env!("GIT_SHORTHASH"), env!("GIT_SHORTDATE")).as_str());
    }
    App::new()
        .init_resource::<GameDirectoriesWrapper>()
        .add_event::<UiSizeChangedEvent>()
        .init_resource::<WindowUiOverlayInfo>()
        .add_startup_system(game_init)
        .add_state(AppState::Loading)
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: window_title,
                    resizable: true,
                    width: 1001.,
                    height: 501.,
                    cursor_visible: true,
                    decorations: true,
                    mode: WindowMode::Windowed,
                    ..Default::default()
                },
                ..default()
            })
            .set(LogPlugin {
                filter: if cfg!(debug_assertions) {
                    "info,wgpu_core=warn,wgpu_hal=warn,exodus=debug"
                } else {
                    "info,wgpu_core=warn,wgpu_hal=warn,exodus=info"
                }.into(),
                level: bevy::log::Level::DEBUG,
            })
        )
        .add_system(resize_notificator)
        .add_plugin(EguiPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(Ui)
        .add_plugin(MapEditorPlugin)
        .add_plugin(LoadingPlugin)
        .run();
}
