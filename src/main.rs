use bevy::prelude::*;

mod core;
use core::events::*;
use core::resources::*;
use core::systems::*;

fn main() {
    App::new()
        .init_resource::<Score>()
        .add_plugins(DefaultPlugins)
        .add_event::<Scored>()
        .add_systems(
            Startup,
            (
                spawn_camera,
                spawn_ball,
                spawn_paddles,
                spawn_borders,
                spawn_winzones,
                spawn_scoreboards,
            ),
        )
        .add_systems(
            Update,
            (
                update_ball_position,
                update_paddles_position,
                update_borders_position,
                update_winzones_position,
                handle_collisions.after(update_ball_position).after(update_paddles_position),
                handle_input,
                handle_score,
                reset_ball.after(handle_score),
                update_score.after(handle_score),
                update_scoreboards.after(update_score),
            ),
        )
        .run();
}
