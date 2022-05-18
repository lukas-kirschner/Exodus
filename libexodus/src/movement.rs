use crate::directions::Directions;

#[derive(Copy, Clone, PartialEq)]
pub struct Movement {
    pub velocity: (f32, f32),
    pub target: (i32, i32),
}

impl Movement {
    pub fn direction(&self) -> Directions {
        if self.velocity.0 > 0. {
            return Directions::EAST;
        } else if self.velocity.0 < 0. {
            return Directions::WEST;
        }
        if self.velocity.1 > 0. {
            return Directions::NORTH;
        } else if self.velocity.1 < 0. {
            return Directions::SOUTH;
        }
        panic!("Encountered movement with velocity of {:?}", self.velocity);
    }
}