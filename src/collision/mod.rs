use bevy::prelude::*;

use crate::{
    bullet::Bullet,
    character::{Character, CharacterActive},
    enemies::{Enemy, EnemyCount},
    events::{AddExplosionEvent, EnemiesLeftEvent},
    state::{AppState, LoaderState},
    utils::{hit_test, GetBoundingRect, IsActive, SetSpeed},
};

fn check_character_collision(
    mut character_query: Query<(&mut Character, &mut CharacterActive)>,
    enemy_query: Query<&Enemy>,
    loader: ResMut<LoaderState>,
    audio: Res<Audio>,
) {
    let (mut character, mut character_active) = character_query.single_mut();

    for enemy in enemy_query.iter() {
        if character_active.get_active()
            && hit_test(character.get_bounding_rect(), enemy.get_bounding_rect())
        {
            character.set_speed(enemy.speed * 0.02);
            character_active.set_active(false);

            audio.play(loader.collision_sound.clone());
        }
    }
}

fn check_bullet_collision(
    bullet_query: Query<(Entity, &Bullet)>,
    enemy_query: Query<(Entity, &Enemy)>,
    mut enemy_count_query: Query<&mut EnemyCount>,
    mut add_explosion_events: EventWriter<AddExplosionEvent>,
    mut enemies_left_events: EventWriter<EnemiesLeftEvent>,
    mut commands: Commands,
    loader: ResMut<LoaderState>,
    audio: Res<Audio>,
) {
    for (bullet_entity, bullet) in bullet_query.iter() {
        for (enemy_entity, enemy) in enemy_query.iter() {
            if hit_test(bullet.get_bounding_rect(), enemy.get_bounding_rect()) {
                add_explosion_events.send(AddExplosionEvent {
                    position: enemy.position,
                });

                let mut enemy_count = enemy_count_query.single_mut();
                enemy_count.remove();
                enemies_left_events.send(EnemiesLeftEvent {
                    enemies_left: enemy_count.count,
                });

                commands.entity(enemy_entity).despawn();
                commands.entity(bullet_entity).despawn();

                audio.play(loader.explosion_sound.clone());
            }
        }
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddExplosionEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::Main).with_system(check_character_collision),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Main).with_system(check_bullet_collision),
            );
    }
}
