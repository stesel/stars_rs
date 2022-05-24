use bevy::{prelude::*};

fn main() {  
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

struct GreatTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Name 1".to_string()));
    commands.spawn().insert(Person).insert(Name("Name 2".to_string()));
    commands.spawn().insert(Person).insert(Name("Name 3".to_string()));
}

fn great_people(time: Res<Time>, mut timer: ResMut<GreatTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0)
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GreatTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people)
            .add_system(great_people);
    }
}


