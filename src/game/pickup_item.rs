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
use libexodus::tiles::{CollectibleKind, Tile, TileKind};

#[derive(Component)]
pub struct PickupItem;

pub struct PickupItemPlugin;

impl Plugin for PickupItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollectibleCollectedEvent>()
            // Collision Handlers
            .add_systems(Update,setup_collectible_event.run_if(in_state(AppState::Playing)).after(AppLabels::PlayerMovement))
            // Event Handlers
            .add_systems(Update,collectible_collected_event.run_if(in_state(AppState::Playing)).after(AppLabels::PlayerMovement));
    }
}
/// A wrapper for Collectibles (Arrows,...)
#[derive(Component)]
pub struct CollectibleWrapper {
    kind: CollectibleKind,
}

#[derive(Event)]
struct CollectibleCollectedEvent {
    player: Entity,
    action: CollectibleKind,
    collectible: Entity,
}
/// Set up a Collectible Event for the given Collectible type.
fn setup_collectible_event(
    mut commands: Commands,
    mut coin_query: Query<(Entity, &mut Transform, &CollectibleWrapper)>,
    players: Query<(&PlayerComponent, &Transform, Entity), Without<CollectibleWrapper>>,
    mut ev_collectible_collected: EventWriter<CollectibleCollectedEvent>,
) {
    for (_player, player_trans, player_entity) in players.iter() {
        let player_pos: Vec3 = player_trans.translation;
        for (collectible_entity, mut coin_trans, collectible) in coin_query.iter_mut() {
            let coin_pos: &mut Vec3 = &mut coin_trans.translation;
            let dist = dist_2d(&player_pos, coin_pos);
            if dist <= COLLECTIBLE_PICKUP_DISTANCE {
                // Fire event
                ev_collectible_collected.send(CollectibleCollectedEvent {
                    player: player_entity,
                    action: collectible.kind.clone(),
                    collectible: collectible_entity,
                });
                // Set the z position of the animation to the z position of the player, such that
                // animations will be rendered behind the player, but above solid tiles:
                coin_pos.z = PLAYER_Z - 0.1;
                // Clearing the collectible here, because the event might be triggered multiple times if we clear it in the event handler
                commands
                    .entity(collectible_entity)
                    .remove::<CollectibleWrapper>()
                    .insert(AnimatedActionSprite::from_ascend_and_zoom(
                        PICKUP_ITEM_DECAY_SPEED,
                        PICKUP_ITEM_ASCEND_SPEED,
                        PICKUP_ITEM_ZOOM_SPEED,
                        AnimatedSpriteAction::None,
                    ));
            }
        }
    }
}

/// Event that despawns the collected collectible and executes the associated action
fn collectible_collected_event(
    mut ev_collectible_collected: EventReader<CollectibleCollectedEvent>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    for ev in ev_collectible_collected.read() {
        let _player: Entity = ev.player;
        let _collectible: Entity = ev.collectible;
        match ev.action {
            CollectibleKind::Decorative => (),
            CollectibleKind::Coins { amount } => scoreboard.coins += amount as i32,
            CollectibleKind::Keys { amount } => scoreboard.keys += amount,
            CollectibleKind::StarCrystals { amount } => scoreboard.crystals += amount,
        };
    }
}

/// Insert the appropriate wrapper when a tile is set up in the game world.
/// Must be called from the game board setup routine
/// TODO Implement support for collecting more than one crystal or key?
pub fn insert_wrappers(tile: &Tile, bundle: &mut EntityCommands) {
    match tile.kind() {
        TileKind::COLLECTIBLE { kind } => bundle.insert(CollectibleWrapper { kind }),
        _ => bundle,
    };
}
