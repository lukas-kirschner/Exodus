use crate::tiles::wall;
use crate::world::GameWorld;

pub fn map_with_border(width: usize, height: usize) -> GameWorld {
    let mut ret: GameWorld = GameWorld::new(width, height);
    if width <= 2 || height <= 2 {
        ret.fill(&wall());
        return ret;
    }
    for i in 0..width {
        ret.set(i, 0, wall());
        ret.set(i, height - 1, wall());
    }
    for i in 1..height - 1 {
        ret.set(0, i, wall());
        ret.set(width - 1, i, wall());
    }
    ret
}
