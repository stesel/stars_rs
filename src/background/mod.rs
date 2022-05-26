use bevy::{prelude::*};

use crate::events;
use events::PositionEvent;

#[derive(Component)]
struct Background;

static SCALE: f32 = 1.05;

fn add_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("background.png"),
            transform: Transform::from_scale(Vec3::splat(SCALE)),
            ..default()
        })
        .insert(Background);
}

fn move_background(
    mut position_events: EventReader<PositionEvent>,
    mut query: Query<&mut Transform, With<Background>>
) {
    for position_event in position_events.iter() {
        let mut transform = query.single_mut();
        
        transform.translation.x = (position_event.position.x * (1.0 - SCALE)) / 2.0;
        transform.translation.y = (position_event.position.y * (1.0 - SCALE)) / 2.0;
    }
}


pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(add_background)
            .add_system(move_background);
    }
}