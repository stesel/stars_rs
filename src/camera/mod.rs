use bevy::{prelude::*};

fn add_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_camera);
    }
}
