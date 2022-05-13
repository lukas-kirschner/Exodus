use crate::tiles;
use crate::tiles::Tile;

#[derive(Clone)]
pub struct GameWorld {
    data: Vec<Vec<Tile>>,
}

impl GameWorld {
    pub fn new(width: usize, height: usize) -> Self {
        assert! {width > 0};
        assert! {height > 0};
        Self {
            data: vec![vec![tiles::AIR; height]; width],
        }
    }
    ///
    /// Set the tile at the given coordinate to the given value.
    pub fn set(&mut self, x: usize, y: usize, tile: &Tile) -> &mut Self {
        self.data[x][y] = tile.clone();
        self
    }
    ///
    /// Get the tile at the given location.
    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.data.get(x)?.get(y)
    }
    ///
    /// Fill the whole map with the given tile and delete everything else.
    pub fn fill(&mut self, tile: &Tile) -> &mut Self {
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                self.set(i, j, tile);
            }
        }
        self
    }
    ///
    /// Get the width of this world.
    ///
    /// ```rust
    /// use libexodus::world::GameWorld;
    /// let world = GameWorld::new(69,1337);
    /// assert_eq!(69, world.width());
    /// ```
    pub fn width(&self) -> usize {
        self.data.len()
    }
    ///
    /// Get the height of this world.
    ///
    /// ```rust
    /// use libexodus::world::GameWorld;
    /// let world = GameWorld::new(69,1337);
    /// assert_eq!(1337, world.height());
    /// ```
    pub fn height(&self) -> usize {
        self.data[0].len()
    }
}