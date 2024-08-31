use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreBoard;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Ball {
    pub position: Vec2,
    pub velocity: Vec2,
    pub shape: Vec2,
}

#[derive(Component)]
pub struct Paddle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub shape: Vec2,
}

#[derive(Component)]
pub struct Border {
    pub position: Vec2,
    pub shape: Vec2,
}

#[derive(Component)]
pub struct WinZone {
    pub position: Vec2,
    pub shape: Vec2,
}
