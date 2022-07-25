use bevy::{prelude::*};

use crate::{consts::{WINDOW_SIZE,POSITION_Z}, utils::random_in_range, state::{AppState, LoaderState}};

static MIN_SPEED: f32 = -200.0;
static MAX_SPEED: f32 = 200.0;
static ENEMY_SIZE: Size = Size {
    width: 128.0,
    height: 128.0
};
static ENEMY_COUNT: u32 = 20;

#[derive(Component, Deref, DerefMut)]
struct EnemyAnimationTimer(Timer);

#[derive(Component)]
struct Enemy {
    speed: Vec2,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            speed: Vec2::new(random_in_range(MIN_SPEED, MAX_SPEED), random_in_range(MIN_SPEED, MAX_SPEED)),
        }
    }
}

fn get_initial_position() -> Vec2 {
    Vec2::new(0.0, 0.0)
}

fn get_restart_position() -> Vec2 {
    Vec2::new(0.0, 0.0)
}

fn add_enemies(
    mut commands: Commands,
    loader: Res<LoaderState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = loader.enemy_image.clone();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(ENEMY_SIZE.width, ENEMY_SIZE.height), 5, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for _ in 0..ENEMY_COUNT {
        let position = get_initial_position();
        let enemy = Enemy::default();
        let rotation_z = -enemy.speed.x.atan2(enemy.speed.y);

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_xyz(position.x, position.y, POSITION_Z.enemy).with_rotation(Quat::from_rotation_z(rotation_z)),
                ..default()
            })
            .insert(EnemyAnimationTimer(Timer::from_seconds(0.07, true)))
            .insert(enemy);
    }
}

fn animate(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut EnemyAnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.len();
        }
    }

}

fn update_enemies(
    time: Res<Time>,
    mut query: Query<(&Enemy, &mut Transform)>,
) {
    let delta_seconds = time.delta_seconds();

    for (enemy, mut transform) in query.iter_mut() {
        if transform.translation.x > WINDOW_SIZE.width / 2.0 + ENEMY_SIZE.width ||
            transform.translation.x < -WINDOW_SIZE.width / 2.0 - ENEMY_SIZE.width ||
            transform.translation.y > WINDOW_SIZE.height / 2.0 + ENEMY_SIZE.height ||
            transform.translation.y < -WINDOW_SIZE.height / 2.0 - ENEMY_SIZE.height
        {
            let position = get_restart_position();
            transform.translation.x = position.x;
            transform.translation.y = position.y;
        } else {
            transform.translation.x += enemy.speed.x * delta_seconds;
            transform.translation.y += enemy.speed.y * delta_seconds;
        }
    }
}

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Main).with_system(add_enemies))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(animate))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(update_enemies));
    }
}