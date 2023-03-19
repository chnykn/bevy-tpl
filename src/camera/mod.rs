use bevy::prelude::*;

mod ui;
mod fly;

use crate::GameState;

pub struct CameraPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(fly::FlyCameraPlugin)
			.add_system(ui::spawn_camera.on_startup())
			.add_system(ui::despawn_camera.in_schedule(OnEnter(GameState::Playing)))
			.add_system(setup_3d_camera.in_schedule(OnEnter(GameState::Playing)))
			;
	}
}

/// set up a 3D camera
fn setup_3d_camera(mut commands: Commands) {
	commands.spawn((
		Camera3dBundle {
			transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		},
		fly::KeySetting{ ..default()},
		fly::CameraState{ ..default()},
	));
}



