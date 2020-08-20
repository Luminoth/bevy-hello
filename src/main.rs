#![deny(warnings)]

mod components;
mod prefabs;
mod systems;

use bevy::prelude::*;

fn add_people(mut commands: Commands) {
    commands
        .spawn((
            prefabs::Person,
            components::Name("Elaina Proctor".to_string()),
        ))
        .spawn((prefabs::Person, components::Name("Renzo Hume".to_string())))
        .spawn((
            prefabs::Person,
            components::Name("Zayna Nieves".to_string()),
        ));
}

fn main() {
    App::build()
        .add_startup_system(add_people.system())
        .add_system(systems::hello_world.system())
        .add_system(systems::greet_people.system())
        .run();
}
