/// (1000 / rows) - margins; // Size of a tile, all tiles are square
pub const TILE_SIZE: f32 = 1.0;
/// Texture size in pixels
pub const TEXTURE_SIZE: usize = 32;
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
pub const MENU_BORDER_WIDTH:f32 = 8.0;