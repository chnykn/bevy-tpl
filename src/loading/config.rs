use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bounding {
	pub min_x: f32,
	pub min_y: f32,

	pub max_x: f32,
	pub max_y: f32,

	pub min_z: f32,
	pub max_z: f32,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Resource, TypeUuid)]
#[uuid = "8a570b5a-f7df-1ae0-5d52-8bac408a1b70"]
pub struct GameConfig {
	pub period: f32,
	pub bounding: Bounding,
}
