use bevy::{input::keyboard::KeyCode, prelude::*};

use crate::{
    consts::{POSITION_Z, WINDOW_SIZE},
    events::TransformEvent,
    state::{AppState, LoaderState},
    utils::{BoundingRect, GetBoundingRect, IsActive, SetSpeed},
};

#[derive(Component, Deref, DerefMut)]
struct CharacterAnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct CharacterActive(bool);
#[derive(Component, Deref, DerefMut)]
struct CharacterInactiveTimer(Timer);

#[derive(Component)]
pub struct Character {
    position: Vec2,
    speed: Vec2,
    mouse: Vec2,
    active: bool,
}

static CHARACTER_SIZE: Size = Size {
    width: 128.0,
    height: 128.0,
};
static MAX_SPEED: f32 = 150.0;
static FRICTION: f32 = 0.96;

static INACTIVE_DURATION: f32 = 3.0;

const CHARACTER_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 1.0);

impl Default for Character {
    fn default() -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            speed: Vec2::new(0.0, 0.0),
            mouse: Vec2::new(WINDOW_SIZE.width / 2.0, WINDOW_SIZE.height / 2.0),
            active: true,
        }
    }
}

impl GetBoundingRect for Character {
    fn get_bounding_rect(&self) -> BoundingRect {
        BoundingRect {
            x: self.position.x,
            y: self.position.y,
            width: CHARACTER_SIZE.width,
            height: CHARACTER_SIZE.height,
        }
    }
}

impl SetSpeed for Character {
    fn set_speed(&mut self, speed: Vec2) {
        self.speed = speed;
    }
}

impl IsActive for Character {
    fn get_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

impl IsActive for CharacterActive {
    fn get_active(&self) -> bool {
        self.0
    }

    fn set_active(&mut self, active: bool) {
        self.0 = active;
    }
}

fn setup(
    mut commands: Commands,
    loader: Res<LoaderState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = loader.character_image.clone();
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(CHARACTER_SIZE.width, CHARACTER_SIZE.height),
        5,
        1,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: CHARACTER_COLOR,
                ..default()
            },
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(0.0, 0.0, POSITION_Z.character),
            ..default()
        })
        .insert(CharacterAnimationTimer(Timer::from_seconds(0.05, true)))
        .insert(CharacterActive(true))
        .insert(CharacterInactiveTimer(Timer::from_seconds(
            INACTIVE_DURATION,
            false,
        )))
        .insert(Character::default());
}

fn animate(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut CharacterAnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.len();
        }
    }
}

fn transform_changed(
    mut position_events: EventWriter<TransformEvent>,
    mut query: Query<(&Character, &mut Transform), Changed<Character>>,
) {
    for (character, mut transform) in query.iter_mut() {
        transform.translation.x = character.position.x;
        transform.translation.y = character.position.y;

        let delta_x = character.mouse.x - character.position.x - WINDOW_SIZE.width / 2.0;
        let delta_y = character.mouse.y - character.position.y - WINDOW_SIZE.height / 2.0;
        let rotation_z = -delta_x.atan2(delta_y);

        transform.rotation = Quat::from_rotation_z(rotation_z);

        position_events.send(TransformEvent {
            position: Vec2::new(character.position.x, character.position.y),
            rotation: rotation_z,
        });
    }
}

fn active_changed(
    mut query: Query<
        (
            &CharacterActive,
            &mut CharacterInactiveTimer,
            &mut TextureAtlasSprite,
        ),
        Changed<CharacterActive>,
    >,
) {
    for (character_active, mut inactive_timer, mut sprite) in query.iter_mut() {
        if character_active.get_active() == false {
            inactive_timer.reset();
            sprite.color.set_a(0.8);
        }
    }
}

fn inactive_timer_changed(
    time: Res<Time>,
    mut query: Query<(
        &mut CharacterActive,
        &mut CharacterInactiveTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut character_active, mut inactive_timer, mut sprite) in query.iter_mut() {
        if character_active.get_active() == false
            && inactive_timer.tick(time.delta()).just_finished()
        {
            character_active.set_active(true);
            sprite.color.set_a(1.0);
        }
    }
}

fn follow_mouse(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut query: Query<&mut Character>,
) {
    for event in cursor_moved_events.iter() {
        let mut character = query.single_mut();
        character.mouse.x = event.position.x;
        character.mouse.y = event.position.y;
    }
}

fn follow_keyboard(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Character>,
) {
    let mut character = query.single_mut();

    let delta_seconds = time.delta_seconds();

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        character.speed.y = MAX_SPEED * delta_seconds;
    }

    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        character.speed.y = -MAX_SPEED * delta_seconds;
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        character.speed.x = MAX_SPEED * delta_seconds;
    }

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        character.speed.x = -MAX_SPEED * delta_seconds;
    }

    character.position.x += character.speed.x;
    character.position.y += character.speed.y;

    let max_x = WINDOW_SIZE.width / 2.0;
    let max_y = WINDOW_SIZE.height / 2.0;

    if character.position.x > max_x {
        character.position.x = max_x;
    } else if character.position.x < -max_x {
        character.position.x = -max_x;
    }

    if character.position.y > max_y {
        character.position.y = max_y;
    } else if character.position.y < -max_y {
        character.position.y = -max_y;
    }

    character.speed *= FRICTION;
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Main).with_system(setup))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(animate))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(transform_changed))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(active_changed))
            .add_system_set(
                SystemSet::on_update(AppState::Main).with_system(inactive_timer_changed),
            )
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(follow_mouse))
            .add_system_set(SystemSet::on_update(AppState::Main).with_system(follow_keyboard));
    }
}
