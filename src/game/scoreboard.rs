use crate::game::constants::FONT_SIZE_HIGHSCORE;
use crate::World;
use bevy::prelude::*;
use bevy_egui::egui::{RichText, Ui};
use libexodus::highscores::highscore::Highscore;

#[derive(Resource, Clone, Debug)]
pub enum GameOverState {
    /// The game was lost, i.e., the player died losing all lives without reaching the exit
    Lost,
    /// The player won the game with the given scoreboard
    Won { score: Scoreboard },
    // /// The player won the game as part of the campaign
    // WON_CAMPAIGN { score: Scoreboard },
}

/// Event that is triggered when a game is won or lost
#[derive(Event)]
pub struct GameOverEvent {
    pub state: GameOverState,
}
#[derive(Resource, Clone, Debug)]
pub struct Scoreboard {
    pub coins: i32,
    pub moves: usize,
    pub keys: usize,
}

impl FromWorld for Scoreboard {
    fn from_world(_: &mut World) -> Self {
        Scoreboard {
            coins: 0,
            moves: 0,
            keys: 0,
        }
    }
}

impl Scoreboard {
    pub fn new(coins: i32, moves: usize, keys: usize) -> Self {
        Scoreboard { coins, moves, keys }
    }
}

impl From<&Highscore> for Scoreboard {
    fn from(value: &Highscore) -> Self {
        Scoreboard::new(value.coins() as i32, value.moves() as usize, 0usize)
    }
}
/// Create a EGUI Scoreboard Label that shows a previous highscore
pub fn egui_highscore_label(ui: &mut Ui, scoreboard: &Option<Scoreboard>) {
    match scoreboard {
        None => {
            ui.label(
                RichText::new(t!("map_selection_screen.no_highscore")).size(FONT_SIZE_HIGHSCORE),
            );
        },
        Some(score) => {
            ui.label(
                RichText::new(t!("map_selection_screen.highscore_heading"))
                    .size(FONT_SIZE_HIGHSCORE),
            );
            ui.label(RichText::new(" ").size(FONT_SIZE_HIGHSCORE));
            ui.label(
                RichText::new(t!(
                    "map_selection_screen.moves_fmt",
                    moves = &score.moves.to_string()
                ))
                .size(14.),
            );
            ui.label(RichText::new(" ").size(FONT_SIZE_HIGHSCORE));
            ui.label(
                RichText::new(t!(
                    "map_selection_screen.coins_fmt",
                    coins = &score.coins.to_string()
                ))
                .size(FONT_SIZE_HIGHSCORE),
            );
        },
    }
}
