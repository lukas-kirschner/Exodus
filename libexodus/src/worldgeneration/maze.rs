use crate::tiles::Tile;
use crate::world::GameWorld;
use crate::worldgeneration::WorldGenerationError::InvalidBorderWidth;
use crate::worldgeneration::{WorldGenerationAlgorithm, WorldGenerationError};
use std::cmp::min;

#[derive(Clone)]
pub(super) struct Border {
    pub width: u32,
    pub height: u32,
    pub color: Tile,
    pub border_width: u32,
}

impl Border {
    /// Generate a new map with border.
    /// All Values must be already validated before calling this function.
    fn generate_validated(&self) -> Result<GameWorld, WorldGenerationError> {
        let mut ret = GameWorld::new(self.width as usize, self.height as usize);
        if self.border_width == 0 || self.border_width > min(self.width, self.height) / 2 {
            return Err(InvalidBorderWidth {
                border_width: self.border_width,
            });
        }
        for dist in 0..self.border_width {
            for x in 0..self.width {
                ret.set(
                    x as usize,
                    (self.height - dist - 1) as usize,
                    self.color.clone(),
                );
                ret.set(x as usize, dist as usize, self.color.clone());
            }
            for y in self.border_width..self.height - self.border_width {
                ret.set(
                    (self.width - dist - 1) as usize,
                    y as usize,
                    self.color.clone(),
                );
                ret.set(dist as usize, y as usize, self.color.clone());
            }
        }
        Ok(ret)
    }
}

impl WorldGenerationAlgorithm for Border {
    fn generate(&self) -> Result<GameWorld, WorldGenerationError> {
        match (self.width, self.height) {
            (0, _) => Err(WorldGenerationError::InvalidWidth { width: self.width }),
            (_, 0) => Err(WorldGenerationError::InvalidHeight {
                height: self.height,
            }),
            (w_u32, h_u32) => match usize::try_from(w_u32) {
                Ok(..) => match usize::try_from(h_u32) {
                    Ok(..) => self.generate_validated(),
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
        let algo = build_generator(
            WorldGenerationKind::Border {
                width: 1,
                color: Tile::WALL,
            },
            2,
            2,
        );
        let result = algo.generate();
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_map_content_matches("WW\nWW", &map);
    }
    #[test]
    fn test_generate_3x3_map() {
        let algo = build_generator(
            WorldGenerationKind::Border {
                width: 1,
                color: Tile::WALL,
            },
            3,
            3,
        );
        let result = algo.generate();
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_map_content_matches("WWW\nW W\nWWW", &map);
    }
    #[test]
    fn test_generate_4x4_map() {
        let algo = build_generator(
            WorldGenerationKind::Border {
                width: 1,
                color: Tile::WALL,
            },
            4,
            4,
        );
        let result = algo.generate();
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_map_content_matches("WWWW\nW  W\nW  W\nWWWW", &map);
    }
    #[test]
    fn test_generate_10x3_map() {
        let algo = build_generator(
            WorldGenerationKind::Border {
                width: 1,
                color: Tile::WALLNATURE,
            },
            10,
            3,
        );
        let result = algo.generate();
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_map_content_matches(
            "
NNNNNNNNNN
N        N
NNNNNNNNNN
",
            &map,
        );
    }
    #[test]
    fn test_generate_10x5_map() {
        let algo = build_generator(
            WorldGenerationKind::Border {
                width: 2,
                color: Tile::WALLNATURE,
            },
            10,
            5,
        );
        let result = algo.generate();
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_map_content_matches(
            "
NNNNNNNNNN
NNNNNNNNNN
NN      NN
NNNNNNNNNN
NNNNNNNNNN
",
            &map,
        );
    }
}
