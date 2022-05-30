use bevy::{prelude::*};

use crate::{consts::{WINDOW_SIZE, POSITION_Z}, state::{AppState, LoaderState}};

#[derive(Component)]
struct Aim;

fn setup(mut commands: Commands, loader: Res<LoaderState>,) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: loader.aim_image.clone(),
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
            .add_system_set(SystemSet::on_enter(AppState::Main).with_system(setup))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(follow_mouse));
    }
}
