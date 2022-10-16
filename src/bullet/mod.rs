use bevy::{prelude::*, utils::Duration};

use crate::{
    consts::{POSITION_Z, WINDOW_SIZE},
    events::TransformEvent,
    state::{AppState, LoaderState},
    utils::{BoundingRect, GetBoundingRect},
};

static BULLET_SPEED: f32 = 300.0;
static BULLET_SIZE: Size = Size {
    width: 5.0,
    height: 20.0,
};
static BULLET_INITIAL_DELAY: f32 = 0.1;
static BULLET_DELAY: f32 = 0.5;

#[derive(Component)]
pub struct BulletPool {
    timer: Timer,
    position: Vec2,
    rotation: f32,
}

#[derive(Component)]
pub struct Bullet {
    position: Vec2,
    speed: Vec2,
}

impl GetBoundingRect for Bullet {
    fn get_bounding_rect(&self) -> BoundingRect {
        BoundingRect {
            x: self.position.x,
            y: self.position.y,
            width: BULLET_SIZE.width,
            height: BULLET_SIZE.width,
        }
    }
}

fn setup_bullet_pool(mut commands: Commands) {
    commands.spawn().insert(BulletPool {
        timer: Timer::from_seconds(BULLET_INITIAL_DELAY, false),
        position: Vec2::new(0.0, 0.0),
        rotation: 0.0,
    });
}

fn handle_transform(
    mut transform_events: EventReader<TransformEvent>,
    mut query: Query<&mut BulletPool>,
) {
    for transform_event in transform_events.iter() {
        let mut bullet_pool = query.single_mut();
        bullet_pool.position.x = transform_event.position.x;
        bullet_pool.position.y = transform_event.position.y;
        bullet_pool.rotation = transform_event.rotation;
    }
}

fn update_bullet_pool(
    time: Res<Time>,
    mouse_button_input: Res<Input<MouseButton>>,
    loader: ResMut<LoaderState>,
    audio: Res<Audio>,
    mut query: Query<&mut BulletPool>,
    mut commands: Commands,
) {
    let just_pressed = mouse_button_input.just_pressed(MouseButton::Left);

    let mut bullet_pool = query.single_mut();
    if bullet_pool.timer.tick(time.delta()).finished() {
        if just_pressed {
            let bullet_position = Vec2::new(bullet_pool.position.x, bullet_pool.position.y);
            let bullet_rotation = bullet_pool.rotation;
            let bullet_speed = Vec2::new(
                -BULLET_SPEED * bullet_rotation.sin(),
                BULLET_SPEED * bullet_rotation.cos(),
            );

            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1.0, 0.0, 1.0, 0.8),
                        custom_size: Some(Vec2::new(BULLET_SIZE.width, BULLET_SIZE.height)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        bullet_position.x,
                        bullet_position.y,
                        POSITION_Z.bullet,
                    )
                    .with_rotation(Quat::from_rotation_z(bullet_rotation)),
                    ..default()
                })
                .insert(Bullet {
                    position: bullet_position,
                    speed: bullet_speed,
                });

            if bullet_pool.timer.duration() == Duration::from_secs_f32(BULLET_INITIAL_DELAY) {
                bullet_pool
                    .timer
                    .set_duration(Duration::from_secs_f32(BULLET_DELAY));
            }

            bullet_pool.timer.reset();

            audio.play(loader.bullet_sound.clone());
        }
    }
}

fn update_bullet(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Bullet, &mut Transform)>,
    mut commands: Commands,
) {
    let delta_seconds = time.delta_seconds();

    for (entity, mut bullet, mut transform) in query.iter_mut() {
        if bullet.position.x < -WINDOW_SIZE.width / 2.0 - BULLET_SIZE.width
            || bullet.position.x > WINDOW_SIZE.width / 2.0 + BULLET_SIZE.width
            || bullet.position.y < -WINDOW_SIZE.height / 2.0 - BULLET_SIZE.height
            || bullet.position.y > WINDOW_SIZE.height / 2.0 + BULLET_SIZE.height
        {
            commands.entity(entity).despawn();
        } else {
            bullet.position.x += bullet.speed.x * delta_seconds;
            bullet.position.y += bullet.speed.y * delta_seconds;

            transform.translation.x = bullet.position.x;
            transform.translation.y = bullet.position.y;
        }
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Main).with_system(setup_bullet_pool))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(update_bullet_pool))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(update_bullet))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(handle_transform));
    }
}
