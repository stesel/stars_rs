use bevy::{prelude::*, diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}};

use crate::consts::WINDOW_SIZE;

#[derive(Component)]
struct FpsText;

fn add_fps_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let style = TextStyle {
        font: asset_server.load("FiraMono-Medium.ttf"),
        font_size: 16.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS:".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            color: Color::GOLD,
                            ..style
                        },
                    },
                ],
                alignment: TextAlignment {
                    horizontal: HorizontalAlign::Left,
                    vertical: VerticalAlign::Top,
                }
            },
            transform: Transform::from_xyz(
                -WINDOW_SIZE.width / 2.0,
                WINDOW_SIZE.height / 2.0,
                4.0,
            ),
            ..default()
        })
        .insert(FpsText);
}

fn update_fps_text(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}

pub struct FpsTextPlugin;

impl Plugin for FpsTextPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(add_fps_text)
            .add_system(update_fps_text);
    }
}