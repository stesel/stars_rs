use crate::{
    consts::{POSITION_Z, WINDOW_SIZE},
    enemies::EnemyCount,
    events::EnemiesLeftEvent,
    state::{AppState, LoaderState},
};
use bevy::prelude::*;

#[derive(Component)]
struct EnemiesLeftText;

fn add_enemies_left_text(
    enemy_count_query: Query<&EnemyCount>,
    mut commands: Commands,
    loader: Res<LoaderState>,
) {
    let style = TextStyle {
        font: loader.font.clone(),
        font_size: 16.0,
        color: Color::WHITE,
    };

    let enemy_count = enemy_count_query.single();

    commands
        .spawn(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Enemies:".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: enemy_count.count.to_string(),
                        style: TextStyle {
                            color: Color::TOMATO,
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
                WINDOW_SIZE.height / 2.0 - 16.0,
                POSITION_Z.enemies_left,
            ),
            ..default()
        })
        .insert(EnemiesLeftText);
}

fn update_enemies_left_text(
    mut enemies_left_events: EventReader<EnemiesLeftEvent>,
    mut query: Query<&mut Text, With<EnemiesLeftText>>,
) {
    for enemies_left_event in enemies_left_events.iter() {
        for mut text in query.iter_mut() {
            text.sections[1].value = format!("{}", enemies_left_event.enemies_left);
        }
    }
}

pub struct EnemiesLeftTextPlugin;

impl Plugin for EnemiesLeftTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Main).with_system(add_enemies_left_text))
            .add_system_set(
                SystemSet::on_update(AppState::Main).with_system(update_enemies_left_text),
            );
    }
}
