use bevy::{prelude::*, utils::Duration};

use crate::{
    consts::{POSITION_Z, WINDOW_SIZE},
    events::TransformEvent,
    state::{AppState, LoaderState},
    utils::{BoundingRect, GetBoundingRect, IsActive},
};

static BULLET_SPEED: f32 = 300.0;
static BULLET_SIZE: Size = Size {
    width: 5.0,
    height: 20.0,
};
static BULLET_INITIAL_DELAY: f32 = 0.1;
static BULLET_DELAY: f32 = 0.5;

#[derive(Component)]
pub struct Bullet {
    timer: Timer,
    initial_position: Vec2,
    position: Vec2,
    rotation: f32,
    speed: Vec2,
    active: bool,
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

impl IsActive for Bullet {
    fn get_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(BULLET_INITIAL_DELAY, false),
            initial_position: Vec2::new(0.0, 0.0),
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
            speed: Vec2::new(0.0, 0.0),
            active: false,
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
    mut query: Query<&mut Bullet>,
) {
    for transform_event in transform_events.iter() {
        let mut bullet = query.single_mut();
        bullet.initial_position.x = transform_event.position.x;
        bullet.initial_position.y = transform_event.position.y;
        bullet.rotation = transform_event.rotation;
    }
}

fn update_bullet(
    mouse_button_input: Res<Input<MouseButton>>,
    time: Res<Time>,
    loader: ResMut<LoaderState>,
    audio: Res<Audio>,
    mut query: Query<(&mut Bullet, &mut Transform)>,
) {
    let delta_seconds = time.delta_seconds();

    let just_pressed = mouse_button_input.just_pressed(MouseButton::Left);

    for (mut bullet, mut transform) in query.iter_mut() {
        if bullet.timer.tick(time.delta()).finished() {
            if just_pressed {
                bullet.active = true;
                let rotation = bullet.rotation;
                bullet.speed = Vec2::new(
                    -BULLET_SPEED * rotation.sin(),
                    BULLET_SPEED * rotation.cos(),
                );
                transform.rotation = Quat::from_rotation_z(rotation);
                bullet.position.x = bullet.initial_position.x;
                bullet.position.y = bullet.initial_position.y;

                if bullet.timer.duration() == Duration::from_secs_f32(BULLET_INITIAL_DELAY) {
                    bullet
                        .timer
                        .set_duration(Duration::from_secs_f32(BULLET_DELAY));
                }

                bullet.timer.reset();

                audio.play(loader.bullet_sound.clone());
            }
        }

        if bullet.active == false {
            return;
        }

        if bullet.position.x < -WINDOW_SIZE.width / 2.0 - BULLET_SIZE.width
            || bullet.position.x > WINDOW_SIZE.width / 2.0 + BULLET_SIZE.width
            || bullet.position.y < -WINDOW_SIZE.height / 2.0 - BULLET_SIZE.height
            || bullet.position.y > WINDOW_SIZE.height / 2.0 + BULLET_SIZE.height
        {
            bullet.active = false;
        } else {
            bullet.position.x += bullet.speed.x * delta_seconds;
            bullet.position.y += bullet.speed.y * delta_seconds;
        }
    }
}

fn bullet_changed(mut query: Query<(&Bullet, &mut Visibility, &mut Transform), Changed<Bullet>>) {
    for (bullet, mut visibility, mut transform) in query.iter_mut() {
        visibility.is_visible = bullet.active;
        transform.translation.x = bullet.position.x;
        transform.translation.y = bullet.position.y;
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Main).with_system(add_bullet))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(handle_transform))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(update_bullet))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(bullet_changed));
    }
}
