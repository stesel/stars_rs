use crate::{
    character::CharacterLifes,
    consts::{POSITION_Z, WINDOW_SIZE},
    events::CharacterLifesEvent,
    state::{AppState, LoaderState},
};
use bevy::prelude::*;

#[derive(Component)]
struct CharacterLifesText;

fn add_character_lifes_text(
    mut commands: Commands,
    character_lifes_query: Query<&CharacterLifes>,
    loader: Res<LoaderState>,
) {
    let style = TextStyle {
        font: loader.font.clone(),
        font_size: 16.0,
        color: Color::WHITE,
    };

    let character_lifes = character_lifes_query.single();

    commands
        .spawn(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Lifes:".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: character_lifes.lifes.to_string(),
                        style: TextStyle {
                            color: Color::TURQUOISE,
                            ..style
                        },
                    },
                ],
                alignment: TextAlignment {
                    horizontal: HorizontalAlign::Left,
                    vertical: VerticalAlign::Top,
                },
            },
            transform: Transform::from_xyz(
                -WINDOW_SIZE.width / 2.0,
                WINDOW_SIZE.height / 2.0 - 16.0 * 2.0,
                POSITION_Z.character_lifes,
            ),
            ..default()
        })
        .insert(CharacterLifesText);
}

fn update_character_lifes_text(
    mut character_lifes_events: EventReader<CharacterLifesEvent>,
    mut query: Query<&mut Text, With<CharacterLifesText>>,
) {
    for character_lifes_event in character_lifes_events.iter() {
        for mut text in query.iter_mut() {
            text.sections[1].value = format!("{}", character_lifes_event.character_lifes);
        }
    }
}

pub struct CharacterLifesTextPlugin;

impl Plugin for CharacterLifesTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Main).with_system(add_character_lifes_text),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Main).with_system(update_character_lifes_text),
        );
    }
}
