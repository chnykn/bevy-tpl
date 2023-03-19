use std::io::Cursor;

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use winit::window::Icon;

use anyhow::{Context, Result};
use crate::util::pipe::log_errors;

const BG_COLOR: Color = Color::rgb(0.1, 0.1, 0.2); //::BLACK;//

//--------------------------------------

/// Overrides the default Bevy plugins and configures things like the screen settings.
pub struct InitialPlugin;

impl Plugin for InitialPlugin {
    fn build(&self, app: &mut App) {
        let default_plugins = DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1280., 800.).into(),
                title: "bevy".to_string(),
                canvas: Some("#bevy".to_owned()),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        });
        app.insert_resource(Msaa::Sample4)
            .insert_resource(ClearColor(BG_COLOR))
            .add_plugins(default_plugins)
            .add_system(set_window_icon.pipe(log_errors).on_startup());
    }
}

fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_windows: Query<Entity, With<PrimaryWindow>>,
) -> Result<()> {
    let primary_entity = primary_windows.single();
    let primary = windows
        .get_window(primary_entity)
        .context("Failed to get primary window")?;
    let icon_buf = Cursor::new(include_bytes!(
        "../assets/icon.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height)?;
        primary.set_window_icon(Some(icon));
    };
    Ok(())
}
