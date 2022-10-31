use bevy::prelude::*;
use crate::AppState;
use crate::game::constants::UI_FONT_SIZE;
use crate::game::scoreboard::Scoreboard;

// The font has been taken from https://ggbot.itch.io/public-pixel-font (CC0 Public Domain)

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Scoreboard>()
            .add_system_set(SystemSet::on_enter(AppState::Playing)
                .with_system(setup_game_ui).after("world").after("player").after("camera").label("ui")
            )
            .add_system_set(SystemSet::on_update(AppState::Playing)
                .with_system(scoreboard_ui_system)
            )
        ;
    }
}

#[derive(Component)]
pub struct ScoreboardUICounter {}

pub fn setup_game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    //Spawn UI Camera
    // commands.spawn_bundle(UiCameraBundle::default());

    //Initialize Coin Counter
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Coins: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/PublicPixel.ttf"),
                        font_size: UI_FONT_SIZE,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/PublicPixel.ttf"),
                        font_size: UI_FONT_SIZE,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                },
                TextSection {
                    value: " Moves: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/PublicPixel.ttf"),
                        font_size: UI_FONT_SIZE,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/PublicPixel.ttf"),
                        font_size: UI_FONT_SIZE,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }).insert(ScoreboardUICounter {});
}

pub fn scoreboard_ui_system(
    scoreboard: Res<Scoreboard>,
    mut textobjects: Query<&mut Text, With<ScoreboardUICounter>>,
) {
    for mut text in textobjects.iter_mut() {
        text.sections[1].value = scoreboard.coins.to_string();
        text.sections[3].value = scoreboard.moves.to_string();
    }
}