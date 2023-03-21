use bevy::{
	input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
	prelude::*,
};
use smooth_bevy_cameras::{
	controllers::orbit::{ControlEvent, OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
	LookTransformPlugin,
};

use crate::GameState;

pub struct CameraPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(LookTransformPlugin)
			.add_plugin(OrbitCameraPlugin::new(true))
			.add_system(spawn_2d_camera.on_startup())
			.add_system(despawn_2d_camera.in_schedule(OnEnter(GameState::Playing)))
			.add_system(spawn_3d_camera.in_schedule(OnEnter(GameState::Playing)))
			.add_system(camera_input_map) //.in_schedule(OnEnter(GameState::Playing))
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
		.spawn(//Camera3dBundle::default()
			   Camera3dBundle {
				   camera: Camera {
					   hdr: true,
					   ..Default::default()
				   },
				   transform: Transform::from_xyz(0.0, 0.0, 20.0)
					   .looking_at(Vec3::ZERO, Vec3::Y),
				   ..Default::default()
			   },
		)
		.insert(OrbitCameraBundle::new(
			OrbitCameraController {
				mouse_rotate_sensitivity: Vec2::splat(0.2),
				mouse_translate_sensitivity: Vec2::splat(0.4),
				mouse_wheel_zoom_sensitivity: 0.4,
				..default()
			},
			Vec3::new(0.0, 0.0, 20.0),
			Vec3::ZERO,
			Vec3::Y,
		));
}

pub fn camera_input_map(
	mut events: EventWriter<ControlEvent>,
	mut mouse_wheel_reader: EventReader<MouseWheel>,
	mut mouse_motion_events: EventReader<MouseMotion>,
	mouse_buttons: Res<Input<MouseButton>>,
	controllers: Query<&OrbitCameraController>,
) {
	// Can only control one camera at a time.
	let controller = if let Some(controller) = controllers.iter().next() {
		controller
	} else {
		return;
	};

	let OrbitCameraController {
		enabled,
		mouse_rotate_sensitivity,
		mouse_translate_sensitivity,
		mouse_wheel_zoom_sensitivity,
		pixels_per_line,
		..
	} = *controller;

	if !enabled {
		return;
	}

	//-------------------

	let mut cursor_delta = Vec2::ZERO;
	for event in mouse_motion_events.iter() {
		cursor_delta += event.delta;
	}

	if mouse_buttons.pressed(MouseButton::Left) {
		events.send(ControlEvent::Orbit(mouse_rotate_sensitivity * cursor_delta));
	}

	if mouse_buttons.pressed(MouseButton::Right) {
		events.send(ControlEvent::TranslateTarget(
			mouse_translate_sensitivity * cursor_delta,
		));
	}

	let mut scalar = 1.0;
	for event in mouse_wheel_reader.iter() {
		// scale the event magnitude per pixel or per line
		let scroll_amount = match event.unit {
			MouseScrollUnit::Line => event.y,
			MouseScrollUnit::Pixel => event.y / pixels_per_line,
		};
		scalar *= 1.0 - scroll_amount * mouse_wheel_zoom_sensitivity;
	}
	events.send(ControlEvent::Zoom(scalar));
}

