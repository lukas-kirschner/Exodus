pub mod config;
pub mod directions;
pub mod directories;
pub mod movement;
pub mod player;
pub mod tiles;
pub mod tilesets;
pub mod world;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
