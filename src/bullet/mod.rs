use bevy::{prelude::*, utils::Duration};

use crate::{consts::{WINDOW_SIZE,POSITION_Z}, events::TransformEvent, state::AppState};

static BULLET_SPEED: f32 = 300.0;
static BULLET_SIZE: Size = Size {
    width: 5.0,
    height: 20.0
};
static BULLET_INITIAL_DELAY: f32 = 0.1;
static BULLET_DELAY: f32 = 0.5;

#[derive(Component)]
struct Bullet {
    timer: Timer,
    position: Vec2,
    rotation: f32,
    speed: Vec2,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(BULLET_INITIAL_DELAY, false),
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
            speed: Vec2::new(0.0, 0.0),
        }
    }
}

fn add_bullet(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 0.0, 1.0, 0.8),
                custom_size: Some(Vec2::new(BULLET_SIZE.width, BULLET_SIZE.height)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, POSITION_Z.bullet),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(Bullet::default());
}

fn handle_transform(
    mut transform_events: EventReader<TransformEvent>,
    mut query: Query<&mut Bullet>
) {
    for transform_event in transform_events.iter() {
        let mut bullet = query.single_mut();
        bullet.position.x = transform_event.position.x;
        bullet.position.y = transform_event.position.y;
        bullet.rotation = transform_event.rotation;
    }
}

fn update_bullet(
    mouse_button_input: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut query: Query<(&mut Bullet, &mut Visibility, &mut Transform)>,
) {
    let delta_seconds = time.delta_seconds();

    let just_pressed = mouse_button_input.just_pressed(MouseButton::Left);

    for (mut bullet, mut visibility, mut transform) in query.iter_mut() {
        if bullet.timer.tick(time.delta()).finished() {
            if just_pressed {
                visibility.is_visible = true;
                let rotation = bullet.rotation;
                bullet.speed = Vec2::new(-BULLET_SPEED * rotation.sin(), BULLET_SPEED * rotation.cos());
                transform.rotation = Quat::from_rotation_z(rotation);
                transform.translation.x = bullet.position.x;
                transform.translation.y = bullet.position.y;

                if bullet.timer.duration() == Duration::from_secs_f32(BULLET_INITIAL_DELAY) {
                    bullet.timer.set_duration(Duration::from_secs_f32(BULLET_DELAY));
                }

                bullet.timer.reset();
            }
        }

        if visibility.is_visible == false {
            return;
        }

        if transform.translation.x < -WINDOW_SIZE.width / 2.0 - BULLET_SIZE.width
            || transform.translation.x > WINDOW_SIZE.width / 2.0 + BULLET_SIZE.width
            || transform.translation.y < -WINDOW_SIZE.height / 2.0 - BULLET_SIZE.height
            || transform.translation.y > WINDOW_SIZE.height / 2.0 + BULLET_SIZE.height  {
            visibility.is_visible = false;
        } else {
            transform.translation.x += bullet.speed.x * delta_seconds;
            transform.translation.y += bullet.speed.y * delta_seconds;
        }
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Main).with_system(add_bullet))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(handle_transform))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(update_bullet));
    }
}