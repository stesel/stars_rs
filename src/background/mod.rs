use bevy::prelude::*;

use crate::{
    consts::POSITION_Z,
    events::TransformEvent,
    state::{AppState, LoaderState},
};

#[derive(Component)]
struct Background;

static SCALE: f32 = 1.05;

fn add_background(mut commands: Commands, loader: ResMut<LoaderState>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: loader.background_image.clone(),
            transform: Transform::from_xyz(0.0, 0.0, POSITION_Z.background)
                .with_scale(Vec3::splat(SCALE)),
            ..default()
        })
        .insert(Background);
}

fn move_background(
    mut transform_events: EventReader<TransformEvent>,
    mut query: Query<&mut Transform, With<Background>>,
) {
    for transform_event in transform_events.iter() {
        let mut transform = query.single_mut();

        transform.translation.x = (transform_event.position.x * (1.0 - SCALE)) / 2.0;
        transform.translation.y = (transform_event.position.y * (1.0 - SCALE)) / 2.0;
    }
}

fn add_tint(mut query: Query<&mut Sprite, With<Background>>) {
    let mut sprite = query.single_mut();
    sprite.color = Color::rgba(0.7, 0.7, 0.7, 0.8);
}

fn remove_tint(mut query: Query<&mut Sprite, With<Background>>) {
    let mut sprite = query.single_mut();
    sprite.color = Color::default();
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_exit(AppState::Loading).with_system(add_background))
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(add_tint))
            .add_system_set(SystemSet::on_enter(AppState::Main).with_system(remove_tint))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(move_background));
    }
}
