use bevy::{prelude::*};

use crate::{consts::{WINDOW_SIZE,POSITION_Z}, utils::{random_in_range,random_in_rect_edge,BoundingRect,GetBoundingRect,Position}, state::{AppState, LoaderState}};

static MIN_SPEED: f32 = 100.0;
static MAX_SPEED: f32 = 200.0;
pub static ENEMY_SIZE: Size = Size {
    width: 128.0,
    height: 128.0
};
static ENEMY_EDGE_POSITION: Position = Position {
    x: (WINDOW_SIZE.width + ENEMY_SIZE.width) / 2.0,
    y: (WINDOW_SIZE.height + ENEMY_SIZE.height) / 2.0,
};
static ENEMY_COUNT: u32 = 5;

#[derive(Component, Deref, DerefMut)]
struct EnemyAnimationTimer(Timer);

#[derive(Component)]
pub struct Enemy {
    position: Vec2,
    speed: Vec2,
}

impl Enemy {
    fn new(position: Vec2, speed: Vec2) -> Self {
        Self {
            position: position,
            speed: speed,
        }
    }
}

impl Default for Enemy {
    fn default() -> Self {
        Self::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0))
    }
}

impl GetBoundingRect for Enemy {
    fn get_bounding_rect(&self) -> BoundingRect {
        BoundingRect {
            x: self.position.x,
            y: self.position.y,
            width: ENEMY_SIZE.width,
            height: ENEMY_SIZE.height,
        }
    }
}

fn get_position() -> Vec2 {
    random_in_rect_edge(
        -ENEMY_EDGE_POSITION.x,
        ENEMY_EDGE_POSITION.x,
        ENEMY_EDGE_POSITION.y,
        -ENEMY_EDGE_POSITION.y,
    )
}

fn get_speed(position: &Vec2) -> Vec2 {
    if position.x == -ENEMY_EDGE_POSITION.x  {
        Vec2::new(random_in_range(MIN_SPEED, MAX_SPEED), random_in_range(-MAX_SPEED, MAX_SPEED))
    } else if position.x == ENEMY_EDGE_POSITION.x {
        Vec2::new(random_in_range(-MAX_SPEED, -MIN_SPEED), random_in_range(-MAX_SPEED, MAX_SPEED))
    } else if position.y == ENEMY_EDGE_POSITION.y {
        Vec2::new(random_in_range(-MAX_SPEED, MAX_SPEED), random_in_range(-MAX_SPEED, -MIN_SPEED))
    } else {
        Vec2::new(random_in_range(-MAX_SPEED, MAX_SPEED), random_in_range(MIN_SPEED, MAX_SPEED))
    }
}

fn get_rotation_z(speed: &Vec2) -> f32 {
    -speed.x.atan2(speed.y)
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
        let position = get_position();
        let speed = get_speed(&position);
        let rotation_z = get_rotation_z(&speed);

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_xyz(position.x, position.y, POSITION_Z.enemy).with_rotation(Quat::from_rotation_z(rotation_z)),
                ..default()
            })
            .insert(EnemyAnimationTimer(Timer::from_seconds(0.07, true)))
            .insert(Enemy::new(position, speed));
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
    mut query: Query<(&mut Enemy, &mut Transform)>,
) {
    let delta_seconds = time.delta_seconds();

    for (mut enemy, mut transform) in query.iter_mut() {
        if transform.translation.x > ENEMY_EDGE_POSITION.x ||
            transform.translation.x < -ENEMY_EDGE_POSITION.x ||
            transform.translation.y > ENEMY_EDGE_POSITION.y ||
            transform.translation.y < -ENEMY_EDGE_POSITION.y
        {
            let position = get_position();
            let speed = get_speed(&position);
            let rotation_z = get_rotation_z(&speed);

            enemy.position.x = position.x;
            enemy.position.y = position.y;
            enemy.speed.x = speed.x;
            enemy.speed.y = speed.y;
    
            transform.translation.x = position.x;
            transform.translation.y = position.y;
            transform.rotation = Quat::from_rotation_z(rotation_z);
        } else {
            enemy.position.x += enemy.speed.x * delta_seconds;
            enemy.position.y += enemy.speed.y * delta_seconds;
        }
    }
}

fn position_changed(
    mut query: Query<(&Enemy, &mut Transform), Changed<Enemy>>
) {
    for (enemy, mut transform) in query.iter_mut() {
        transform.translation.x = enemy.position.x;
        transform.translation.y = enemy.position.y;
    }
}

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Main).with_system(add_enemies))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(animate))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(update_enemies))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(position_changed));
    }
}