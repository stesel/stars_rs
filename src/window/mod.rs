use crate::consts::WINDOW_SIZE;
use bevy::prelude::*;

pub fn get_window_pluggin() -> WindowPlugin {
    WindowPlugin {
        window: WindowDescriptor {
            canvas: Some("#bevy".to_owned()),
            width: WINDOW_SIZE.width,
            height: WINDOW_SIZE.height,
            title: "stars_rs".to_owned(),
            cursor_visible: false,
            ..default()
        },
        ..default()
    }
}
