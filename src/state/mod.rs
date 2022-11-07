use bevy::{asset::HandleId, prelude::*};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    Main,
    Menu,
}

pub struct LoaderState {
    pub background_image: Handle<Image>,
    pub enemy_image: Handle<Image>,
    pub character_image: Handle<Image>,
    pub explosion_image: Handle<Image>,
    pub aim_image: Handle<Image>,
    pub button_sound: Handle<AudioSource>,
    pub bullet_sound: Handle<AudioSource>,
    pub collision_sound: Handle<AudioSource>,
    pub explosion_sound: Handle<AudioSource>,
    pub font: Handle<Font>,
}
impl LoaderState {
    pub fn ids(&self) -> Vec<HandleId> {
        vec![
            self.background_image.id,
            self.character_image.id,
            self.explosion_image.id,
            self.aim_image.id,
            self.button_sound.id,
            self.bullet_sound.id,
            self.collision_sound.id,
            self.explosion_sound.id,
            self.font.id,
        ]
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct GameState {
    pub enemies_left: u32,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Loading);
    }
}
