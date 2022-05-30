use bevy::{prelude::*, input::system::exit_on_esc_system};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(exit_on_esc_system);
    }
}