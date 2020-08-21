use bevy::prelude::*;

use crate::components;
use crate::prefabs;

pub struct GreetTimer(pub Timer);

pub fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    mut query: Query<(&prefabs::Person, &components::Name)>,
) {
    // update greeting timer
    timer.0.tick(time.delta_seconds);

    // if the timer elapsed, greet and reset
    if timer.0.finished {
        for (_person, name) in &mut query.iter() {
            println!("hello {}!", name.0);
        }
        timer.0.reset();
    }
}
