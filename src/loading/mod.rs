use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use bevy_common_assets::toml::TomlAssetPlugin;
// use bevy_kira_audio::AudioSource;

mod config;

use crate::GameState;
use config::GameConfig;

//------------------------------------------------------------

pub struct LoadingPlugin;

// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
// Alternatively you can write the logic to load assets yourself
// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(TomlAssetPlugin::<GameConfig>::new(&["toml"]))
			.add_loading_state(
				LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
			)
			.add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
			.add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
			.add_collection_to_loading_state::<_, ConfigAssets>(GameState::Loading);
		// .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading);
	}
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources
// (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
	#[asset(path = "fonts/FiraSans-Bold.ttf")]
	pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
	#[asset(path = "textures/bevy.png")]
	pub texture_bevy: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct ConfigAssets {
	#[asset(path = "config/game.toml")]
	pub game: Handle<GameConfig>,
}

// #[derive(AssetCollection, Resource)]
// pub struct AudioAssets {
//     #[asset(path = "audio/flying.ogg")]
//     pub flying: Handle<AudioSource>,
// }

