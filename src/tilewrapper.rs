use bevy::prelude::*;
use libexodus::world::GameWorld;
use libexodus::world::presets;

///
/// A wall that the player cannot pass through and that will not interact with the player in any way
#[derive(Component)]
pub struct Wall<> {
    // pub tile: &'a Tile,
}

///
/// A wrapper around a GameWorld
pub struct MapWrapper<> {
    pub world: GameWorld,
}

impl FromWorld for MapWrapper {
    fn from_world(_: &mut World) -> Self {
        MapWrapper {
            world: presets::map_with_border(24, 10),
        }
    }
}

impl MapWrapper {
    pub fn set_world(&mut self, world: GameWorld) {
        self.world = world;
    }
}