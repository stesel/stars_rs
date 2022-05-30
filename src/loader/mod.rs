use std::f32::consts::PI;

use bevy::{prelude::*, asset::{LoadState}};

use crate::state::{AppState, LoaderState};

#[derive(Component)]
struct LoaderSprite;

fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    let loader_entity = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(251.0, 226.0, 196.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            ..default()
        }).insert(LoaderSprite).id();

    let background_image: Handle<Image> = asset_server.load("background.png");
    let character_image: Handle<Image> = asset_server.load("character.png");
    let aim_image: Handle<Image> = asset_server.load("aim.png");
    let font: Handle<Font> = asset_server.load("FiraMono-Medium.ttf");

    commands.insert_resource(LoaderState {
        loader_entity: loader_entity,
        background_image: background_image,
        character_image: character_image,
        aim_image: aim_image,
        font: font,
    });
}

fn loading(
    loader: Res<LoaderState>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<AppState>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<LoaderSprite>>
) {
    match asset_server.get_group_load_state(loader.ids()) {
        LoadState::Loading => {
            let mut transform = query.single_mut();
            transform.rotation = Quat::from_rotation_z((time.seconds_since_startup().cos() as f32) * PI);
        },
        LoadState::Loaded => {
            state.set(AppState::Main).unwrap();
        },
        _ => {},
    }
}

fn loaded(mut commands: Commands, loader: Res<LoaderState>) {
    commands.entity(loader.loader_entity).despawn();
}

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load))
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(loading))
            .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(loaded));
    }
}
