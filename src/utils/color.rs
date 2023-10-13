use bevy::prelude::*;

pub fn from_str(value: &String) -> Color {
	let split = value.split(" ");
	let rgba = split.collect::<Vec<&str>>();

	let l = rgba.len();

	//如果小于3个，就随机颜色
	if l < 3 {
		return Color::rgba(1.0, 1.0, 1.0, 1.0);
	}

	let r = rgba[0].parse::<f32>().unwrap();
	let g = rgba[1].parse::<f32>().unwrap();
	let b = rgba[2].parse::<f32>().unwrap();
	let mut a: f32 = 1.0;
	if l > 3 {
		a = rgba[3].parse::<f32>().unwrap();
	}
	Color::rgba(r, g, b, a)
}