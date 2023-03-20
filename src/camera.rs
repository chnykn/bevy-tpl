use bevy::prelude::*;
use smooth_bevy_cameras::{
	controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
	LookTransformPlugin,
};

use crate::GameState;

pub struct CameraPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(LookTransformPlugin)
			.add_plugin(OrbitCameraPlugin::default())
			.add_system(spawn_2d_camera.on_startup())
			.add_system(despawn_2d_camera.in_schedule(OnEnter(GameState::Playing)))
			.add_system(spawn_3d_camera.in_schedule(OnEnter(GameState::Playing)))
		;
	}
}


pub fn spawn_2d_camera(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}

pub fn despawn_2d_camera(mut commands: Commands, query: Query<Entity, With<Camera2d>>) {
	for entity in &query {
		commands.entity(entity).despawn_recursive();
	}
}


// set up a 3D camera
fn spawn_3d_camera(mut commands: Commands) {
	commands
		.spawn(Camera3dBundle::default())
		.insert(OrbitCameraBundle::new(
			OrbitCameraController {
				mouse_rotate_sensitivity: Vec2::splat(0.2),
				mouse_translate_sensitivity: Vec2::splat(0.4),
				mouse_wheel_zoom_sensitivity: 0.4,
				..default()
			},
			Vec3::new(-5.0, 5.0, 5.0),
			Vec3::new(0., 0., 0.),
			Vec3::Y,
		));
}



