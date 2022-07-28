use bevy::{prelude::*};

use crate::{character::Character, enemies::Enemy, state::AppState, utils::HitTest};

fn check_collision(
    character_query: Query<(&Character)>,
    enemy_query: Query<(&Enemy)>,
    time: Res<Time>,
) {
    let character = character_query.single();

    for (enemy) in enemy_query.iter() {
        if character.hit_test(enemy) {
            println!("character hits {}", time.delta_seconds());
        }
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Main).with_system(check_collision));
    }
}