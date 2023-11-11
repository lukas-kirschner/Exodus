use crate::animation::animated_action_sprite::{AnimatedActionSprite, AnimatedSpriteAction};
use crate::game::constants::{
    COLLECTIBLE_PICKUP_DISTANCE, PICKUP_ITEM_ASCEND_SPEED, PICKUP_ITEM_DECAY_SPEED,
    PICKUP_ITEM_ZOOM_SPEED, PLAYER_Z,
};
use crate::game::player::PlayerComponent;
use crate::game::scoreboard::Scoreboard;
use crate::util::dist_2d;
use crate::{AppLabels, AppState};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use libexodus::tiles::{Tile, TileKind};

#[derive(Component)]
pub struct PickupItem;

pub struct PickupItemPlugin;

impl Plugin for PickupItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollectibleCollectedEvent>()
            // Collision Handlers
            .add_systems(Update,setup_collectible_event::<CoinWrapper>.run_if(in_state(AppState::Playing)).after(AppLabels::PlayerMovement))
            .add_systems(Update,setup_collectible_event::<KeyWrapper>.run_if(in_state(AppState::Playing)).after(AppLabels::PlayerMovement))
            .add_systems(Update,setup_collectible_event::<CollectibleWrapper>.run_if(in_state(AppState::Playing)).after(AppLabels::PlayerMovement))
            // Event Handlers
            .add_systems(Update,collectible_collected_event.run_if(in_state(AppState::Playing)).after(AppLabels::PlayerMovement));
    }
}

trait CollectibleWrapperTrait {
    fn get_action(&self) -> CollectibleAction;
}

/// A wrapper for coins
#[derive(Component)]
pub struct CoinWrapper {
    /// The value of this coin, i.e. the score a player gets for collecting the coin
    pub coin_value: i32,
}

impl CollectibleWrapperTrait for CoinWrapper {
    fn get_action(&self) -> CollectibleAction {
        CollectibleAction::AddCoins { coins: 1 }
    }
}

/// A wrapper for keys
#[derive(Component)]
pub struct KeyWrapper;

impl CollectibleWrapperTrait for KeyWrapper {
    fn get_action(&self) -> CollectibleAction {
        CollectibleAction::AddKeys { keys: 1 }
    }
}

/// A wrapper for Collectibles (Arrows,...)
#[derive(Component)]
pub struct CollectibleWrapper;

impl CollectibleWrapperTrait for CollectibleWrapper {
    fn get_action(&self) -> CollectibleAction {
        CollectibleAction::None
    }
}

enum CollectibleAction {
    AddCoins { coins: u32 },
    AddKeys { keys: u32 },
    None,
}
#[derive(Event)]
struct CollectibleCollectedEvent {
    player: Entity,
    action: CollectibleAction,
    collectible: Entity,
}
/// Set up a Collectible Event for the given Collectible type.
fn setup_collectible_event<WrapperType: Component + CollectibleWrapperTrait>(
    mut commands: Commands,
    mut coin_query: Query<(Entity, &mut Transform, &WrapperType)>,
    players: Query<(&PlayerComponent, &Transform, Entity), Without<WrapperType>>,
    mut ev_collectible_collected: EventWriter<CollectibleCollectedEvent>,
) {
    for (_player, player_trans, player_entity) in players.iter() {
        let player_pos: Vec3 = player_trans.translation;
        for (coin_entity, mut coin_trans, coin) in coin_query.iter_mut() {
            let coin_pos: &mut Vec3 = &mut coin_trans.translation;
            let dist = dist_2d(&player_pos, coin_pos);
            if dist <= COLLECTIBLE_PICKUP_DISTANCE {
                // Fire event
                ev_collectible_collected.send(CollectibleCollectedEvent {
                    player: player_entity,
                    action: coin.get_action(),
                    collectible: coin_entity,
                });
                // Set the z position of the animation to the z position of the player, such that
                // animations will be rendered behind the player, but above solid tiles:
                coin_pos.z = PLAYER_Z - 0.1;
                // Clearing the collectible here, because the event might be triggered multiple times if we clear it in the event handler
                commands.entity(coin_entity).remove::<WrapperType>().insert(
                    AnimatedActionSprite::from_ascend_and_zoom(
                        PICKUP_ITEM_DECAY_SPEED,
                        PICKUP_ITEM_ASCEND_SPEED,
                        PICKUP_ITEM_ZOOM_SPEED,
                        AnimatedSpriteAction::None,
                    ),
                );
            }
        }
    }
}

/// Event that despawns the collected collectible and executes the associated action
fn collectible_collected_event(
    mut ev_collectible_collected: EventReader<CollectibleCollectedEvent>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    for ev in ev_collectible_collected.iter() {
        let _player: Entity = ev.player;
        let _collectible: Entity = ev.collectible;
        match ev.action {
            CollectibleAction::AddCoins { coins } => scoreboard.coins += coins as i32,
            CollectibleAction::AddKeys { keys } => scoreboard.keys += keys as usize,
            CollectibleAction::None => (),
        };
    }
}

/// Insert the appropriate wrapper when a tile is set up in the game world.
/// Must be called from the game board setup routine
pub fn insert_wrappers(tile: &Tile, bundle: &mut EntityCommands) {
    match tile.kind() {
        TileKind::COIN => {
            bundle.insert(CoinWrapper { coin_value: 1 });
        },
        TileKind::KEY => {
            bundle.insert(KeyWrapper);
        },
        TileKind::COLLECTIBLE => {
            bundle.insert(CollectibleWrapper);
        },
        _ => {},
    }
}
