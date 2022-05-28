use bevy::{prelude::*};

use crate::consts::{WINDOW_SIZE, POSITION_Z};

#[derive(Component)]
struct Aim;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("aim.png"),
        transform: Transform::from_xyz(0.0, 0.0, POSITION_Z.aim),
        ..default()
    })
    .insert(Aim);
}

fn follow_mouse(mut cursor_moved_events: EventReader<CursorMoved>, mut query: Query<&mut Transform, With<Aim>>) {
    for event in cursor_moved_events.iter() {
        let mut transform = query.single_mut();
        transform.translation.x = event.position.x - WINDOW_SIZE.width / 2.0;
        transform.translation.y = event.position.y - WINDOW_SIZE.height / 2.0;
    }
}

pub struct AimPlugin;

impl Plugin for AimPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(follow_mouse);
    }
}