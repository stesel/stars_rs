mod aim;
mod app;
mod background;
mod bullet;
mod camera;
mod character;
mod collision;
mod enemies;
mod events;
mod explosion;
mod fps;
mod loader;
mod menu;
mod rain;
mod state;
mod window;

mod consts;
mod utils;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugin(window::WindowPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(app::AppPlugin)
        .add_plugin(events::EventsPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(state::StatePlugin)
        .add_plugin(loader::LoaderPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(background::BackgroundPlugin)
        .add_plugin(bullet::BulletPlugin)
        .add_plugin(enemies::EnemiesPlugin)
        .add_plugin(character::CharacterPlugin)
        .add_plugin(explosion::ExplosionPlugin)
        .add_plugin(collision::CollisionPlugin)
        .add_plugin(rain::RainPlugin)
        .add_plugin(aim::AimPlugin)
        .add_plugin(fps::FpsTextPlugin)
        .run();
}
