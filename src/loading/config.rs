use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};


// Reflect, FromReflect,
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Resource, TypeUuid)]
// #[reflect(Serialize, Deserialize, Resource)]
#[uuid = "a7c64b93-4d6e-4420-b8c1-df481dca9387"]
pub struct GameConfig {
    pub thing1: SomeThing,
}

//Reflect, FromReflect,
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[reflect(Serialize, Deserialize)]
pub struct SomeThing {
    pub v1: f32,
    pub v2: f32,
}

