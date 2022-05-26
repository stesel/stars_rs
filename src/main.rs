mod window;
mod camera;
mod character;
mod background;
mod fps;

mod events;

use bevy::{prelude::*};

fn main() {  
    App::new()
        .add_plugin(window::WindowPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(character::CharacterPlugin)
        .add_plugin(background::BackgroundPlugin)
        .add_plugin(fps::FpsTextPlugin)
        .run();
}
