use bevy::{prelude::*};

#[derive(Component, Deref, DerefMut)]
struct CharacterAnimationTimer(Timer);

#[derive(Component)]
struct CharacterTransform {
    position: Vec2,
    rotation: Quat,
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
        .insert(CharacterTransform { position: Vec2::new(0.0, 0.0), rotation: Quat::default() });
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

fn transform_changed(mut query: Query<(&CharacterTransform, &mut Transform), Changed<CharacterTransform>>) {
    for (character_transform, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(character_transform.position.x, character_transform.position.y, 0.0);
        transform.rotation = character_transform.rotation;
    }
}

fn follow_mouse(
    windows: Res<Windows>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut query: Query<&mut CharacterTransform>
) {
    for event in cursor_moved_events.iter() {
        let mut character_transform = query.single_mut();
        let window = windows.get(event.id).unwrap();
        let window_width = window.width();
        let window_height = window.height();
        let delta_x = event.position.x - window_width / 2.0;
        let delta_y = event.position.y - window_height / 2.0;
        let rotation_z = -delta_x.atan2(delta_y);
        character_transform.rotation = Quat::from_rotation_z(rotation_z);
    }
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(animate)
            .add_system(transform_changed)
            .add_system(follow_mouse);
    }
}