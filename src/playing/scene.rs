use bevy::{
	prelude::*,
};
use rand::{Rng, thread_rng};

use crate::loading::config::Bounding;
use crate::loading::config::GameConfig;
use crate::loading::ConfigAssets;
use crate::loading::FontAssets;
use crate::playing::bound;

const NUM_BODIES: usize = 100;


pub fn setup_scene(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	font_assets: Res<FontAssets>,
	config_assets: Res<ConfigAssets>,
	assets_config: ResMut<Assets<GameConfig>>,
) {
	let cfg = assets_config.get(&config_assets.config);
	let bb = &(cfg.unwrap().bounding);

	setup_light(&mut commands, bb);
	bound::build(&mut commands, &mut meshes, &mut materials, bb);


	//---random ball---

	let color_range = 0.5..1.0;

	let mesh = meshes.add(
		Mesh::try_from(shape::Icosphere {
			radius: 1.0,
			subdivisions: 3,
		})
			.unwrap(),
	);

	let mut rng = thread_rng();

	for _ in 0..NUM_BODIES {
		let radius: f32 = rng.gen_range(0.04..0.08);

		let position = Vec3::new(
			rng.gen_range(bb.min_x..bb.max_x),
			rng.gen_range(bb.min_y..bb.max_y),
			rng.gen_range(bb.min_z..bb.max_z),
		);

		commands.spawn(
			PbrBundle {
				transform: Transform {
					translation: position,
					scale: Vec3::splat(radius),
					..default()
				},
				mesh: mesh.clone(),
				material: materials.add(
					Color::rgb(
						rng.gen_range(color_range.clone()),
						rng.gen_range(color_range.clone()),
						rng.gen_range(color_range.clone()),
					)
						.into(),
				),
				..default()
			}
		);
	}


//---text---

	commands
		.spawn(TextBundle {
			style: Style {
				align_self: AlignSelf::FlexStart,
				flex_direction: FlexDirection::Column,
				..Default::default()
			},
			text: Text {
				sections: vec![
					TextSection {
						value: "Path between shooter and mouse cursor: ".to_string(),
						style: TextStyle {
							font: font_assets.fira_sans.clone(),
							font_size: 20.0,
							color: Color::WHITE,
						},
					},
					TextSection {
						value: "Direct!".to_string(),
						style: TextStyle {
							font: font_assets.fira_sans.clone(),
							font_size: 20.0,
							color: Color::WHITE,
						},
					},
				],
				..Default::default()
			},
			..Default::default()
		});
// .insert(PathStatus);
}

fn setup_light(commands: &mut Commands, bouding: &Bounding) {
	let cen_x = (bouding.min_x + bouding.max_x) / 2.0;
	let cen_y = (bouding.min_y + bouding.max_y) / 2.0;
	let cen_z = (bouding.min_z + bouding.max_z) / 2.0;

	// ambient light
	commands.insert_resource(AmbientLight {
		color: Color::WHITE,
		brightness: 0.03,
	});

	// point light
	commands.spawn(PointLightBundle {
		transform: Transform::from_xyz(cen_x, cen_y, cen_z),
		point_light: PointLight {
			intensity: 2400.0,  //150w
			color: Color::WHITE,
			range: 100.0,
			radius: 1.0,
			// shadows_enabled: true,
			..default()
		},
		..default()
	});


	// directional 'sun' light
	/*
	commands.spawn(DirectionalLightBundle {
		directional_light: DirectionalLight {
			shadows_enabled: true,
			..default()
		},
		transform: Transform {
			translation: Transform::from_xyz(cen_x, cen_y, cen_z),,
			rotation: Quat::from_euler(EulerRot::XYZ, -PI / 4.0, PI / 4.0, 0.0),

			..default()
		},
		// The default cascade config is designed to handle large scenes.
		// As this example has a much smaller world, we can tighten the shadow
		// bounds for better visual quality.
		cascade_shadow_config: CascadeShadowConfigBuilder {
			first_cascade_far_bound: 4.0,
			maximum_distance: 10.0,
			..default()
		}
			.into(),
		..default()
	});
	*/
}