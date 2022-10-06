use bevy::prelude::*;

use crate::{
    bullet::Bullet,
    character::Character,
    enemies::{Enemy, EnemyCount},
    events::AddExplosionEvent,
    state::AppState,
    utils::{hit_test, GetBoundingRect, IsActive, SetSpeed},
};

fn check_character_collision(
    mut character_query: Query<&mut Character>,
    enemy_query: Query<&Enemy>,
) {
    let mut character = character_query.single_mut();

    for enemy in enemy_query.iter() {
        if character.get_active()
            && hit_test(character.get_bounding_rect(), enemy.get_bounding_rect())
        {
            character.set_speed(enemy.speed * 0.02);
            character.set_active(false);
        }
    }
}

fn check_bullet_collision(
    mut bullet_query: Query<&mut Bullet>,
    enemy_query: Query<(Entity, &Enemy)>,
    mut enemy_count_query: Query<&mut EnemyCount>,
    mut add_explosion_events: EventWriter<AddExplosionEvent>,
    mut commands: Commands,
) {
    for mut bullet in bullet_query.iter_mut() {
        for (entity, enemy) in enemy_query.iter() {
            if bullet.get_active()
                && hit_test(bullet.get_bounding_rect(), enemy.get_bounding_rect())
            {
                add_explosion_events.send(AddExplosionEvent {
                    position: enemy.position,
                });
                let mut enemy_count = enemy_count_query.single_mut();
                enemy_count.remove();
                commands.entity(entity).despawn();
                bullet.set_active(false);
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
