use bevy::{prelude::*, diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}};

#[derive(Component)]
struct FpsText;

fn add_fps_text(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();

    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS:".to_string(),
                        style: TextStyle {
                            font: asset_server.load("FiraMono-Medium.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("FiraMono-Medium.ttf"),
                            font_size: 20.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                alignment: TextAlignment {
                    horizontal: HorizontalAlign::Left,
                    vertical: VerticalAlign::Top,
                }
            },
            transform: Transform::from_xyz(
                -window.width() / 2.0,
                window.height() / 2.0,
                1.0,
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