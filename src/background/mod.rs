use bevy::{prelude::*};

use crate::{events::TransformEvent, consts::POSITION_Z, state::{AppState, LoaderState}};

#[derive(Component)]
struct Background;

static SCALE: f32 = 1.05;

fn add_background(mut commands: Commands, loader: ResMut<LoaderState>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: loader.background_image.clone(),
            transform: Transform::from_xyz(0.0, 0.0, POSITION_Z.background).with_scale(Vec3::splat(SCALE)),
            ..default()
        })
        .insert(Background);
}

fn move_background(
    mut transform_events: EventReader<TransformEvent>,
    mut query: Query<&mut Transform, With<Background>>
) {
    for position_event in transform_events.iter() {
        let mut transform = query.single_mut();
        
        transform.translation.x = (position_event.position.x * (1.0 - SCALE)) / 2.0;
        transform.translation.y = (position_event.position.y * (1.0 - SCALE)) / 2.0;
    }
}


pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Main).with_system(add_background))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(move_background));
    }
}
