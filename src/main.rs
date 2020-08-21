#![deny(warnings)]

mod components;
mod plugins;
mod prefabs;
mod systems;

use bevy::prelude::*;
use bevy::render::pass::ClearColor;

fn main() {
    App::build()
        .add_default_plugins()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugin(plugins::HelloPlugin)
        .run();
}
