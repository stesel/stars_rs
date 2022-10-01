use bevy::{prelude::*, asset::HandleId};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    Main,
    Menu,
}

pub struct LoaderState {
    pub loader_entity: Entity,
    pub background_image: Handle<Image>,
    pub enemy_image: Handle<Image>,
    pub character_image: Handle<Image>,
    pub explosion_image: Handle<Image>,
    pub aim_image: Handle<Image>,
    pub font: Handle<Font>
}
impl LoaderState {
    pub fn ids(&self) -> Vec<HandleId> {
        vec![
            self.background_image.id,
            self.character_image.id,
            self.explosion_image.id,
            self.aim_image.id,
            self.font.id,
        ]
    }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Loading);
    }
}
