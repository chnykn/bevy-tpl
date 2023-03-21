// These two generate a lot of false positives for Bevy systems
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]


use bevy::prelude::*;

pub mod util;

mod initial;
mod loading;
mod menus;
mod camera;
mod playing;


#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
	/// During the loading State the LoadingPlugin will load our assets
	#[default]
	Loading,
	/// During this State the actual game logic is executed
	Playing,
	/// Here the menu is drawn and waiting for player interaction
	Menu,
}


pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_state::<GameState>()
			.add_plugin(initial::InitialPlugin)
			.add_plugin(loading::LoadingPlugin)
			.add_plugin(menus::MenusPlugin)
			.add_plugin(camera::CameraPlugin)
			.add_plugin(playing::PlayingPlugin)
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
