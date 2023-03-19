// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bvtpl::GamePlugin;

fn main() {
    // println!("Hello, world!");
    App::new().add_plugin(GamePlugin).run();
}
