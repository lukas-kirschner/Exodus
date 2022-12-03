use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use libexodus::tiles::{Tile, TileKind};
use crate::AppState;
use crate::game::constants::{COLLECTIBLE_PICKUP_DISTANCE, PICKUP_ITEM_ASCEND_SPEED, PICKUP_ITEM_DECAY_SPEED, PICKUP_ITEM_ZOOM_SPEED};
use crate::game::player::PlayerComponent;
use crate::game::scoreboard::Scoreboard;
use crate::util::dist_2d;


#[derive(Component)]
pub struct PickupItem;

/// Handler that takes care of animating and despawning the picked up item.
pub fn pickup_item_animation(
    mut commands: Commands,
    mut dead_players: Query<(&mut TextureAtlasSprite, &mut Transform, Entity), With<PickupItem>>,
    time: Res<Time>,
) {
    for (mut sprite, mut transform, entity) in dead_players.iter_mut() {
        let new_a: f32 = sprite.color.a() - (PICKUP_ITEM_DECAY_SPEED * time.delta_seconds());
        if new_a <= 0.0 {
            // The player has fully decayed and can be despawned
            commands.entity(entity).despawn_recursive();
            return;
        }
        sprite.color.set_a(new_a);
        transform.translation.y += PICKUP_ITEM_ASCEND_SPEED * time.delta_seconds();
        transform.scale += Vec3::splat(PICKUP_ITEM_ZOOM_SPEED * time.delta_seconds());
    }
}

pub struct PickupItemPlugin;

impl Plugin for PickupItemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CollectibleCollectedEvent>()
            // Collision Handlers
            .add_system_set(SystemSet::on_update(AppState::Playing)
                .with_system(setup_collectible_event::<CoinWrapper>).after("player_movement")
            )
            .add_system_set(SystemSet::on_update(AppState::Playing)
                .with_system(setup_collectible_event::<KeyWrapper>).after("player_movement")
            )
            .add_system_set(SystemSet::on_update(AppState::Playing)
                .with_system(setup_collectible_event::<CollectibleWrapper>).after("player_movement")
            )
            // Event Handlers
            .add_system_set(SystemSet::on_update(AppState::Playing)
                .with_system(collectible_collected_event).after("player_movement")
            )
            .add_system_set(SystemSet::on_update(AppState::Playing)
                .with_system(pickup_item_animation).after("player_movement")
            )
        ;
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

struct CollectibleCollectedEvent {
    player: Entity,
    action: CollectibleAction,
    collectible: Entity,
}

fn setup_collectible_event<WrapperType: Component + CollectibleWrapperTrait>(
    mut commands: Commands,
    coin_query: Query<(Entity, &Transform, &WrapperType)>,
    players: Query<(&PlayerComponent, &Transform, Entity)>,
    mut ev_collectible_collected: EventWriter<CollectibleCollectedEvent>,
) {
    for (_player, player_trans, player_entity) in players.iter() {
        let player_pos: Vec3 = player_trans.translation;
        for (coin_entity, coin_trans, coin) in coin_query.iter() {
            let coin_pos: Vec3 = coin_trans.translation;
            let dist = dist_2d(&player_pos, &coin_pos);
            if dist <= COLLECTIBLE_PICKUP_DISTANCE {
                // Fire event
                ev_collectible_collected.send(CollectibleCollectedEvent {
                    player: player_entity,
                    action: coin.get_action(),
                    collectible: coin_entity,
                });
                // Clearing the collectible here, because the event might be triggered multiple times if we clear it in the event handler
                commands.entity(coin_entity).remove::<WrapperType>().insert(PickupItem);
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
        match ev.action {
            CollectibleAction::AddCoins { coins } => scoreboard.coins += coins as i32,
            CollectibleAction::AddKeys { keys } => scoreboard.keys += keys as usize,
            CollectibleAction::None => (),
        };
    }
}

/// Insert the appropriate wrapper when a tile is set up in the game world.
/// Must be called from the game board setup routine
pub fn insert_wrappers(
    tile: &Tile,
    bundle: &mut EntityCommands,
) {
    match tile.kind() {
        TileKind::COIN => {
            bundle.insert(CoinWrapper { coin_value: 1 });
        }
        TileKind::KEY => {
            bundle.insert(KeyWrapper);
        }
        TileKind::COLLECTIBLE => {
            bundle.insert(CollectibleWrapper);
        }
        _ => {}
    }
}