mod camera;
mod character;
mod window;

use bevy::{prelude::*};

fn main() {  
    App::new()
        .add_plugin(window::WindowPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(character::CharacterPlugin)
        .run();
}
