use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};


// Reflect, FromReflect,
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Resource, TypeUuid)]
// #[reflect(Serialize, Deserialize, Resource)]
#[uuid = "a7c64b93-4d6e-4420-b8c1-df481dca9387"]
pub struct GameConfig {
    pub camera: Camera,
}

//Reflect, FromReflect,
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[reflect(Serialize, Deserialize)]
pub struct Camera {
    pub fixed_angle: FixedAngle,
    pub first_person: FirstPerson,
    pub third_person: ThirdPerson,
    pub mouse_sensitivity_x: f32,
    pub mouse_sensitivity_y: f32,
}

// Reflect, FromReflect,
#[derive(Default, Debug, Clone, PartialEq,Serialize, Deserialize)]
// #[reflect(Serialize, Deserialize)]
pub struct FixedAngle {
    pub min_distance: f32,
    pub max_distance: f32,
    pub zoom_speed: f32,
    pub rotation_smoothing: f32,
    pub translation_smoothing: f32,
    pub zoom_in_smoothing: f32,
    pub zoom_out_smoothing: f32,
    pub pitch: f32,
}

//Reflect, FromReflect,
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[reflect(Serialize, Deserialize)]
pub struct FirstPerson {
    pub translation_smoothing: f32,
    pub rotation_smoothing: f32,
    pub max_pitch: f32,
    pub min_pitch: f32,
    pub tracking_smoothing: f32,
}

//Reflect, FromReflect, 
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[reflect(Serialize, Deserialize)]
pub struct ThirdPerson {
    pub translation_smoothing: f32,
    pub rotation_smoothing: f32,
    pub max_pitch: f32,
    pub min_pitch: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub zoom_speed: f32,
    pub min_distance_to_objects: f32,
    pub tracking_smoothing: f32,
    pub zoom_in_smoothing: f32,
    pub zoom_out_smoothing: f32,
}
