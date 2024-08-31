use crate::core::components::*;
use crate::core::events::*;
use crate::core::resources::*;
use crate::core::utils::*;
use bevy::math::bounding::*;
use bevy::prelude::*;
use bevy::sprite::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = 15.0;

    commands
        .spawn_empty()
        .insert(Transform::default())
        .insert(Ball {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(5.0, 5.0),
            shape: Vec2::new(radius, radius),
        })
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Circle::new(radius))).into(),
            material: materials.add(ColorMaterial::from(Color::srgb(1.0, 1.0, 1.0))),
            ..default()
        });
}

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let width = 30.0;
    let height = 90.0;
    let padding = 20.0;

    let player_x = -500.0 + width / 2.0 + padding;
    let enemy_x = 500.0 - width / 2.0 - padding;

    commands
        .spawn_empty()
        .insert(Transform::default())
        .insert(Player {})
        .insert(Paddle {
            position: Vec2::new(player_x, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            shape: Vec2::new(width, height),
        })
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Rectangle::new(width, height))).into(),
            material: materials.add(ColorMaterial::from(Color::srgb(1.0, 1.0, 1.0))),
            ..default()
        });

    commands
        .spawn_empty()
        .insert(Transform::default())
        .insert(Enemy {})
        .insert(Paddle {
            position: Vec2::new(enemy_x, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            shape: Vec2::new(width, height),
        })
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Rectangle::new(width, height))).into(),
            material: materials.add(ColorMaterial::from(Color::srgb(1.0, 1.0, 1.0))),
            ..default()
        });
}

pub fn spawn_borders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let height = 20.0;
    let width = 1000.0;

    let top_border_y = 300.0;
    let bottom_border_y = -300.0;

    commands
        .spawn_empty()
        .insert(Transform::default())
        .insert(Border {
            position: Vec2::new(0.0, top_border_y),
            shape: Vec2::new(width, height),
        })
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Rectangle::new(width, height))).into(),
            material: materials.add(ColorMaterial::from(Color::srgb(1.0, 0.0, 0.0))),
            ..default()
        });

    commands
        .spawn_empty()
        .insert(Transform::default())
        .insert(Border {
            position: Vec2::new(0.0, bottom_border_y),
            shape: Vec2::new(width, height),
        })
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Rectangle::new(width, height))).into(),
            material: materials.add(ColorMaterial::from(Color::srgb(1.0, 0.0, 0.0))),
            ..default()
        });
}

pub fn spawn_winzones(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let height = 580.0;
    let width = 20.0;

    let left_winzone_x = 500.0 + width / 2.0;
    let right_winzone_x = -500.0 - width / 2.0;

    commands
        .spawn_empty()
        .insert(Transform::default())
        .insert(Player {})
        .insert(WinZone {
            position: Vec2::new(left_winzone_x, 0.0),
            shape: Vec2::new(width, height),
        })
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Rectangle::new(width, height))).into(),
            material: materials.add(ColorMaterial::from(Color::srgb(0.0, 1.0, 0.0))),
            ..default()
        });

    commands
        .spawn_empty()
        .insert(Transform::default())
        .insert(Enemy {})
        .insert(WinZone {
            position: Vec2::new(right_winzone_x, 0.0),
            shape: Vec2::new(width, height),
        })
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Rectangle::new(width, height))).into(),
            material: materials.add(ColorMaterial::from(Color::srgb(0.0, 1.0, 0.0))),
            ..default()
        });
}

pub fn spawn_scoreboards(mut commands: Commands) {
    commands
        .spawn_empty()
        .insert(Player {})
        .insert(ScoreBoard {})
        .insert(
            TextBundle::from("Player: 0")
                .with_text_justify(JustifyText::Center)
                .with_background_color(Color::srgb(0.0, 0.0, 0.0))
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                }),
        );

    commands
        .spawn_empty()
        .insert(Enemy {})
        .insert(ScoreBoard {})
        .insert(
            TextBundle::from("Enemy: 0")
                .with_text_justify(JustifyText::Center)
                .with_background_color(Color::srgb(0.0, 0.0, 0.0))
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..default()
                }),
        );
}

pub fn update_ball_position(mut ball_query: Query<(&mut Transform, &mut Ball)>) {
    for (mut transform, mut ball) in ball_query.iter_mut() {
        ball.position.x += ball.velocity.x;
        ball.position.y += ball.velocity.y;

        transform.translation = ball.position.extend(0.0);
    }
}

pub fn update_paddles_position(mut paddles_query: Query<(&mut Transform, &mut Paddle)>) {
    for (mut transform, mut paddle) in paddles_query.iter_mut() {
        paddle.position.x += paddle.velocity.x;
        paddle.position.y += paddle.velocity.y;

        let top_position = 300.0 - paddle.shape.y / 2.0 - 20.0 / 2.0;
        if paddle.position.y >= top_position {
            paddle.position.y = top_position;
        }

        let bottom_position = -300.0 + paddle.shape.y / 2.0 + 20.0 / 2.0;
        if paddle.position.y <= bottom_position {
            paddle.position.y = bottom_position;
        }

        transform.translation = paddle.position.extend(0.0);
    }
}

pub fn update_borders_position(mut borders_query: Query<(&mut Transform, &Border)>) {
    for (mut transform, border) in borders_query.iter_mut() {
        transform.translation = border.position.extend(0.0);
    }
}

pub fn update_winzones_position(mut winzones_query: Query<(&mut Transform, &WinZone)>) {
    for (mut transform, winzone) in winzones_query.iter_mut() {
        transform.translation = winzone.position.extend(0.0);
    }
}

pub fn handle_collisions(
    mut ball_query: Query<&mut Ball>,
    paddles_query: Query<&Paddle>,
    borders_query: Query<&Border>,
) {
    for mut ball in ball_query.iter_mut() {
        for paddle in paddles_query.iter() {
            if let Some(collision) = check_collisions(
                BoundingCircle::new(ball.position, ball.shape.x),
                Aabb2d::new(paddle.position, paddle.shape / 2.0),
            ) {
                match collision {
                    Collision::Left => ball.velocity.x *= -1.0,
                    Collision::Right => ball.velocity.x *= -1.0,
                    Collision::Top => ball.velocity.y *= -1.0,
                    Collision::Bottom => ball.velocity.y *= -1.0,
                }
            }
        }

        for border in borders_query.iter() {
            if let Some(collision) = check_collisions(
                BoundingCircle::new(ball.position, ball.shape.x),
                Aabb2d::new(border.position, border.shape / 2.0),
            ) {
                match collision {
                    Collision::Left => ball.velocity.x *= -1.0,
                    Collision::Right => ball.velocity.x *= -1.0,
                    Collision::Top => ball.velocity.y *= -1.0,
                    Collision::Bottom => ball.velocity.y *= -1.0,
                }
            }
        }
    }
}

pub fn handle_input(
    mut player_query: Query<&mut Paddle, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for mut player in player_query.iter_mut() {
        if input.pressed(KeyCode::ArrowUp) {
            player.velocity.y = 5.0;
        } else if input.pressed(KeyCode::ArrowDown) {
            player.velocity.y = -5.0;
        } else {
            player.velocity.y = 0.0;
        }
    }
}

pub fn handle_score(
    ball_query: Query<&Ball>,
    winzones_query: Query<(&WinZone, Option<&Player>, Option<&Enemy>)>,
    mut events: EventWriter<Scored>,
) {
    for ball in ball_query.iter() {
        for (winzone, player, enemy) in winzones_query.iter() {
            if BoundingCircle::new(ball.position, ball.shape.x)
                .intersects(&Aabb2d::new(winzone.position, winzone.shape / 2.0))
            {
                if player.is_some() {
                    events.send(Scored(Scorer::Player));
                } else if enemy.is_some() {
                    events.send(Scored(Scorer::Enemy));
                }
            }
        }
    }
}

pub fn update_score(mut score: ResMut<Score>, mut events: EventReader<Scored>) {
    for event in events.read() {
        match event.0 {
            Scorer::Player => score.player += 1,
            Scorer::Enemy => score.enemy += 1,
        }
    }
}

pub fn reset_ball(mut ball_query: Query<&mut Ball>, mut events: EventReader<Scored>) {
    for mut ball in ball_query.iter_mut() {
        for event in events.read() {
            match event.0 {
                Scorer::Player => {
                    ball.position = Vec2::new(0.0, 0.0);
                    ball.velocity = Vec2::new(-5.0, 5.0);
                }
                Scorer::Enemy => {
                    ball.position = Vec2::new(0.0, 0.0);
                    ball.velocity = Vec2::new(5.0, 5.0);
                }
            }
        }
    }
}

pub fn update_scoreboards(
    score: Res<Score>,
    mut scoreboards_query: Query<(&mut Text, Option<&Player>, Option<&Enemy>), With<ScoreBoard>>,
    mut events: EventReader<Scored>,
) {
    for (mut text, player, enemy) in scoreboards_query.iter_mut() {
        for event in events.read() {
            match event.0 {
                Scorer::Player => {
                    if player.is_some() {
                        text.sections[0].value = format!("Player: {}", score.player);
                    }
                }
                Scorer::Enemy => {
                    if enemy.is_some() {
                        text.sections[0].value = format!("Enemy: {}", score.enemy);
                    }
                }
            }
        }
    }
}
