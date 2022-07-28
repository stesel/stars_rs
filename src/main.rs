mod window;
mod app;
mod camera;
mod state;
mod loader;
mod menu;
mod background;
mod bullet;
mod enemies;
mod character;
mod collision;
mod rain;
mod aim;
mod fps;

mod consts;
mod utils;
mod events;

use bevy::{prelude::*};

fn main() {
    App::new()
        .add_plugin(window::WindowPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(app::AppPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(state::StatePlugin)
        .add_plugin(loader::LoaderPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(background::BackgroundPlugin)
        .add_plugin(bullet::BulletPlugin)
        .add_plugin(enemies::EnemiesPlugin)
        .add_plugin(character::CharacterPlugin)
        .add_plugin(collision::CollisionPlugin)
        .add_plugin(rain::RainPlugin)
        .add_plugin(aim::AimPlugin)
        .add_plugin(fps::FpsTextPlugin)
        .run();
}
