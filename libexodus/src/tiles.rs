use crate::directions::FromDirection;
use crate::directions::FromDirection::{FROMEAST, FROMNORTH, FROMSOUTH, FROMWEST};
use std::fmt;
use std::fmt::Formatter;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InteractionKind {
    /// When interacting with this tile, the player may decide to play a map.
    /// This interaction kind is mainly used for tile-based Campaign Trails
    LaunchMap { map_name: String },
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

// Tiles Definitions
#[derive(Clone, Debug, Eq, PartialEq, EnumIter, EnumCountMacro)]
pub enum Tile {
    /// An air tile without a texture
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
    /// Spikes that sit on the ground and point up
    SPIKES,
    /// Spikes that sit on the ground and point up, alternative texture
    SPIKESALT,
    /// Spikes that sit on a slope and point up, only deadly if touched from above.
    SPIKESSLOPED,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESL,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESR,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESLR,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESB,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESLB,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESRB,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESTB,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESRLTB,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESRTB,
    /// Spikes that sit on a wall and are only deadly if the player comes from the direction the spikes face to
    WALLSPIKESLTB,
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
            Tile::PLAYERSPAWN => TileKind::PLAYERSPAWN,
            Tile::COIN => TileKind::COIN,
            Tile::LADDER => TileKind::LADDER,
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
            Tile::WALLSPIKESR => TileKind::DEADLY {
                from: vec![FROMEAST],
            },
            Tile::WALLSPIKESLR => TileKind::DEADLY {
                from: vec![FROMEAST, FROMWEST],
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
            Tile::DOOR => TileKind::DOOR,
            Tile::KEY => TileKind::KEY,
            Tile::OPENDOOR => TileKind::AIR,
            Tile::ARROWRIGHT => TileKind::COLLECTIBLE,
            Tile::ARROWLEFT => TileKind::COLLECTIBLE,
            Tile::ARROWUP => TileKind::COLLECTIBLE,
            Tile::ARROWDOWN => TileKind::COLLECTIBLE,
            Tile::EXIT => TileKind::EXIT,
            Tile::CAMPAIGNTRAILWALKWAY => TileKind::LADDER,
            Tile::CAMPAIGNTRAILMAPENTRYPOINT { interaction } => TileKind::SPECIAL {
                interaction: interaction.clone(),
            },
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
            Tile::PLAYERSPAWN => None,
            Tile::COIN => Some(217),
            Tile::LADDER => Some(220),
            Tile::SPIKES => Some(227),
            Tile::SPIKESALT => Some(228),
            Tile::SPIKESSLOPED => Some(250),
            Tile::WALLSPIKESL => Some(244),
            Tile::WALLSPIKESR => Some(242),
            Tile::WALLSPIKESLR => Some(243),
            Tile::WALLSPIKESB => Some(233),
            Tile::WALLSPIKESLB => Some(232),
            Tile::WALLSPIKESRB => Some(234),
            Tile::WALLSPIKESTB => Some(238),
            Tile::WALLSPIKESRLTB => Some(254),
            Tile::WALLSPIKESLTB => Some(237),
            Tile::WALLSPIKESRTB => Some(239),
            Tile::DOOR => Some(200),
            Tile::KEY => Some(201),
            Tile::OPENDOOR => Some(196),
            Tile::ARROWRIGHT => Some(35),
            Tile::ARROWLEFT => Some(36),
            Tile::ARROWUP => Some(37),
            Tile::ARROWDOWN => Some(34),
            Tile::EXIT => Some(40),
            Tile::CAMPAIGNTRAILWALKWAY => Some(9),
            Tile::CAMPAIGNTRAILMAPENTRYPOINT { .. } => Some(10),
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
            Tile::PLAYERSPAWN => "playerspawn",
            Tile::DOOR => "door",
            Tile::OPENDOOR => "opendoor",
            Tile::COIN => "coin",
            Tile::KEY => "key",
            Tile::LADDER => "ladder",
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
            Tile::ARROWRIGHT => "arrow_right",
            Tile::ARROWLEFT => "arrow_left",
            Tile::ARROWUP => "arrow_up",
            Tile::ARROWDOWN => "arrow_down",
            Tile::EXIT => "exit",
            Tile::CAMPAIGNTRAILWALKWAY => "campaign_trail_walkway",
            Tile::CAMPAIGNTRAILMAPENTRYPOINT { .. } => "campaign_trail_entry_point",
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
                Tile::PLAYERSPAWN => "Player Spawn",
                Tile::COIN => "Coin",
                Tile::LADDER => "Ladder",
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
}

impl UITiles {
    pub fn atlas_index(&self) -> Option<AtlasIndex> {
        Some(match *self {
            UITiles::EDITBUTTON => 22,
            UITiles::PLAYBUTTON => 21,
            UITiles::DELETEBUTTON => 20,
            UITiles::BACKBUTTON => 19,
            UITiles::SAVEBUTTON => 31,
            UITiles::REPLAYBUTTON => 47,
            UITiles::DISCARDBUTTON => 15,
        })
    }
}
