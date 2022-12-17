use std::fmt::{Display, Formatter};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumIter, EnumCountMacro)]
pub enum Tileset {
    TinyPlatformQuestTiles,
    Classic,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<(u8, u8, u8)> for Color {
    fn from(c: (u8, u8, u8)) -> Self {
        Color {
            r: c.0,
            g: c.1,
            b: c.2,
        }
    }
}

impl Display for Color {
    /// Get the HTML Color in #RRGGBB format
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl Tileset {
    pub fn background_color(&self) -> Color {
        match self {
            Tileset::TinyPlatformQuestTiles => (0x90, 0x90, 0x90),
            Tileset::Classic => (0xff, 0xff, 0xff),
        }.into()
    }
    pub fn texture_size(&self) -> usize {
        match self {
            Tileset::TinyPlatformQuestTiles => 16,
            Tileset::Classic => 16,
        }
    }
}

impl Display for Tileset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tileset::TinyPlatformQuestTiles => write!(f, "Tiny Platform Quest Tiles"),
            Tileset::Classic => write!(f, "Classic"),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::tilesets::Color;

    #[test]
    fn test_color_from_tuple() {
        let color = Color::from((255, 255, 255));
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 255);
    }

    #[test]
    fn test_color_to_html() {
        let color = Color::from((255, 255, 255));
        assert_eq!("#FFFFFF", color.to_string());
    }
}