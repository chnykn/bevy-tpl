use bevy::{
	prelude::*,
	input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
};

#[derive(Component, Clone)]
pub struct KeySetting {
	pub forward: KeyCode,
	pub back: KeyCode,
	pub left: KeyCode,
	pub right: KeyCode,
	pub up: KeyCode,
	pub down: KeyCode,
	pub run: KeyCode,
	pub mouse_key: MouseButton,
	pub enable_mouse: KeyCode,
}

/// Provides basic movement functionality to the attached camera
#[derive(Component, Clone)]
pub struct CameraState {
	pub enabled: bool,
	pub initialized: bool,
	pub sensitivity: f32,
	pub walk_speed: f32,
	pub run_speed: f32,
	pub friction: f32,
	pub pitch: f32,
	pub yaw: f32,
	pub velocity: Vec3,
	pub orbit_focus: Vec3,
	pub orbit_mode: bool,
	pub scroll_wheel_speed: f32,
	pub lock_y: bool,
}

impl Default for KeySetting {
	fn default() -> Self {
		Self {
			forward: KeyCode::W,
			back: KeyCode::S,
			left: KeyCode::A,
			right: KeyCode::D,
			up: KeyCode::E,
			down: KeyCode::Q,
			run: KeyCode::LShift,
			mouse_key: MouseButton::Left,
			enable_mouse: KeyCode::M,
		}
	}
}


impl Default for CameraState {
	fn default() -> Self {
		Self {
			enabled: true,
			initialized: false,
			sensitivity: 0.25,
			walk_speed: 5.0,
			run_speed: 15.0,
			friction: 0.5,
			pitch: 0.0,
			yaw: 0.0,
			velocity: Vec3::ZERO,
			orbit_focus: Vec3::ZERO,
			orbit_mode: true,
			scroll_wheel_speed: 0.1,
			lock_y: true,
		}
	}
}

pub fn camera_controller(
	time: Res<Time>,
	mut mouse_events: EventReader<MouseMotion>,
	mouse_button_input: Res<Input<MouseButton>>,
	mut scroll_evr: EventReader<MouseWheel>,
	key_input: Res<Input<KeyCode>>,
	mut move_toggled: Local<bool>,
	mut query: Query<(&mut Transform, &KeySetting, &mut CameraState), With<Camera3d>>,
) {
	let r = query.get_single_mut();

	let (mut transform, keys, mut state) = match r {
		Ok(res) => res,
		Err(_err) => {
			return;
		},
	};


	let dt = time.delta_seconds();

	// if let Ok((mut transform, mut options)) = query.get_single_mut() {
		if !state.initialized {
			let (_roll, yaw, pitch) = transform.rotation.to_euler(EulerRot::ZYX);
			state.yaw = yaw;
			state.pitch = pitch;
			state.initialized = true;
		}
		if !state.enabled {
			return;
		}

		let mut scroll_distance = 0.0;

		// Handle scroll input
		for ev in scroll_evr.iter() {
			match ev.unit {
				MouseScrollUnit::Line => {
					scroll_distance = ev.y;
				}
				MouseScrollUnit::Pixel => (),
			}
		}

		// Handle key input
		let mut axis_input = Vec3::ZERO;
		if key_input.pressed(keys.forward) {
			axis_input.z -= 1.0;
		}
		if key_input.pressed(keys.back) {
			axis_input.z += 1.0;
		}
		if key_input.pressed(keys.right) {
			axis_input.x -= 1.0;
		}
		if key_input.pressed(keys.left) {
			axis_input.x += 1.0;
		}
		if key_input.pressed(keys.up) {
			axis_input.y -= 1.0;
		}
		if key_input.pressed(keys.down) {
			axis_input.y += 1.0;
		}
		if key_input.just_pressed(keys.enable_mouse) {
			*move_toggled = !*move_toggled;
		}

		// Apply movement update
		if axis_input != Vec3::ZERO {
			let max_speed = if key_input.pressed(keys.run) {
				state.run_speed
			} else {
				state.walk_speed
			};
			state.velocity = axis_input.normalize() * max_speed;
		} else {
			let friction = state.friction.clamp(0.0, 1.0);
			state.velocity *= 1.0 - friction;
			if state.velocity.length_squared() < 1e-6 {
				state.velocity = Vec3::ZERO;
			}
		}
		let forward = transform.forward();
		let right = transform.right();
		let mut translation_delta = state.velocity.x * dt * right
			+ state.velocity.y * dt * Vec3::Y
			+ state.velocity.z * dt * forward;
		let mut scroll_translation = Vec3::ZERO;
		if state.orbit_mode && state.scroll_wheel_speed > 0.0 {
			scroll_translation = scroll_distance
				* transform.translation.distance(state.orbit_focus)
				* state.scroll_wheel_speed
				* forward;
		}
		if state.lock_y {
			translation_delta *= Vec3::new(1.0, 0.0, 1.0);
		}
		transform.translation += translation_delta + scroll_translation;
	state.orbit_focus += translation_delta;

		// Handle mouse input
		let mut mouse_delta = Vec2::ZERO;
		if mouse_button_input.pressed(keys.mouse_key) || *move_toggled {
			for mouse_event in mouse_events.iter() {
				mouse_delta += mouse_event.delta;
			}
		} else {
			mouse_events.clear();
		}

		if mouse_delta != Vec2::ZERO {
			let sensitivity = if state.orbit_mode {
				state.sensitivity * 2.0
			} else {
				state.sensitivity
			};
			let (pitch, yaw) = (
				(state.pitch - mouse_delta.y * 0.5 * sensitivity * dt).clamp(
					-0.99 * std::f32::consts::FRAC_PI_2,
					0.99 * std::f32::consts::FRAC_PI_2,
				),
				state.yaw - mouse_delta.x * sensitivity * dt,
			);

			// Apply look update
			transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, yaw, pitch);
			state.pitch = pitch;
			state.yaw = yaw;

			if state.orbit_mode {
				let rot_matrix = Mat3::from_quat(transform.rotation);
				transform.translation = state.orbit_focus
					+ rot_matrix.mul_vec3(Vec3::new(
					0.0,
					0.0,
					state.orbit_focus.distance(transform.translation),
				));
			}
		}
	// }
}

/// Simple flying camera plugin.
/// In order to function, the [`CameraOptions`] component should be attached to the camera entity.
#[derive(Default)]
pub struct FlyCameraPlugin;

impl Plugin for FlyCameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(camera_controller);
	}
}