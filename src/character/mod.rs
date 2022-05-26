use bevy::{prelude::*, input::{keyboard::{KeyCode}}};

#[derive(Component, Deref, DerefMut)]
struct CharacterAnimationTimer(Timer);

#[derive(Component)]
struct Character {
    position: Vec2,
    speed: Vec2,
    mouse: Vec2,
}

static MAX_SPEED: f32 = 150.0;
static FRICTION: f32 = 0.96;

impl Default for Character {
    fn default() -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            speed: Vec2::new(0.0, 0.0),
            mouse: Vec2::new(300.0, 250.0),
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let texture_handle = asset_server.load("character.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(128.0, 128.0), 5, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..default()
        })
        .insert(CharacterAnimationTimer(Timer::from_seconds(0.05, true)))
        .insert(Character::default());
}

fn animate(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut CharacterAnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.len();
        }
    }

}

fn transform_changed(windows: Res<Windows>, mut query: Query<(&Character, &mut Transform), Changed<Character>>) {
    for (character, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(character.position.x, character.position.y, 0.0);

        let window = windows.get_primary().unwrap();
        let window_width = window.width();
        let window_height = window.height();
        let delta_x = character.mouse.x - character.position.x - window_width / 2.0;
        let delta_y = character.mouse.y - character.position.y - window_height / 2.0;
        let rotation_z = -delta_x.atan2(delta_y);

        transform.rotation = Quat::from_rotation_z(rotation_z);
    }
}

fn follow_mouse(mut cursor_moved_events: EventReader<CursorMoved>,mut query: Query<&mut Character>) {
    for event in cursor_moved_events.iter() {
        let mut character = query.single_mut();
        character.mouse.x = event.position.x;
        character.mouse.y = event.position.y;
    }
}

fn follow_keyboard(
    windows: Res<Windows>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Character>
) {
    let mut character = query.single_mut();

    let delta = time.delta().as_secs_f32();
    

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        character.speed.y = MAX_SPEED * delta;
    }

    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        character.speed.y = -MAX_SPEED * delta;
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        character.speed.x = MAX_SPEED * delta;
    }

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        character.speed.x = -MAX_SPEED * delta;
    }

    character.position.x += character.speed.x;
    character.position.y += character.speed.y;

    let window = windows.get_primary().unwrap();
    let max_x = window.width() / 2.0;
    let max_y = window.height() / 2.0;

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
        app
            .add_startup_system(setup)
            .add_system(animate)
            .add_system(transform_changed)
            .add_system(follow_mouse)
            .add_system(follow_keyboard);
    }
}
