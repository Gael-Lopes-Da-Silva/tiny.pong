use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub player: u32,
    pub enemy: u32,
}
