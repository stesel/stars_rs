use crate::{
    consts::POSITION_Z,
    state::{AppState, LoaderState},
};
use bevy::{asset::LoadState, prelude::*};
use std::f32::consts::PI;

#[derive(Component)]
struct LoaderSprite;

fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(251.0, 226.0, 196.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, POSITION_Z.loader),
            ..default()
        })
        .insert(LoaderSprite);

    let background_image: Handle<Image> = asset_server.load("background.png");
    let enemy_image: Handle<Image> = asset_server.load("enemy.png");
    let character_image: Handle<Image> = asset_server.load("character.png");
    let explosion_image: Handle<Image> = asset_server.load("explosion.png");
    let aim_image: Handle<Image> = asset_server.load("aim.png");
    let button_sound: Handle<AudioSource> = asset_server.load("button.ogg");
    let bullet_sound: Handle<AudioSource> = asset_server.load("bullet.ogg");
    let collision_sound: Handle<AudioSource> = asset_server.load("collision.ogg");
    let explosion_sound: Handle<AudioSource> = asset_server.load("explosion.ogg");
    let font: Handle<Font> = asset_server.load("FiraMono-Medium.ttf");

    commands.insert_resource(LoaderState {
        background_image,
        enemy_image,
        character_image,
        explosion_image,
        aim_image,
        button_sound,
        bullet_sound,
        collision_sound,
        explosion_sound,
        font,
    });
}

fn loading(
    loader: Res<LoaderState>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<AppState>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<LoaderSprite>>,
) {
    match asset_server.get_group_load_state(loader.ids()) {
        LoadState::Loading => {
            let mut transform = query.single_mut();
            transform.rotation = Quat::from_rotation_z(time.elapsed_seconds().cos() * PI);
        }
        LoadState::Loaded => {
            state.set(AppState::Menu).unwrap();
        }
        _ => {}
    }
}

fn loaded(mut commands: Commands, query: Query<(Entity, &LoaderSprite)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load))
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(loading))
            .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(loaded));
    }
}
