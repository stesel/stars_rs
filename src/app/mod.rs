use bevy::{prelude::*, window::close_on_esc};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(close_on_esc);
    }
}
