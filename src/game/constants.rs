/// (1000 / rows) - margins; // Size of a tile, all tiles are square
pub const TILE_SIZE: f32 = 1.0;
/// Texture size in pixels
pub const TEXTURE_SIZE: usize = 32;
/// The Player's Z coordinate
pub const PLAYER_Z: f32 = -1.0;
/// The World Tiles' Z coordinate
pub const WORLD_Z: f32 = -2.0;
/// The Map Editor Preview Tiles' Z coordinate
pub const MAPEDITOR_PREVIEWTILE_Z: f32 = 0.0;
/// The Map Editor Preview Tiles' Alpha value [0.0,1.0]
pub const MAPEDITOR_PREVIEWTILE_ALPHA: f32 = 0.75;
/// The Map Editor Preview Tiles' Air Texture Atlas Index
pub const MAPEDITOR_PREVIEWTILE_AIR_ATLAS_INDEX: usize = 45;
/// The speed of the player movement
pub const PLAYER_SPEED: f32 = 4.0;
/// The pickup distance for coins
pub const COIN_PICKUP_DISTANCE: f32 = 0.1;
/// The UI Font Size for main UI elements
pub const UI_FONT_SIZE: f32 = TEXTURE_SIZE as f32;
/// The speed a dead player ascends to heaven at
pub const DEAD_PLAYER_ASCEND_SPEED: f32 = 3.0;
/// The speed a dead player ascends to heaven at
pub const DEAD_PLAYER_ZOOM_SPEED: f32 = 0.1;
/// The speed a dead player decays
pub const DEAD_PLAYER_DECAY_SPEED: f32 = 2.0;
/// The border width of menu panels
pub const MENU_BORDER_WIDTH: f32 = 8.0;
/// The button size for the map editor
pub const MAPEDITOR_BUTTON_SIZE: f32 = 38.;