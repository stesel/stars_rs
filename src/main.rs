mod window;
mod camera;
mod background;
mod character;
mod rain;
mod aim;
mod fps;

mod consts;
mod events;

use bevy::{prelude::*};

fn main() {  
    App::new()
        .add_plugin(window::WindowPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(background::BackgroundPlugin)
        .add_plugin(character::CharacterPlugin)
        .add_plugin(rain::RainPlugin)
        .add_plugin(aim::AimPlugin)
        .add_plugin(fps::FpsTextPlugin)
        .run();
}
