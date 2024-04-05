use crate::tiles::Tile;
use crate::world::GameWorld;
use crate::worldgeneration::{WorldGenerationAlgorithm, WorldGenerationError};

#[derive(Clone)]
pub(super) struct Filled {
    pub width: u32,
    pub height: u32,
    pub color: Tile,
}

impl WorldGenerationAlgorithm for Filled {
    fn generate(&self) -> Result<GameWorld, WorldGenerationError> {
        match (self.width, self.height) {
            (0, _) => Err(WorldGenerationError::InvalidWidth { width: self.width }),
            (_, 0) => Err(WorldGenerationError::InvalidHeight {
                height: self.height,
            }),
            (w_u32, h_u32) => match usize::try_from(w_u32) {
                Ok(w) => match usize::try_from(h_u32) {
                    Ok(h) => {
                        let mut world = GameWorld::new(w, h);
                        world.fill(&self.color);
                        Ok(world)
                    },
                    Err(e) => Err(WorldGenerationError::HeightOutOfRange { e }),
                },
                Err(e) => Err(WorldGenerationError::WidthOutOfRange { e }),
            },
        }
    }

    fn width(&self) -> &u32 {
        &self.width
    }

    fn height(&self) -> &u32 {
        &self.height
    }
}

#[cfg(test)]
mod tests {
    use crate::tiles::Tile;
    use crate::worldgeneration::tests::assert_map_content_matches;
    use crate::worldgeneration::{build_generator, WorldGenerationKind};

    #[test]
    fn test_generate_2x2_map() {
        let algo = build_generator(WorldGenerationKind::Filled { color: Tile::WALL }, 2, 2, 0);
        let result = algo.generate();
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_map_content_matches("WW\nWW", &map);
    }
    #[test]
    fn test_generate_10x2_map() {
        let algo = build_generator(WorldGenerationKind::Filled { color: Tile::WALL }, 10, 2, 0);
        let result = algo.generate();
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_map_content_matches("WWWWWWWWWW\nWWWWWWWWWW", &map);
    }
}
