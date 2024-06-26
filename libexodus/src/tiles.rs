use crate::directions::FromDirection;
use crate::directions::FromDirection::{FROMEAST, FROMNORTH, FROMSOUTH, FROMWEST};
use std::fmt;
use std::fmt::Formatter;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

pub const EXITING_PLAYER_SPRITE: usize = 247; // The player turning heir back to the camera
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InteractionKind {
    /// When interacting with this tile, the player may decide to play a map.
    /// This interaction kind is mainly used for tile-based Campaign Trails
    LaunchMap { map_name: String },
    /// When interacting with this tile, the player should be teleported
    /// to the given teleport exit.
    TeleportTo { teleport_id: TeleportId },
}
impl Default for InteractionKind {
    fn default() -> Self {
        InteractionKind::LaunchMap {
            map_name: "".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TileKind {
    ///
    /// A tile that cannot interact with the player in any way
    AIR,
    ///
    /// A solid tile
    SOLID,
    ///
    /// A tile that kills the player on impact
    DEADLY { from: Vec<FromDirection> },
    ///
    /// A special tile that the player can interact with
    SPECIAL { interaction: InteractionKind },
    ///
    ///
    PLAYERSPAWN,
    ///
    /// A collectible coin
    COIN,
    ///
    /// A collectible key
    KEY,
    ///
    /// A collectible that does not change any counter when collected
    COLLECTIBLE,
    ///
    /// A door that can be opened (removed) using a key
    DOOR,
    ///
    /// A ladder
    LADDER,
    ///
    /// The map exit
    EXIT,
}

pub type AtlasIndex = usize;

/// The ID for a Teleport
#[derive(Default, Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, EnumIter, Hash)]
pub enum TeleportId {
    #[default]
    ONE,
    TWO,
    THREE,
    FOUR,
}

impl TryFrom<u8> for TeleportId {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(TeleportId::const_from_u8(value))
    }
}
impl From<&TeleportId> for u8 {
    fn from(value: &TeleportId) -> Self {
        value.const_to_u8()
    }
}

impl TeleportId {
    pub const fn const_to_u8(&self) -> u8 {
        match self {
            TeleportId::ONE => 0,
            TeleportId::TWO => 1,
            TeleportId::THREE => 2,
            TeleportId::FOUR => 3,
        }
    }
    pub const fn const_from_u8(value: u8) -> Self {
        match value {
            0 => TeleportId::ONE,
            1 => TeleportId::TWO,
            2 => TeleportId::THREE,
            3 => TeleportId::FOUR,
            _ => panic!("Invalid Teleport ID"),
        }
    }
}

// Tiles Definitions
#[derive(Clone, Debug, Eq, PartialEq, EnumIter, EnumCountMacro, Default)]
pub enum Tile {
    /// An air tile without a texture
    #[default]
    AIR,
    /// A tile of Wall, a solid block that cannot be destroyed
    WALL,
    /// An alternative Wall with a "nature" texture
    WALLNATURE,
    /// An alternative Wall with a "cobblestone" texture
    WALLCOBBLE,
    /// An alternative Wall with a "smooth" texture
    WALLSMOOTH,
    /// An alternative Wall with a "chiseled" texture
    WALLCHISELED,
    /// A solid slope
    SLOPE,
    /// A solid pillar
    PILLAR,
    /// The position where the player spawns
    PLAYERSPAWN,
    /// A door that can be opened with a key
    DOOR,
    /// A opened door
    OPENDOOR,
    /// A collectible coin
    COIN,
    /// A collectible key
    KEY,
    /// A ladder
    LADDER,
    /// A decorated ladder with a Nature-Themed slope
    LADDERNATURE,
    /// A decorated ladder with a Wall-Themed slope
    LADDERSLOPE,
    /// Spikes that sit on the ground and point up
    SPIKES,
    /// Spikes that sit on the ground and point up, alternative texture
    SPIKESALT,
    /// Spikes placed on a slope and point up, only deadly if touched from above.
    SPIKESSLOPED,
    /// Spikes placed on a wall, pointing up. Only deadly if approached from above.
    WALLSPIKEST,
    /// Spikes placed on a wall, pointing to the left. Only deadly if approached from the left.
    WALLSPIKESL,
    /// Spikes placed on a wall, pointing to the right. Only deadly if approached from the right.
    WALLSPIKESR,
    /// Spikes placed on a wall, pointing to the left and right. Only deadly if approached from the left or right.
    WALLSPIKESLR,
    /// Spikes placed on a wall, pointing to the bottom. Only deadly if approached from below.
    WALLSPIKESB,
    /// Spikes placed on a wall, pointing to the left and bottom. Only deadly if approached from the left or from below.
    WALLSPIKESLB,
    /// Spikes placed on a wall, pointing to the right and bottom. Only deadly if approached from the right or from below.
    WALLSPIKESRB,
    /// Spikes placed on a wall, pointing to the top and bottom. Only deadly if approached from above or from below.
    WALLSPIKESTB,
    /// Spikes placed on a wall, pointing to the top and left. Only deadly if approached from above or from the left.
    WALLSPIKESLT,
    /// Spikes placed on a wall, pointing to the top and right. Only deadly if approached from above or from the right.
    WALLSPIKESRT,
    /// Spikes placed on a wall, pointing to all directions. Deadly if approached from any direction.
    WALLSPIKESRLTB,
    /// Spikes placed on a wall, pointing to the top, right and bottom. Not deadly if approached from the left.
    WALLSPIKESRTB,
    /// Spikes placed on a wall, pointing to the right, left and bottom. Not deadly if approached from the top.
    WALLSPIKESRLB,
    /// Spikes placed on a wall, pointing to the top, right and left. Not deadly if approached from below.
    WALLSPIKESRLT,
    /// Spikes placed on a wall, pointing to the top, left and bottom. Not deadly if approached from the right.
    WALLSPIKESLTB,
    /// A decorative Cobblestone Roof Tile, with a 45 degrees angle on the left.
    COBBLEROOFSLOPEL,
    /// A decorative Cobblestone Roof Tile, with a 45 degrees angle on the right.
    COBBLEROOFSLOPER,
    /// An arrow that points to the right
    ARROWRIGHT,
    /// An arrow that points to the left
    ARROWLEFT,
    /// An arrow that points up
    ARROWUP,
    /// An arrow that points down
    ARROWDOWN,
    /// The map exit
    EXIT,
    /// A Campaign Trail Connecting Segment between two maps
    CAMPAIGNTRAILWALKWAY,
    /// A Campaign Trail Map Entry Point
    CAMPAIGNTRAILMAPENTRYPOINT { interaction: InteractionKind },
    /// The border of a campaign trail, invisible but cannot be entered or interacted with
    CAMPAIGNTRAILBORDER,
    /// A locked Campaign Trail Map that cannot be played yet
    CAMPAIGNTRAILLOCKEDMAPENTRYPOINT { interaction: InteractionKind },
    /// A message tile that shows a message to the player as soon as they interact with it.
    MESSAGE { message_id: usize },
    /// The entry of a Teleport, there may be more than one of these on a single map
    TELEPORTENTRY { teleport_id: TeleportId },
    /// The exit of a teleport, there may be only one on each map
    TELEPORTEXIT { teleport_id: TeleportId },
}

impl Tile {
    /// Get the tile kind of the given tile
    pub fn kind(&self) -> TileKind {
        match self {
            Tile::AIR => TileKind::AIR,
            Tile::WALL => TileKind::SOLID,
            Tile::WALLNATURE => TileKind::SOLID,
            Tile::WALLCOBBLE => TileKind::SOLID,
            Tile::WALLSMOOTH => TileKind::SOLID,
            Tile::WALLCHISELED => TileKind::SOLID,
            Tile::SLOPE => TileKind::SOLID,
            Tile::PILLAR => TileKind::SOLID,
            Tile::PLAYERSPAWN => TileKind::PLAYERSPAWN,
            Tile::COIN => TileKind::COIN,
            Tile::LADDER => TileKind::LADDER,
            Tile::LADDERNATURE => TileKind::LADDER,
            Tile::LADDERSLOPE => TileKind::LADDER,
            Tile::SPIKES => TileKind::DEADLY {
                from: vec![FROMNORTH, FROMSOUTH, FROMEAST, FROMWEST],
            },
            Tile::SPIKESALT => TileKind::DEADLY {
                from: vec![FROMNORTH, FROMSOUTH, FROMEAST, FROMWEST],
            },
            Tile::SPIKESSLOPED => TileKind::DEADLY {
                from: vec![FROMNORTH],
            },
            Tile::WALLSPIKESL => TileKind::DEADLY {
                from: vec![FROMWEST],
            },
            Tile::WALLSPIKEST => TileKind::DEADLY {
                from: vec![FROMNORTH],
            },
            Tile::WALLSPIKESR => TileKind::DEADLY {
                from: vec![FROMEAST],
            },
            Tile::WALLSPIKESLR => TileKind::DEADLY {
                from: vec![FROMEAST, FROMWEST],
            },
            Tile::WALLSPIKESLT => TileKind::DEADLY {
                from: vec![FROMWEST, FROMNORTH],
            },
            Tile::WALLSPIKESRT => TileKind::DEADLY {
                from: vec![FROMEAST, FROMNORTH],
            },
            Tile::WALLSPIKESB => TileKind::DEADLY {
                from: vec![FROMSOUTH],
            },
            Tile::WALLSPIKESLB => TileKind::DEADLY {
                from: vec![FROMWEST, FROMSOUTH],
            },
            Tile::WALLSPIKESRB => TileKind::DEADLY {
                from: vec![FROMEAST, FROMSOUTH],
            },
            Tile::WALLSPIKESTB => TileKind::DEADLY {
                from: vec![FROMNORTH, FROMSOUTH],
            },
            Tile::WALLSPIKESRLTB => TileKind::DEADLY {
                from: vec![FROMNORTH, FROMSOUTH, FROMEAST, FROMWEST],
            },
            Tile::WALLSPIKESRTB => TileKind::DEADLY {
                from: vec![FROMNORTH, FROMSOUTH, FROMEAST],
            },
            Tile::WALLSPIKESLTB => TileKind::DEADLY {
                from: vec![FROMNORTH, FROMSOUTH, FROMWEST],
            },
            Tile::WALLSPIKESRLB => TileKind::DEADLY {
                from: vec![FROMSOUTH, FROMEAST, FROMWEST],
            },
            Tile::WALLSPIKESRLT => TileKind::DEADLY {
                from: vec![FROMNORTH, FROMEAST, FROMWEST],
            },
            Tile::DOOR => TileKind::DOOR,
            Tile::KEY => TileKind::KEY,
            Tile::OPENDOOR => TileKind::AIR,
            Tile::ARROWRIGHT => TileKind::COLLECTIBLE,
            Tile::ARROWLEFT => TileKind::COLLECTIBLE,
            Tile::ARROWUP => TileKind::COLLECTIBLE,
            Tile::ARROWDOWN => TileKind::COLLECTIBLE,
            Tile::MESSAGE { .. } => TileKind::COLLECTIBLE,
            Tile::EXIT => TileKind::EXIT,
            Tile::CAMPAIGNTRAILWALKWAY => TileKind::LADDER,
            Tile::CAMPAIGNTRAILMAPENTRYPOINT { interaction } => TileKind::SPECIAL {
                interaction: interaction.clone(),
            },
            Tile::CAMPAIGNTRAILBORDER => TileKind::SOLID,
            Tile::CAMPAIGNTRAILLOCKEDMAPENTRYPOINT { .. } => TileKind::SOLID,
            Tile::TELEPORTENTRY { teleport_id } => TileKind::SPECIAL {
                interaction: InteractionKind::TeleportTo {
                    teleport_id: *teleport_id,
                },
            },
            Tile::TELEPORTEXIT { .. } => TileKind::AIR,
            Tile::COBBLEROOFSLOPEL => TileKind::AIR,
            Tile::COBBLEROOFSLOPER => TileKind::AIR,
        }
    }
    pub fn atlas_index(&self) -> Option<AtlasIndex> {
        match self {
            Tile::AIR => None,
            Tile::WALL => Some(58),
            Tile::WALLNATURE => Some(103),
            Tile::WALLCOBBLE => Some(123),
            Tile::WALLSMOOTH => Some(57),
            Tile::WALLCHISELED => Some(52),
            Tile::SLOPE => Some(140),
            Tile::PILLAR => Some(29),
            Tile::PLAYERSPAWN => None,
            Tile::COIN => Some(217),
            Tile::LADDER => Some(220),
            Tile::LADDERNATURE => Some(226),
            Tile::LADDERSLOPE => Some(225),
            Tile::SPIKES => Some(227),
            Tile::SPIKESALT => Some(228),
            Tile::SPIKESSLOPED => Some(250),
            Tile::WALLSPIKESL => Some(244),
            Tile::WALLSPIKESR => Some(242),
            Tile::WALLSPIKEST => Some(231),
            Tile::WALLSPIKESLR => Some(243),
            Tile::WALLSPIKESB => Some(233),
            Tile::WALLSPIKESLB => Some(232),
            Tile::WALLSPIKESRT => Some(230),
            Tile::WALLSPIKESRB => Some(234),
            Tile::WALLSPIKESLT => Some(216),
            Tile::WALLSPIKESTB => Some(238),
            Tile::WALLSPIKESRLTB => Some(254),
            Tile::WALLSPIKESLTB => Some(237),
            Tile::WALLSPIKESRLB => Some(236),
            Tile::WALLSPIKESRLT => Some(235),
            Tile::WALLSPIKESRTB => Some(239),
            Tile::DOOR => Some(200),
            Tile::KEY => Some(201),
            Tile::OPENDOOR => Some(196),
            Tile::ARROWRIGHT => Some(35),
            Tile::ARROWLEFT => Some(36),
            Tile::ARROWUP => Some(37),
            Tile::ARROWDOWN => Some(34),
            Tile::EXIT => Some(40),
            Tile::CAMPAIGNTRAILWALKWAY => Some(78),
            Tile::CAMPAIGNTRAILMAPENTRYPOINT { .. } => Some(76),
            Tile::CAMPAIGNTRAILBORDER => None,
            Tile::CAMPAIGNTRAILLOCKEDMAPENTRYPOINT { .. } => Some(77),
            Tile::MESSAGE { .. } => Some(33),
            Tile::TELEPORTENTRY { teleport_id } => {
                Some(1 + (u8::from(teleport_id) * 2) as AtlasIndex)
            },
            Tile::TELEPORTEXIT { teleport_id } => {
                Some(2 + (u8::from(teleport_id) * 2) as AtlasIndex)
            },
            Tile::COBBLEROOFSLOPEL => Some(124),
            Tile::COBBLEROOFSLOPER => Some(125),
        }
    }
    pub fn can_collide_from(&self, from_direction: &FromDirection) -> bool {
        match self.kind() {
            TileKind::AIR => false,
            TileKind::SOLID => true,
            TileKind::DEADLY { from } => !from.iter().any(|fromdir| *fromdir == *from_direction),
            TileKind::SPECIAL { .. } => false,
            TileKind::PLAYERSPAWN => false,
            TileKind::COIN => false,
            TileKind::LADDER => false,
            TileKind::KEY => false,
            TileKind::DOOR => true,
            TileKind::COLLECTIBLE => false,
            TileKind::EXIT => false,
        }
    }
    pub fn is_deadly_from(&self, from_direction: &FromDirection) -> bool {
        match self.kind() {
            TileKind::AIR => false,
            TileKind::SOLID => false,
            TileKind::DEADLY { from } => from.iter().any(|fromdir| *fromdir == *from_direction),
            TileKind::SPECIAL { .. } => false,
            TileKind::PLAYERSPAWN => false,
            TileKind::COIN => false,
            TileKind::LADDER => false,
            TileKind::KEY => false,
            TileKind::DOOR => false,
            TileKind::COLLECTIBLE => false,
            TileKind::EXIT => false,
        }
    }
    /// Get a unique string id, describing this tile. Suitable for i18n keys.
    /// Consists only of underscores and lower-case characters.
    pub fn str_id(&self) -> &str {
        match self {
            Tile::AIR => "air",
            Tile::WALL => "wall",
            Tile::WALLNATURE => "wallnature",
            Tile::WALLCOBBLE => "wallcobble",
            Tile::WALLSMOOTH => "wallsmooth",
            Tile::WALLCHISELED => "wallchiseled",
            Tile::SLOPE => "slope",
            Tile::PILLAR => "pillar",
            Tile::PLAYERSPAWN => "playerspawn",
            Tile::DOOR => "door",
            Tile::OPENDOOR => "opendoor",
            Tile::COIN => "coin",
            Tile::KEY => "key",
            Tile::LADDER => "ladder",
            Tile::LADDERNATURE => "laddernature",
            Tile::LADDERSLOPE => "ladderslope",
            Tile::SPIKES => "spikes",
            Tile::SPIKESALT => "spikes_alt",
            Tile::SPIKESSLOPED => "spikes_sloped",
            Tile::WALLSPIKESL => "spikes_wall_l",
            Tile::WALLSPIKESR => "spikes_wall_r",
            Tile::WALLSPIKESLR => "spikes_wall_lr",
            Tile::WALLSPIKESB => "spikes_wall_b",
            Tile::WALLSPIKESLB => "spikes_wall_lb",
            Tile::WALLSPIKESRB => "spikes_wall_rb",
            Tile::WALLSPIKESTB => "spikes_wall_tb",
            Tile::WALLSPIKESRLTB => "spikes_wall_rltb",
            Tile::WALLSPIKESRTB => "spikes_wall_rtb",
            Tile::WALLSPIKESLTB => "spikes_wall_ltb",
            Tile::WALLSPIKEST => "spikes_wall_t",
            Tile::WALLSPIKESLT => "spikes_wall_lt",
            Tile::WALLSPIKESRT => "spikes_wall_rt",
            Tile::WALLSPIKESRLB => "spikes_wall_rlb",
            Tile::WALLSPIKESRLT => "spikes_wall_rlt",
            Tile::ARROWRIGHT => "arrow_right",
            Tile::ARROWLEFT => "arrow_left",
            Tile::ARROWUP => "arrow_up",
            Tile::ARROWDOWN => "arrow_down",
            Tile::EXIT => "exit",
            Tile::CAMPAIGNTRAILWALKWAY => "campaign_trail_walkway",
            Tile::CAMPAIGNTRAILMAPENTRYPOINT { .. } => "campaign_trail_entry_point",
            Tile::CAMPAIGNTRAILBORDER => "campaign_trail_border",
            Tile::CAMPAIGNTRAILLOCKEDMAPENTRYPOINT { .. } => "campaign_trail_locked_entry_point",
            Tile::MESSAGE { .. } => "message",
            Tile::TELEPORTENTRY { .. } => "teleport_entry",
            Tile::TELEPORTEXIT { .. } => "teleport_exit",
            Tile::COBBLEROOFSLOPEL => "cobblestone_roof_l",
            Tile::COBBLEROOFSLOPER => "cobblestone_roof_r",
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::AIR => "Air",
                Tile::WALL => "Wall",
                Tile::WALLNATURE => "Wall (Nature)",
                Tile::WALLCOBBLE => "Wall (Cobblestone)",
                Tile::WALLSMOOTH => "Wall (Smooth)",
                Tile::WALLCHISELED => "Wall (Chiseled)",
                Tile::SLOPE => "Slope",
                Tile::PILLAR => "Pillar",
                Tile::PLAYERSPAWN => "Player Spawn",
                Tile::COIN => "Coin",
                Tile::LADDER => "Ladder",
                Tile::LADDERNATURE => "Ladder with Nature Slope",
                Tile::LADDERSLOPE => "Ladder with Slope",
                Tile::SPIKES => "Spikes",
                Tile::SPIKESALT => "Spikes (Alt Texture)",
                Tile::SPIKESSLOPED => "Sloped Spikes",
                Tile::WALLSPIKESL => "L Wall Spikes",
                Tile::WALLSPIKESR => "R Wall Spikes",
                Tile::WALLSPIKESLR => "LR Wall Spikes",
                Tile::WALLSPIKESB => "B Wall Spikes",
                Tile::WALLSPIKESLB => "LB Wall Spikes",
                Tile::WALLSPIKESRB => "RB Wall Spikes",
                Tile::WALLSPIKESTB => "TB Wall Spikes",
                Tile::WALLSPIKESRLTB => "RLTB Wall Spikes",
                Tile::WALLSPIKESRTB => "RTB Wall Spikes",
                Tile::WALLSPIKESLTB => "LTB Wall Spikes",
                Tile::WALLSPIKEST => "T Wall Spikes",
                Tile::WALLSPIKESLT => "LT Wall Spikes",
                Tile::WALLSPIKESRT => "RT Wall Spikes",
                Tile::WALLSPIKESRLB => "RLB Wall Spikes",
                Tile::WALLSPIKESRLT => "RLT Wall Spikes",
                Tile::DOOR => "Door",
                Tile::OPENDOOR => "Open Door",
                Tile::KEY => "Key",
                Tile::ARROWRIGHT => "Arrow Right",
                Tile::ARROWLEFT => "Arrow Left",
                Tile::ARROWUP => "Arrow Up",
                Tile::ARROWDOWN => "Arrow Down",
                Tile::EXIT => "Exit",
                Tile::CAMPAIGNTRAILWALKWAY => "Campaign Trail Walkway",
                Tile::CAMPAIGNTRAILMAPENTRYPOINT { .. } => "Campaign Trail Map Entry Point",
                Tile::CAMPAIGNTRAILBORDER => "Campaign Trail Border",
                Tile::CAMPAIGNTRAILLOCKEDMAPENTRYPOINT { .. } =>
                    "Campaign Trail Locked Map Entry Point",
                Tile::MESSAGE { .. } => "Message",
                Tile::TELEPORTENTRY { .. } => "Teleport Entry",
                Tile::TELEPORTEXIT { .. } => "Teleport Exit",
                Tile::COBBLEROOFSLOPEL => "Cobblestone Roof L",
                Tile::COBBLEROOFSLOPER => "Cobblestone Roof R",
            }
        )
    }
}
#[derive(EnumIter)]
pub enum UITiles {
    /// Texture for the Edit Map Button
    EDITBUTTON,
    /// Texture for the Play Button
    PLAYBUTTON,
    /// Texture for the Delete Button
    DELETEBUTTON,
    /// Texture for the Back Button
    BACKBUTTON,
    /// Texture for the Save Button
    SAVEBUTTON,
    /// Texture for the Replay Button
    REPLAYBUTTON,
    /// Texture for the Discard Highscore Button
    DISCARDBUTTON,
    /// Texture for the Create new Map Button
    CREATENEWBUTTON,
}

impl UITiles {
    pub fn atlas_index(&self) -> Option<AtlasIndex> {
        Some(match *self {
            UITiles::EDITBUTTON => 22,
            UITiles::PLAYBUTTON => 21,
            UITiles::DELETEBUTTON => 20,
            UITiles::BACKBUTTON => 19,
            UITiles::SAVEBUTTON => 31,
            UITiles::CREATENEWBUTTON => 63,
            UITiles::REPLAYBUTTON => 47,
            UITiles::DISCARDBUTTON => 15,
        })
    }
}
