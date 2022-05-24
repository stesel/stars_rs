use bevy::{prelude::*};

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let texture_handle = asset_server.load("character.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(128.0, 128.0), 5, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.05, true)));
}

fn animate(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>, &mut Transform)>
) {
    for (mut timer, mut sprite, texture_atlas_handle, mut transform) in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.len();

        }

        let rotation_z = (transform.rotation.z.to_radians() + 0.01) % std::f32::consts::PI;
        transform.rotate(Quat::from_rotation_z(rotation_z));
    }

}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(animate);
    }
}