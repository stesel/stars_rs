use bevy::prelude::*;

pub struct TransformEvent {
    pub position: Vec2,
    pub rotation: f32,
}

pub struct AddExplosionEvent {
    pub position: Vec2,
}

pub struct EnemiesLeftEvent {
    pub enemies_left: u32,
}

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TransformEvent>()
            .add_event::<AddExplosionEvent>()
            .add_event::<EnemiesLeftEvent>();
    }
}
