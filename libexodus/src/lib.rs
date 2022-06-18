pub mod world;
pub mod tiles;
pub mod player;
pub mod movement;
pub mod directions;
pub mod directories;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
