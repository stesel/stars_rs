use bevy::prelude::*;

use crate::consts::WINDOW_SIZE;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            canvas: Some("#bevy".to_owned()),
            width: WINDOW_SIZE.width,
            height: WINDOW_SIZE.height,
            title: "stars_rs".to_owned(),
            cursor_visible: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::DARK_GRAY));
    }
}
