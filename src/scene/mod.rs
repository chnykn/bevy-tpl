use bevy::prelude::*;

use crate::GameState;

mod util;
mod light;
mod bound;

#[derive(Debug, Clone, Resource)]
pub struct PlayStage {
	pub value: i32,

	pub store: i32,
}


pub struct ScenePlugin;


impl Plugin for ScenePlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(PlayStage { value: 0, store: -1})
			.add_systems(OnEnter(GameState::Playing), util::setup)
			// .add_systems(scene::key_press.in_set(OnUpdate(GameState::Playing)))
			.add_systems(Update, util::key_press.run_if(in_state(GameState::Playing)))
		;
	}
}
