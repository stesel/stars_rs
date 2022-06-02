use bevy::{prelude::*};

use crate::{consts::{WINDOW_SIZE,POSITION_Z}, utils::random_in_range, state::AppState};

static SPEED_X: f32 = 10.0;
static MIN_SPEED_Y: f32 = 130.0;
static MAX_SPEED_Y: f32 = 40.0;
static DROP_SIZE: Size = Size {
    width: 2.0,
    height: 8.0
};
static DROP_COUNT: u32 = 200;

#[derive(Component)]
struct Drop {
    speed: Vec2,
}

impl Default for Drop {
    fn default() -> Self {
        Self {
            speed: Vec2::new(SPEED_X, random_in_range(MIN_SPEED_Y, MAX_SPEED_Y)),
        }
    }
}

fn get_initial_position() -> Vec2 {
    let x = random_in_range(-WINDOW_SIZE.width / 2.0 - DROP_SIZE.width, WINDOW_SIZE.width / 2.0);
    let y = random_in_range(-WINDOW_SIZE.height / 2.0, (WINDOW_SIZE.height + DROP_SIZE.height) / 2.0);
    Vec2::new(x, y)
}

fn get_restart_position() -> Vec2 {
    let x = random_in_range(-WINDOW_SIZE.width / 2.0 - DROP_SIZE.width, WINDOW_SIZE.width / 2.0);
    let y = (WINDOW_SIZE.height + DROP_SIZE.height) / 2.0;
    Vec2::new(x, y)
}

fn add_rain(mut commands: Commands) {
    for _ in 0..DROP_COUNT {
        let position = get_initial_position();
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(200.0, 200.0, 200.0, 0.05),
                    custom_size: Some(Vec2::new(DROP_SIZE.width, DROP_SIZE.height)),
                    ..default()
                },
                transform: Transform::from_xyz(position.x, position.y, POSITION_Z.rain).with_rotation(Quat::from_rotation_z(0.15)),
                ..default()
            })
            .insert(Drop::default());
    }
}

fn update_rain(
    time: Res<Time>,
    mut query: Query<(&Drop, &mut Transform)>,
) {
    let delta_seconds = time.delta_seconds();

    for (drop, mut transform) in query.iter_mut() {
        if transform.translation.x > (WINDOW_SIZE.width + DROP_SIZE.width) / 2.0 || transform.translation.y < -(WINDOW_SIZE.height + DROP_SIZE.height) / 2.0 {
            let position = get_restart_position();
            transform.translation.x = position.x;
            transform.translation.y = position.y;
        } else {
            transform.translation.x += drop.speed.x * delta_seconds;
            transform.translation.y -= drop.speed.y * delta_seconds;
        }
    }
}

pub struct RainPlugin;

impl Plugin for RainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Main).with_system(add_rain))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(update_rain));
    }
}