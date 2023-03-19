use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Eq, PartialEq, Component,  Serialize, Deserialize)]
pub struct UiCamera;

pub fn spawn_camera(mut commands: Commands) {
	commands.spawn((Camera2dBundle::default(), UiCamera, Name::new("Camera")));
}

pub fn despawn_camera(mut commands: Commands, query: Query<Entity, With<UiCamera>>) {
	for entity in &query {
		commands.entity(entity).despawn_recursive();
	}
}
