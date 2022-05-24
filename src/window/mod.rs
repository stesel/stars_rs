use bevy::{prelude::*};

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            width: 600.0,
            height: 500.0,
            title: "stars_rs".to_string(),
            ..default()
        });
    }
}