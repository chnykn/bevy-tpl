// These two generate a lot of false positives for Bevy systems
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]


use bevy::prelude::*;

pub mod utils;

mod loading;
mod menus;
mod camera;
mod scene;


#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash, SystemSet)]
pub enum GameState {
	/// During the loading State the LoadingPlugin will load our assets
	#[default]
	Loading,

	/// Here the menu is drawn and waiting for player interaction
	Menu,

	/// During this State the actual game logic is executed
	Playing,
}


pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_state::<GameState>()
			.add_plugins(loading::LoadingPlugin)
			.add_plugins(menus::MenusPlugin)
			.add_plugins(camera::CameraPlugin)
			.add_plugins(scene::ScenePlugin)
		;

		// #[cfg(feature = "dev")]
		// app.add_plugin(DevPlugin);

		// #[cfg(debug_assertions)]
		// {
		//     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
		//         .add_plugin(LogDiagnosticsPlugin::default());
		// }
	}
}
