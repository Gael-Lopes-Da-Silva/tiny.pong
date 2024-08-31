use ruscii::app::*;
use ruscii::drawing::*;
use ruscii::gui::*;
use ruscii::keyboard::*;
use ruscii::spatial::*;
use ruscii::terminal::*;

struct Entity {
    position: Vec2,
    velocity: Vec2,
    size: Vec2,
    score: i32,
}

struct Ball {
    position: Vec2,
    velocity: Vec2,
}

struct Game {
    dimension: Vec2,
    left_player: Entity,
    right_player: Entity,
}

impl Game {
    fn new(dimension: Vec2) -> Self {
        Self {
            dimension: dimension,
            left_player: Entity {
                position: Vec2::xy(1, dimension.y / 2),
                velocity: Vec2::zero(),
                size: Vec2::xy(2, 6),
                score: 0,
            },
            right_player: Entity {
                position: Vec2::xy(dimension.x - 3, dimension.y / 2),
                velocity: Vec2::zero(),
                size: Vec2::xy(2, 6),
                score: 0,
            },
        }
    }
}

fn main() {
    let mut app = App::default();
    let mut fps = FPSCounter::default();

    let mut show_infos = false;

    let mut game = Game::new(app.window().size());

    app.run(|state: &mut State, window: &mut Window| {
        for key in state.keyboard().last_key_events() {
            match key {
                KeyEvent::Pressed(Key::Esc) => state.stop(),
                KeyEvent::Pressed(Key::Q) => state.stop(),
                KeyEvent::Pressed(Key::F3) => show_infos = !show_infos,
                _ => {}
            }
        }

        let mut pencil = Pencil::new(window.canvas_mut());

        fps.update();
        if show_infos {
            pencil.draw_text(&format!("FPS: {}", fps.count()), Vec2::zero());
        }

        pencil.draw_vline('|', Vec2::xy(game.dimension.x / 2, 2), game.dimension.y - 2);

        pencil.draw_rect(&RectCharset::double_lines(), game.left_player.position - Vec2::y(game.left_player.size.y / 2), game.left_player.size);
        pencil.draw_rect(&RectCharset::double_lines(), game.right_player.position - Vec2::y(game.right_player.size.y / 2), game.right_player.size);
    });
}
