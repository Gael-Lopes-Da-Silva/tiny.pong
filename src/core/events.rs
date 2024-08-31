use bevy::prelude::*;

pub enum Scorer {
    Player,
    Enemy,
}

#[derive(Event)]
pub struct Scored(pub Scorer);
