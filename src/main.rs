// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Cursor;

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use winit::window::Icon;


use bvtpl::GamePlugin;

const BG_COLOR: Color = Color::rgb(0.05, 0.05, 0.1); //Color::BLACK;

fn main() {

	let default_plugins = DefaultPlugins.set(WindowPlugin {
		primary_window: Some(Window {
			title: "Bevy game".to_string(), // ToDo
			resolution: (1280., 800.).into(),
			canvas: Some("#bevy".to_owned()),
			present_mode: PresentMode::AutoVsync,
			..default()
		}),
		..default()
	});

	App::new()
		.insert_resource(Msaa::Sample4) //Msaa::Off
		.insert_resource(ClearColor(BG_COLOR))
		.add_systems(Startup,set_window_icon)
		.add_plugins(default_plugins)
		.add_plugins(GamePlugin)
		.run();
}


fn set_window_icon(
	windows: NonSend<WinitWindows>,
	primary_windows: Query<Entity, With<PrimaryWindow>>,
)  {
	let primary_entity = primary_windows.single();
	let primary = windows.get_window(primary_entity).unwrap();
	let icon_buf = Cursor::new(include_bytes!(
		"../assets/icon.png"
	));
	if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
		let image = image.into_rgba8();
		let (width, height) = image.dimensions();
		let rgba = image.into_raw();
		let icon = Icon::from_rgba(rgba, width, height).unwrap();
		primary.set_window_icon(Some(icon));
	};
}
