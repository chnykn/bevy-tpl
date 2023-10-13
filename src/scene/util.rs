use bevy::{
	prelude::*,
	// pbr::{CascadeShadowConfigBuilder},
};

use crate::GameState;
use crate::loading::config::Bounding;
use super::bound;
use super::light;
use super::PlayStage;
// use super::gas::DbSQL;

#[derive(Debug, Clone, Resource)]
pub struct HasSetup;


pub fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut text_query: Query<&mut Text>,
	mut play_stage: ResMut<PlayStage>,
	done: Option<Res<HasSetup>>,
	bounding: Res<Bounding>,
) {
	// play_stage.store>=0, 表示从GameState::Playing 退回到 GameState::Menu
	// 然后重新又返回GameState::Playing，需要恢复原有play_stage.value
	if play_stage.store >=0 {
		play_stage.value = play_stage.store;
		play_stage.store = -1;
	};

	if done.is_some() { return; }

	// let mut text = text_query.single_mut();
	// text.sections[0].value = String::from("请按下回车或空格键后启动模拟");

	for mut text in &mut text_query {
		text.sections[0].value = String::from("请按下回车或空格键后启动模拟");
		break; //只需要设置第一个文本, 但是又不知道如何从text_query获取第一个，只能循环了
	}


	light::setup(&mut commands, &bounding);
	bound::setup(&mut commands, &mut meshes, &mut materials, &bounding);

	commands.insert_resource(HasSetup {});
}

pub fn key_press(
	keys: Res<Input<KeyCode>>,
	// mut game_state: ResMut<State<GameState>>,
	mut next_game_state: ResMut<NextState<GameState>>,
	mut light_query: Query<(&mut Visibility, &light::LightKind)>,
	mut play_stage: ResMut<PlayStage>,
) {
	if keys.just_pressed(KeyCode::L) {
		for (mut viz, _kind) in light_query.iter_mut() {
			if *viz == Visibility::Visible {
				*viz = Visibility::Hidden;
			} else if *viz == Visibility::Hidden {
				*viz = Visibility::Visible;
			}
		}

	} else if keys.just_pressed(KeyCode::Escape) {
		// for mut viz in text_query.iter_mut() {
		// 	*viz = Visibility::Hidden;
		// }

		//暂停 状态下，为了避免gas::下的各system继续工作，设置 play_stage.value = -1;
		play_stage.store = play_stage.value;
		play_stage.value = -1;
		next_game_state.set(GameState::Menu);

	} else if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Return) {

		if play_stage.value < 4 {
			play_stage.value += 1;
		}

	}
}
