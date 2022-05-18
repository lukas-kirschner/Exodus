use bevy::prelude::*;
use libexodus::tiles::Tile;
use libexodus::world::GameWorld;

///
/// A wall that the player cannot pass through and that will not interact with the player in any way
#[derive(Component)]
pub struct Wall<> {
    // pub tile: &'a Tile,
}

///
/// A wrapper around a GameWorld
#[derive(Component)]
pub struct MapWrapper<> {
    pub world: GameWorld,
}