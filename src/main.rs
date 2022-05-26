mod camera;
mod character;
mod window;
mod fps;

use bevy::{prelude::*};

fn main() {  
    App::new()
        .add_plugin(window::WindowPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(character::CharacterPlugin)
        .add_plugin(fps::FpsTextPlugin)
        .run();
}
