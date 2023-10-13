use std::f32::consts::PI;

use bevy::{
	prelude::*,
	// pbr::{CascadeShadowConfigBuilder},
};

use super::bound;
use crate::loading::config::Bounding;

#[derive(Debug, Component)]
pub struct LightKind(pub i32);


pub fn setup(commands: &mut Commands, bounding: &Bounding) {
	let cen_x = (bounding.min_x + bounding.max_x) / 2.0;
	let cen_y = (bounding.min_y + bounding.max_y) / 2.0;
	let cen_z = (bounding.min_z + bounding.max_z) / 2.0;

	// ambient light
	// commands.insert_resource(AmbientLight {
	// 	color: Color::WHITE,
	// 	brightness: 0.03,
	// });


	// point light
	commands.spawn((
		PointLightBundle {
			//cen_x, cen_y, cen_z
			transform: Transform::from_xyz(cen_x, cen_y, bounding.max_z-bound::THICKNESS),
			point_light: PointLight {
				intensity: 2400.0,  //150w
				color: Color::WHITE,
				range: 100.0,
				radius: 1.0,
				shadows_enabled: false,
				..default()
			},
			visibility: Visibility::Visible,
			..default()
		},
		LightKind(0)
	));


	// directional 'sun' light
	commands.spawn((
		DirectionalLightBundle {
			directional_light: DirectionalLight {
				shadows_enabled: false,
				..default()
			},
			transform: Transform {
				translation: Vec3::new(cen_x, cen_y, cen_z),
				rotation: Quat::from_euler(EulerRot::XYZ, -PI / 4.0, PI / 4.0, 0.0),

				..default()
			},
			// The default cascade config is designed to handle large scenes.
			// As this example has a much smaller world, we can tighten the shadow bounds for better visual quality.
			// cascade_shadow_config: CascadeShadowConfigBuilder {
			// 	first_cascade_far_bound: 4.0,
			// 	maximum_distance: 10.0,
			// 	..default()
			// }
			// 	.into(),
			visibility: Visibility::Hidden,
			..default()
		},
		LightKind(1)
	));
}