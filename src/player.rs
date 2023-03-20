use bevy::prelude::*;

use crate::GameState;

pub struct PlayerPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(start_play.in_schedule(OnEnter(GameState::Playing)))
		;
	}
}

fn start_play(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// plane
	commands.spawn(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Plane {
			size: 5.0,
			..default()
		})),
		material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
		..default()
	});

	// cube
	commands.spawn(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
		material: materials.add(Color::rgba(0.8, 0.7, 0.6, 0.5).into()),
		transform: Transform::from_xyz(0.0, 0.5, 0.0),
		..default()
	});

	// light
	commands.spawn(PointLightBundle {
		point_light: PointLight {
			intensity: 1500.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(6.0, 8.0, 6.0),
		..default()
	});
}