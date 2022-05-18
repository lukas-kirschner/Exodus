#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Directions {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
pub enum FromDirection {
    FROMNORTH,
    FROMSOUTH,
    FROMEAST,
    FROMWEST,
}

impl From<Directions> for FromDirection {
    ///
    /// Convert a Direction into the corresponding FromDirection, i.e. the opposite direction
    fn from(direction: Directions) -> Self {
        match direction {
            Directions::NORTH => { FromDirection::FROMSOUTH }
            Directions::SOUTH => { FromDirection::FROMNORTH }
            Directions::EAST => { FromDirection::FROMWEST }
            Directions::WEST => { FromDirection::FROMEAST }
        }
    }
}

impl From<FromDirection> for Directions {
    ///
    /// Convert a FromDirection into the corresponding Direction, i.e. the opposite direction
    fn from(fromdirection: FromDirection) -> Self {
        match fromdirection {
            FromDirection::FROMNORTH => { Directions::SOUTH }
            FromDirection::FROMSOUTH => { Directions::NORTH }
            FromDirection::FROMEAST => { Directions::WEST }
            FromDirection::FROMWEST => { Directions::EAST }
        }
    }
}