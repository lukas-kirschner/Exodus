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

    pub fn int_target_from_direction(&self, player_x: f32, player_y: f32) -> (i32, i32) {
        match self.direction() {
            Directions::NORTH => { (player_x as i32, player_y.floor() as i32 + 1) }
            Directions::SOUTH => { (player_x as i32, player_y.ceil() as i32 - 1) }
            Directions::EAST => { (player_x.floor() as i32 + 1, player_y as i32) }
            Directions::WEST => { (player_x.ceil() as i32 - 1, player_y as i32) }
        }
    }
}