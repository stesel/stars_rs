use bevy::{prelude::*};

fn add_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            width: 600.0,
            height: 500.0,
            title: "stars_rs".to_string(),
            ..default()
        });
        app.add_startup_system(add_camera);
    }
}