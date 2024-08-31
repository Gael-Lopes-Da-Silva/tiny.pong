use bevy::math::bounding::*;

pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

pub fn check_collisions(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest_point = wall.closest_point(ball.center());
    let offset = ball.center() - closest_point;

    let collision = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0.0 {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.x > 0.0 {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(collision)
}
