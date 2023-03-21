
use bevy::prelude::*;

use crate::GameState;

mod scene;
mod bound;

pub struct PlayingPlugin;


impl Plugin for PlayingPlugin {
	fn build(&self, app: &mut App) {
		 app.add_system(scene::setup_scene.in_schedule(OnEnter(GameState::Playing)))
		 ;
	}
}
