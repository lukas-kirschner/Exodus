use crate::world::GameWorld;
use crate::worldgeneration::{WorldGenerationAlgorithm, WorldGenerationError};

#[derive(Clone)]
pub(super) struct Empty {
    pub width: u32,
    pub height: u32,
}

impl WorldGenerationAlgorithm for Empty {
    fn generate(&self) -> Result<GameWorld, WorldGenerationError> {
        match (self.width, self.height) {
            (0, _) => Err(WorldGenerationError::InvalidWidth { width: self.width }),
            (_, 0) => Err(WorldGenerationError::InvalidHeight {
                height: self.height,
            }),
            (w_u32, h_u32) => match usize::try_from(w_u32) {
                Ok(w) => match usize::try_from(h_u32) {
                    Ok(h) => Ok(GameWorld::new(w, h)),
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
    use crate::worldgeneration::tests::assert_map_content_matches;
    use crate::worldgeneration::{WorldGenerationKind, build_generator};

    #[test]
    fn test_generate_2x2_map() {
        let algo = build_generator(WorldGenerationKind::Empty, 2, 2, 0);
        let result = algo.generate();
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_map_content_matches("  \n  ", &map);
    }
}
