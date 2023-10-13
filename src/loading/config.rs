use std::fs::File;
use std::io::prelude::*;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct Bounding {
	pub min_x: f32,
	pub min_y: f32,

	pub max_x: f32,
	pub max_y: f32,

	pub min_z: f32,
	pub max_z: f32,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
	pub period: f32,
	pub bounding: Bounding,
}

pub fn load() -> GameConfig {
	let file_path = "config.toml";
	let mut file = match File::open(file_path) {
		Ok(f) => f,
		Err(e) => panic!("no such file {} exception:{}", file_path, e)
	};
	let mut str_val = String::new();
	match file.read_to_string(&mut str_val) {
		Ok(s) => s
		,
		Err(e) => panic!("Error Reading file: {}", e)
	};

	let result: GameConfig = toml::from_str(&str_val).unwrap();
	result
}