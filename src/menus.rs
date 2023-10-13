use bevy::prelude::*;

use crate::GameState;
// use crate::loading::config;
use crate::loading::FontAssets;
use crate::scene::PlayStage;

pub struct MenusPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenusPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<ButtonColors>()
			.add_systems(OnEnter(GameState::Menu), setup_menu)
			.add_systems(Update, click_button.run_if(in_state(GameState::Menu)))
			.add_systems(OnExit(GameState::Menu), clear_menu);
	}
}

#[derive(Resource)]
struct ButtonColors {
	normal: Color,
	hovered: Color,
}

impl Default for ButtonColors {
	fn default() -> Self {
		ButtonColors {
			normal: Color::rgb(0.15, 0.15, 0.15),
			hovered: Color::rgb(0.25, 0.25, 0.25),
		}
	}
}

//-------------------------------------------------

fn spawn_button(commands: &mut Commands, zindex: i32, title: &str,
				font_assets: &Res<FontAssets>, button_colors: &Res<ButtonColors>) {
	commands
		.spawn(ButtonBundle {
			style: Style {
				width: Val::Px(120.0),
				height:Val::Px(50.0),

				margin: UiRect::all(Val::Auto),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				..Default::default()
			},
			background_color: button_colors.normal.into(),
			z_index: ZIndex::Local(zindex), // 使用z_index区分 "重新开始" 与 "开始"/"继续" 按钮
			..Default::default()
		})
		.with_children(|parent| {
			parent.spawn(TextBundle::from_section(
				title,
				TextStyle {
					font: font_assets.jsong.clone(),
					font_size: 32.0,
					color: Color::rgb(0.9, 0.9, 0.9),
				},
			));
		});
}

fn setup_menu(
	mut commands: Commands,
	play_stage: Res<PlayStage>,
	font_assets: Res<FontAssets>,
	button_colors: Res<ButtonColors>,
) {

	if play_stage.value == -1 {
		spawn_button(&mut commands, 0, "重新开始", &font_assets, &button_colors);
	}

	let title = if play_stage.value == -1 {"继续"} else {"开始"};
	spawn_button(&mut commands, 1, title, &font_assets, &button_colors);
}

fn click_button(
	// mut commands: Commands,
	button_colors: Res<ButtonColors>,
	mut state: ResMut<NextState<GameState>>,
	mut interaction_query: Query<
		(&Interaction, &ZIndex, &mut BackgroundColor),
		(Changed<Interaction>, With<Button>),
	>,
	mut play_stage: ResMut<PlayStage>,
) {
	for (interaction, z_index, mut color) in &mut interaction_query {
		match *interaction {
			Interaction::Pressed => {

				if let ZIndex::Local(1) = z_index {
					// "开始"/"继续" 按钮
					state.set(GameState::Playing);
				} else {
					// "重新开始" 按钮
					//xxxx.reset();
					play_stage.value = 1;
					play_stage.store = -1;
					// play_stage.last_val = 0;

					state.set(GameState::Playing);
				}
			}
			Interaction::Hovered => {
				*color = button_colors.hovered.into();
			}
			Interaction::None => {
				*color = button_colors.normal.into();
			}
		}
	}
}

fn clear_menu(mut commands: Commands, btn_query: Query<Entity, With<Button>>) {
	for ent in &btn_query {
		 commands.entity(ent).despawn_recursive();
	}
	// commands.entity(btn_query.single()).despawn_recursive();
}
