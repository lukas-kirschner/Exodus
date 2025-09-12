use crate::World;
use crate::textures::egui_textures::EguiButtonTextures;
use crate::ui::UIMARGIN;
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Align, Layout, RichText, Ui};
use libexodus::highscores::highscore::Highscore;
use libexodus::player::Player;
use libexodus::tiles::Tile;

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
    pub crystals: usize,
    pub coins: i32,
    pub moves: usize,
    pub keys: usize,
}

impl FromWorld for Scoreboard {
    fn from_world(_: &mut World) -> Self {
        Scoreboard {
            crystals: 0,
            coins: 0,
            moves: 0,
            keys: 0,
        }
    }
}

impl Scoreboard {
    pub fn new(coins: i32, crystals: usize, moves: usize, keys: usize) -> Self {
        Scoreboard {
            coins,
            crystals,
            moves,
            keys,
        }
    }
}

impl From<&Highscore> for Scoreboard {
    fn from(value: &Highscore) -> Self {
        Scoreboard::new(value.coins() as i32, 0usize, value.moves() as usize, 0usize)
    }
}
/// Create a EGUI Scoreboard Label that shows a previous highscore
pub fn egui_highscore_label(
    ui: &mut Ui,
    scoreboard: &Option<Scoreboard>,
    textures: &EguiButtonTextures,
) {
    ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
        ui.style_mut().spacing.item_spacing = (0.0, 0.0).into();
        ui.style_mut().spacing.indent = 0.0;
        match scoreboard {
            None => {
                ui.label(
                    RichText::new(t!("map_selection_screen.no_highscore"))
                        .text_style(egui::TextStyle::Name("Highscore".into())),
                );
            },
            Some(score) => {
                let h = ui
                    .label(
                        RichText::new(t!("map_selection_screen.highscore_heading"))
                            .text_style(egui::TextStyle::Name("Highscore".into())),
                    )
                    .rect
                    .height();
                ui.add_space(UIMARGIN);
                ui.image(SizedTexture::new(
                    textures.textures[&Player::atlas_index_right()].0,
                    (h, h),
                ));
                ui.label(
                    RichText::new(t!(
                        "map_selection_screen.moves_fmt",
                        moves = &score.moves.to_string()
                    ))
                    .text_style(egui::TextStyle::Name("Highscore".into())),
                );
                ui.add_space(UIMARGIN);
                ui.image(SizedTexture::new(
                    textures.textures[&Tile::COIN.atlas_index().unwrap()].0,
                    (h, h),
                ));
                ui.label(
                    RichText::new(t!(
                        "map_selection_screen.coins_fmt",
                        coins = &score.coins.to_string()
                    ))
                    .text_style(egui::TextStyle::Name("Highscore".into())),
                );
            },
        }
    });
}
