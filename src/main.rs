use crate::animation::AnimationPlugin;
use crate::campaign::campaign_trail::CampaignTrailPlugin;
use crate::campaign::campaign_trail_asset_loader::CampaignTrailAssetPlugin;
use crate::campaign::MainCampaignLoader;
use crate::game::{GamePlugin, HighscoresDatabaseWrapper};
use crate::mapeditor::MapEditorPlugin;
use crate::textures::tileset_manager::{RpgSpriteHandles, TilesetManager};
use crate::textures::Textures;
use crate::ui::uicontrols::WindowUiOverlayInfo;
use crate::ui::{Ui, UiSizeChangedEvent};
use bevy::asset::LoadedFolder;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::render::view::Layer;
use bevy::window::{PrimaryWindow, WindowMode, WindowResized, WindowResolution};
use bevy_egui::EguiPlugin;
use libexodus::config::Config;
use libexodus::directories::GameDirectories;
use libexodus::highscores::highscores_database::HighscoresDatabase;
use std::fs;
use std::path::PathBuf;

#[macro_use]
extern crate rust_i18n;
i18n!("locales");

mod animation;
mod campaign;
mod dialogs;
mod game;
mod mapeditor;
mod textures;
mod ui;
mod util;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum AppLabels {
    PlayerMovement,
    Gravity,
    World,
    ResetScore,
    Player,
    GameOverTrigger,
    GameUI,
    Camera,
    LoadMaps,
    /// Prepare data to display to the player, e.g. maps and textures.
    /// Executed before all camera systems and world set-up.
    PrepareData,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    MapSelectionScreen,
    CreditsScreen,
    ConfigScreen,
    CampaignTrailScreen,
    Playing,
    MapEditor,
    MapEditorDialog,
    #[default]
    /// Loading and processing all textures
    Loading,
    /// Loading all campaign maps
    LoadingCampaign,
    Process,
    MainMenu,
    GameOverScreen,
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
impl GameConfig {
    pub fn texture_size(&self) -> f32 {
        self.config.tile_set.texture_size() as f32
    }
}

impl FromWorld for GameDirectoriesWrapper {
    fn from_world(_: &mut World) -> Self {
        GameDirectoriesWrapper {
            game_directories: GameDirectories::from_system_config()
                .map_err(|err| format!("Invalid system configuration! Error: {}", err))
                .unwrap(),
        }
    }
}

/// The layer to draw the world and players onto
pub const LAYER_ID: Layer = 1;

/// Main init method for the game.
/// This method ensures that all necessary directories actually exist and are writable.
/// TODO - This needs to be COMPLETELY refactored in order to make this game portable for WebGL and Android
fn game_init(
    mut commands: Commands,
    directories: Res<GameDirectoriesWrapper>,
    mut res_tileset: ResMut<TilesetManager>,
) {
    if !directories.game_directories.maps_dir.as_path().exists() {
        fs::create_dir_all(&directories.game_directories.maps_dir).unwrap_or_else(|_| {
            panic!(
                "Could not create the map directory at {}!",
                directories
                    .game_directories
                    .maps_dir
                    .as_path()
                    .to_str()
                    .unwrap_or("<Invalid>")
            )
        });
    }
    if !directories.game_directories.config_dir.as_path().exists() {
        fs::create_dir_all(&directories.game_directories.config_dir).unwrap_or_else(|_| {
            panic!(
                "Could not create the config directory at {}!",
                directories
                    .game_directories
                    .config_dir
                    .as_path()
                    .to_str()
                    .unwrap_or("<Invalid>")
            )
        });
    }
    info!(
        "Set Maps Directory to {}",
        &directories
            .game_directories
            .maps_dir
            .as_path()
            .to_str()
            .unwrap_or("<Invalid Path>")
    );
    info!(
        "Set Config Directory to {}",
        &directories
            .game_directories
            .config_dir
            .as_path()
            .to_str()
            .unwrap_or("<Invalid Path>")
    );
    let config_file = directories.game_directories.config_file();
    info!(
        "Loading Config File {}",
        config_file.as_path().to_str().unwrap_or("<Invalid Path>")
    );
    let config = Config::load_from_file(config_file.as_path())
        .map_err(|e| {
            if config_file.exists() {
                error!(
                    "Could not load config file - resorting to default config! {}",
                    e.to_string()
                )
            } else {
                warn!("The config file does not exist")
            }
        })
        .unwrap_or_default();
    debug!(
        "Loaded Config with language {}",
        config.game_language.to_string()
    );
    rust_i18n::set_locale(config.game_language.locale());
    res_tileset.current_tileset = config.tile_set;
    commands.insert_resource(GameConfig {
        config,
        file: config_file,
    });
    // Load the Highscores Database
    let highscores_file = directories.game_directories.highscores_file();
    info!(
        "Loading High Scores File {}",
        highscores_file
            .as_path()
            .to_str()
            .unwrap_or("<Invalid Path>")
    );
    let highscores_database = HighscoresDatabase::load_from_file(highscores_file.as_path())
        .map_err(|e| {
            if highscores_file.exists() {
                panic!("Could not load high scores file! {}", e)
            } else {
                warn!(
                    "The high scores file does not exist - Initializing a new empty one at {}",
                    highscores_file.to_str().unwrap_or("<invalid>")
                )
            }
        })
        .unwrap_or_default();
    commands.insert_resource(HighscoresDatabaseWrapper {
        highscores: highscores_database,
        file: highscores_file,
    });
    // Initialize Styling and fonts for egui
}

struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RpgSpriteHandles>()
            .init_resource::<TilesetManager>()
            .add_plugins(Textures);
    }
}

fn resize_notificator(
    mut resize_event: EventReader<WindowResized>,
    mut ev_camera_writer: EventWriter<UiSizeChangedEvent>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(_) = window.get_single() else {
        return;
    };
    for _ in resize_event.read() {
        // event_window = commands.entity(e.window);
        // if event_window == primary {
        ev_camera_writer.send(UiSizeChangedEvent);
        // }
    }
}

pub(crate) fn get_buildnr() -> String {
    option_env!("BUILD_NUMBER")
        .map(|b| format!(".{}", b))
        .unwrap_or_default()
}

#[derive(Resource)]
/// A struct containing all asset handles that should be waited for before entering the
/// Process state. These handles are only used for waiting, not for querying
pub struct AllAssetHandles {
    pub handles: Vec<Handle<LoadedFolder>>,
    pub file_handles: Vec<UntypedHandle>,
}
impl FromWorld for AllAssetHandles {
    fn from_world(_: &mut World) -> Self {
        AllAssetHandles {
            handles: vec![],
            file_handles: vec![],
        }
    }
}

fn main() {
    let mut window_title: String = format!(
        "{} {}{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        get_buildnr()
    );
    if cfg!(debug_assertions) {
        window_title.push_str(
            format!(
                " Build {} ({})",
                env!("GIT_SHORTHASH"),
                env!("GIT_SHORTDATE")
            )
            .as_str(),
        );
    }
    App::new()
        .init_resource::<GameDirectoriesWrapper>()
        .add_event::<UiSizeChangedEvent>()
        .init_resource::<WindowUiOverlayInfo>()
        .init_resource::<AllAssetHandles>()
        .add_systems(Startup, game_init)
        .init_state::<AppState>()
        .insert_resource(Msaa::Sample2)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: window_title,
                        resizable: true,
                        resolution: WindowResolution::new(1001., 501.),
                        decorations: true,
                        mode: WindowMode::Windowed,
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    filter: if cfg!(debug_assertions) {
                        "info,wgpu_core=warn,wgpu_hal=warn,exodus=debug"
                    } else {
                        "info,wgpu_core=warn,wgpu_hal=warn,exodus=info"
                    }
                    .into(),
                    level: bevy::log::Level::DEBUG,
                    update_subscriber: None,
                }),
        )
        .add_systems(Update, resize_notificator)
        .add_plugins(CampaignTrailAssetPlugin)
        .add_plugins(MainCampaignLoader)
        .add_plugins(EguiPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(Ui)
        .add_plugins(MapEditorPlugin)
        .add_plugins(CampaignTrailPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(LoadingPlugin)
        .run();
}
