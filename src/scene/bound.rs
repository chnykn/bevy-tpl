use bevy::prelude::*;

use crate::loading::config::Bounding;

pub const THICKNESS: f32 = 0.04;
const SIDE_COLOR: Color = Color::rgba(0.6, 0.6, 0.8, 0.2);

#[derive(Component)]
pub struct BoundSide;

//--------------------------------------

pub enum Location {
	Left,
	Right,
	Bottom,
	Top,
	Back,
	#[allow(dead_code)]
	Front,
}

impl Location {
	pub fn position(&self, bounding: &Bounding) -> Vec3 {
		let d = THICKNESS / 2.0;
		let cen_x = (bounding.min_x + bounding.max_x) / 2.0;
		let cen_y = (bounding.min_y + bounding.max_y) / 2.0;
		let cen_z = (bounding.min_z + bounding.max_z) / 2.0;

		match self {
			Location::Left => Vec3::new(bounding.min_x - d, cen_y, cen_z),
			Location::Right => Vec3::new(bounding.max_x + d, cen_y, cen_z),
			Location::Bottom => Vec3::new(cen_x, bounding.min_y - d, cen_z),
			Location::Top => Vec3::new(cen_x, bounding.max_y + d, cen_z),
			Location::Back => Vec3::new(cen_x, cen_y, bounding.min_z - d),
			Location::Front => Vec3::new(cen_x, cen_y, bounding.max_z + d),
		}
	}

	pub fn size(&self, bounding: &Bounding) -> Vec3 {
		let d = THICKNESS * 2.0;

		let size_x = bounding.max_x - bounding.min_x;
		let size_y = bounding.max_y - bounding.min_y;
		let size_z = bounding.max_z - bounding.min_z;

		match self {
			Location::Left | Location::Right => {
				Vec3::new(THICKNESS, size_y + d, size_z + d)
			}
			Location::Bottom | Location::Top => {
				Vec3::new(size_x + d, THICKNESS, size_z + d)
			}
			Location::Back | Location::Front => {
				Vec3::new(size_x + d, size_y + d, THICKNESS)
			}
		}
	}
}

//--------------------------------------

fn new_side_bundle(mat: &Handle<StandardMaterial>, cube: &Handle<Mesh>,
				   location: Location, bounding: &Bounding) -> PbrBundle {
	PbrBundle {
		mesh: cube.clone(),
		material: mat.clone(),
		transform: Transform {
			translation: location.position(bounding),
			scale: location.size(bounding),
			..Default::default()
		},
		..Default::default()
	}
}

pub fn setup(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	bounding: &Bounding) {

	// Ground
	commands.spawn((
		PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Plane::default())),
			material: materials.add( Color::GRAY.into()),
			transform: Transform {
				translation: Vec3::new(
					(bounding.min_x + bounding.max_x) / 2.0,
					bounding.min_y - THICKNESS / 2.0,
					(bounding.min_z + bounding.max_z) / 2.0),
				scale: Vec3::new(
					(bounding.max_x - bounding.min_x) * 4.0,
					THICKNESS,
					(bounding.max_z - bounding.min_z) * 4.0),
				..Default::default()
			},
			..Default::default()
		},
		// RaycastMesh::<RaycastSet>::default(),
	));

	// Side
	let cube = meshes.add(Mesh::from(shape::Cube::default()));
	// let mat = materials.add(StandardMaterial {
	// 	base_color: DEF_COLOR,
	// 	// perceptual_roughness: 0.9,
	// 	..Default::default()
	// });
	let mat = materials.add(SIDE_COLOR.into());

	commands.spawn((new_side_bundle(&mat, &cube, Location::Left, &bounding), BoundSide));
	commands.spawn((new_side_bundle(&mat, &cube, Location::Right, &bounding), BoundSide));
	commands.spawn((new_side_bundle(&mat, &cube, Location::Bottom, &bounding), BoundSide));
	commands.spawn((new_side_bundle(&mat, &cube, Location::Top, &bounding), BoundSide));
	commands.spawn((new_side_bundle(&mat, &cube, Location::Back, &bounding), BoundSide));
	// commands.spawn((new_bundle(&mat, &cube, Location::Front, &bounding), BoundSide));
}