/// A single high score record
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Highscore {
    num_moves: u32,
    num_coins: u32,
}

impl Highscore {
    pub fn new(num_moves: u32, num_coins: u32) -> Self {
        Highscore {
            num_moves,
            num_coins,
        }
    }
    pub fn moves(&self) -> u32 {
        self.num_moves
    }
    pub fn coins(&self) -> u32 {
        self.num_coins
    }
}


#[cfg(test)]
mod tests {
    use crate::highscores::highscore::Highscore;

    #[test]
    fn test_highscore_getters() {
        let highscore = Highscore::new(69, 42069);
        assert_eq!(69, highscore.moves());
        assert_eq!(42069, highscore.coins());
    }

    #[test]
    fn test_highscore_constructor() {
        let highscore = Highscore::new(0, 111);
        let highscore2 = Highscore {
            num_moves: 0,
            num_coins: 111,
        };
        assert_eq!(highscore, highscore2);
    }
}