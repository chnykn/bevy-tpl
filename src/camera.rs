use bevy::{
	input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
	prelude::*,
};
use smooth_bevy_cameras::{
	controllers::orbit::{ControlEvent, OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
	LookTransformPlugin,
};

use crate::GameState;
use crate::loading::config::Bounding;

#[derive(Debug, Clone, Resource)]
pub struct HasSetup3D;


pub struct CameraPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(LookTransformPlugin)
			.add_plugins(OrbitCameraPlugin::new(true))
			.add_systems(OnEnter(GameState::Menu), setup_2d_camera)
			.add_systems(OnEnter(GameState::Playing), setup_3d_camera)
			.add_systems(Update, orbit_3d_camera.run_if(in_state(GameState::Playing)))
		;
	}
}


pub fn setup_2d_camera(
	mut commands: Commands,
	done3d: Option<Res<HasSetup3D>>,
	// camera_query: Query<Entity, With<Camera3d>>,
	// camera_ctl_query: Query<Entity, With<OrbitCameraController>>,
) {
	if done3d.is_some() { return; }

	// for entity in &camera_query {
	// 	commands.entity(entity).despawn_recursive();
	// }
	//
	// for entity in &camera_ctl_query {
	// 	commands.entity(entity).despawn_recursive();
	// }

	commands.spawn(Camera2dBundle::default());
}

// set up a 3D camera
fn setup_3d_camera(
	mut commands: Commands,
	query: Query<Entity, With<Camera2d>>,
	done3d: Option<Res<HasSetup3D>>,
	bounding: Res<Bounding>,
) {
	if done3d.is_some() { return; }

	for entity in &query {
		commands.entity(entity).despawn_recursive();
	}

	let b = &bounding;
	let ty = (b.max_y - b.min_y) * 0.2 + b.max_y;
	let tz = (b.max_z - b.min_z) * 2.0 + b.max_z;

	commands
		.spawn(//Camera3dBundle::default()
			   Camera3dBundle {
				   camera: Camera {
					   hdr: true,
					   ..Default::default()
				   },
				   transform: Transform::from_xyz(0.0, ty, tz)
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
			Vec3::new(0.0, ty, tz),
			Vec3::ZERO,
			Vec3::Y,
		));

	commands.insert_resource(HasSetup3D {});
}

pub fn orbit_3d_camera(
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

