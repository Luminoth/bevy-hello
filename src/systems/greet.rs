use crate::components;
use crate::prefabs;

pub fn greet_people(_: &prefabs::Person, name: &components::Name) {
    println!("hello {}!", name.0);
}
