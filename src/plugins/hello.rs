use bevy::prelude::*;

use crate::components;
use crate::prefabs;
use crate::systems;

pub struct HelloPlugin;

impl HelloPlugin {
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
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(HelloPlugin::add_people.system())
            .add_system(systems::hello_world.system())
            .add_system(systems::greet_people.system());
    }
}
