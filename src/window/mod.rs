use bevy::{prelude::*};

use crate::consts::WINDOW_SIZE;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WindowDescriptor {
                width: WINDOW_SIZE.width,
                height: WINDOW_SIZE.height,
                title: "stars_rs".to_string(),
                ..default()
            })
            .insert_resource(ClearColor(Color::DARK_GRAY));
    }
}
