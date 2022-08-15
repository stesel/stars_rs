use bevy::{prelude::*};

use crate::{character::Character, enemies::Enemy, state::AppState, utils::{GetBoundingRect, IsActive, SetSpeed, hit_test}};

fn check_collision(
    mut character_query: Query<&mut Character>,
    enemy_query: Query<&Enemy>,
    time: Res<Time>,
) {
    let mut character = character_query.single_mut();

    for enemy in enemy_query.iter() {
        if character.get_active() && hit_test(character.get_bounding_rect(), enemy.get_bounding_rect()) {
            println!("character hits {}", time.delta_seconds());
            character.set_speed(enemy.speed * 0.02);
            character.set_active(false);
        }
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Main).with_system(check_collision));
    }
}