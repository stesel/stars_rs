use crate::{
    consts::POSITION_Z,
    events::AddExplosionEvent,
    state::{AppState, LoaderState},
    utils,
};
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
struct ExplosionAnimationTimer(Timer);

#[derive(Component)]
struct Explosion;

static EXPLOSION_SIZE: utils::Size = utils::Size {
    width: 128.0,
    height: 128.0,
};

impl Default for Explosion {
    fn default() -> Self {
        Self
    }
}

fn add_explosion(
    mut add_explosion_events: EventReader<AddExplosionEvent>,
    mut commands: Commands,
    loader: Res<LoaderState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for add_explosion_event in add_explosion_events.iter() {
        let position = add_explosion_event.position;

        let texture_handle = loader.explosion_image.clone();
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(EXPLOSION_SIZE.width, EXPLOSION_SIZE.height),
            5,
            1,
            Option::None,
            Option::None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        commands
            .spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite { ..default() },
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_xyz(position.x, position.y, POSITION_Z.explosion),
                ..default()
            })
            .insert(ExplosionAnimationTimer(Timer::from_seconds(
                0.08,
                TimerMode::Repeating,
            )))
            .insert(Explosion::default());
    }
}

fn animate(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        Entity,
        &mut ExplosionAnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
    mut commands: Commands,
) {
    for (entity, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            if sprite.index < texture_atlas.len() - 1 {
                sprite.index = sprite.index + 1;
            } else {
                timer.reset();
                commands.entity(entity).despawn();
            }
        }
    }
}

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Main).with_system(add_explosion))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(animate));
    }
}
