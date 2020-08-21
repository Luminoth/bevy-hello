#![deny(warnings)]

mod components;
mod plugins;
mod prefabs;
mod systems;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(plugins::HelloPlugin)
        .run();
}
