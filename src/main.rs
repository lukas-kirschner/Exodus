use std::fs;
use bevy::asset::LoadState;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResized};
use bevy_egui::{EguiContext, EguiPlugin};
use indoc::printdoc;
use libexodus::directories::GameDirectories;
use crate::creditsscreen::CreditsScreen;
use crate::game::constants::TEXTURE_SIZE;
use crate::game::GamePlugin;
use crate::mainmenu::MainMenu;
use crate::mapeditor::MapEditorPlugin;
use crate::mapselectionscreen::MapSelectionScreenPlugin;
use crate::uicontrols::{egui_fonts, UiSizeChangedEvent, WindowUiOverlayInfo};

mod game;
mod mainmenu;
mod creditsscreen;
mod uicontrols;
mod mapselectionscreen;
mod mapeditor;
mod util;
mod dialogs;
mod egui_textures;

// We use https://opengameart.org/content/tiny-platform-quest-sprites free textures
// TODO !!! Textures are CC-BY-SA 3.0
// TODO There is a bug in Bevy that causes adjacent textures from the atlas to leak through due to precision errors: https://github.com/bevyengine/bevy/issues/1949


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    MapSelectionScreen,
    CreditsScreen,
    Playing,
    MapEditor,
    MapEditorDialog,
    Loading,
}

#[derive(Resource)]
pub struct GameDirectoriesWrapper {
    pub game_directories: GameDirectories,
}

impl FromWorld for GameDirectoriesWrapper {
    fn from_world(_: &mut World) -> Self {
        GameDirectoriesWrapper {
            game_directories: GameDirectories::from_system_config().map_err(|err| format!("Invalid system configuration! Error: {}", err)).unwrap()
        }
    }
}

/// Main init method for the game.
/// This method ensures that all necessary directories actually exist and are writable.
fn game_init(
    directories: Res<GameDirectoriesWrapper>,
    mut ctx: ResMut<EguiContext>,
) {
    if !directories.game_directories.maps_dir.as_path().exists() {
        fs::create_dir_all(&directories.game_directories.maps_dir)
            .expect(format!("Could not create the map directory at {}!", directories.game_directories.maps_dir.as_path().to_str().unwrap_or("<Invalid>")).as_str());
    }
    if !directories.game_directories.config_dir.as_path().exists() {
        fs::create_dir_all(&directories.game_directories.config_dir)
            .expect(format!("Could not create the config directory at {}!", directories.game_directories.config_dir.as_path().to_str().unwrap_or("<Invalid>")).as_str());
    }
    printdoc! {"
    Using directory structure:
        Maps directory: {maps_dir}
        Config Directory: {config_dir}
    ",
        maps_dir = &directories.game_directories.maps_dir.as_path().to_str().unwrap_or("<Invalid Path>"),
        config_dir = &directories.game_directories.config_dir.as_path().to_str().unwrap_or("<Invalid Path>")
    }
    // Initialize Styling and fonts for egui
    egui_fonts(ctx.ctx_mut());
}

#[derive(Resource)]
pub struct RpgSpriteHandles {
    // TODO Change to include metadata of textures
    handles: Vec<HandleUntyped>,
}

impl FromWorld for RpgSpriteHandles {
    fn from_world(_: &mut World) -> Self {
        RpgSpriteHandles {
            handles: vec![],
        }
    }
}

#[derive(Resource)]
pub struct PlayerSpriteHandles {
    // TODO Change to include metadata of textures
    handles: Vec<HandleUntyped>,
}

impl FromWorld for PlayerSpriteHandles {
    fn from_world(_: &mut World) -> Self {
        PlayerSpriteHandles {
            handles: vec![],
        }
    }
}

fn load_textures(
    mut rpg_sprite_handles: ResMut<RpgSpriteHandles>,
    mut player_sprite_handles: ResMut<PlayerSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    // Load the textures - Bevy takes care of resolving the paths, see https://bevy-cheatbook.github.io/assets/assetserver.html
    rpg_sprite_handles.handles = asset_server.load_folder("textures/tilesets").unwrap();
    player_sprite_handles.handles = asset_server.load_folder("textures/players").unwrap();
}

#[derive(Resource)]
pub struct CurrentMapTextureAtlasHandle {
    pub handle: Handle<TextureAtlas>,
}

impl FromWorld for CurrentMapTextureAtlasHandle {
    fn from_world(_: &mut World) -> Self {
        CurrentMapTextureAtlasHandle {
            handle: Handle::default()
        }
    }
}

#[derive(Resource)]
pub struct CurrentPlayerTextureAtlasHandle {
    pub handle: Handle<TextureAtlas>,
}

impl FromWorld for CurrentPlayerTextureAtlasHandle {
    fn from_world(_: &mut World) -> Self {
        CurrentPlayerTextureAtlasHandle {
            handle: Handle::default()
        }
    }
}

fn check_and_init_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<RpgSpriteHandles>,
    player_sprite_handles: ResMut<PlayerSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if let LoadState::Loaded =
    asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        if let LoadState::Loaded =
        asset_server.get_group_load_state(player_sprite_handles.handles.iter().map(|handle| handle.id))
        {
            // Load Tilesets
            for handle in &sprite_handles.handles {
                let handle = handle.typed_weak();
                let texture_atlas = TextureAtlas::from_grid(
                    handle,
                    Vec2::splat(TEXTURE_SIZE as f32),
                    16,
                    16,
                    None,
                    None,
                );
                let atlas_handle = texture_atlases.add(texture_atlas);
                // TODO Keep a database of all loaded textures here to allow using multiple textures
                commands.insert_resource(CurrentMapTextureAtlasHandle { handle: atlas_handle.clone() });
            }
            // Load Player Texture Sets
            for handle in &player_sprite_handles.handles {
                let handle = handle.typed_weak();
                let texture_atlas = TextureAtlas::from_grid(
                    handle,
                    Vec2::splat(TEXTURE_SIZE as f32),
                    16,
                    16,
                    None,
                    None,
                );
                let atlas_handle = texture_atlases.add(texture_atlas);
                // TODO Keep a database of all loaded textures here to allow using multiple player textures
                commands.insert_resource(CurrentPlayerTextureAtlasHandle { handle: atlas_handle.clone() });
            }
            // TODO no more code duplication
            // Finish loading and start the main menu
            state.set(AppState::MainMenu).unwrap();
        }
    }
}

struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RpgSpriteHandles>()
            .init_resource::<PlayerSpriteHandles>()
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
        .add_plugin(MainMenu)
        .add_plugin(MapSelectionScreenPlugin)
        .add_plugin(CreditsScreen)
        .add_plugin(MapEditorPlugin)
        .add_plugin(LoadingPlugin)
        .run();
}
