use std::io::Cursor;

use ruscii::app::*;
use ruscii::drawing::*;
use ruscii::gui::*;
use ruscii::keyboard::*;
use ruscii::spatial::*;
use ruscii::terminal::*;

use rand::prelude::*;

use rodio::*;

static WIN: &[u8] = include_bytes!("assets/win.wav").as_slice();
static BOUNCE: &[u8] = include_bytes!("assets/bounce.wav").as_slice();
static EXPLOSION: &[u8] = include_bytes!("assets/explosion.wav").as_slice();
static START: &[u8] = include_bytes!("assets/start.wav").as_slice();

enum States {
    Start,
    Playing,
    Paused,
    NextRound,
    Win,
}

struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

struct Entity {
    position: Vector2,
    velocity: Vector2,
    size: Vector2,
    score: i32,
}

struct Ball {
    position: Vector2,
    velocity: Vector2,
}

struct Game {
    dimension: Vec2,
    left_player: Entity,
    right_player: Entity,
    ball: Ball,
    state: States,
    winner: i8,
    round: i32,
    collid_count: i32,
    bot_difficulty: i32,
    stream_handle: OutputStreamHandle,
}

impl Game {
    fn new(dimension: Vec2, stream_handle: OutputStreamHandle) -> Self {
        Self {
            dimension,
            left_player: Entity {
                position: Vector2::zero(),
                velocity: Vector2::zero(),
                size: Vector2::new(3.0, 8.0),
                score: 0,
            },
            right_player: Entity {
                position: Vector2::zero(),
                velocity: Vector2::zero(),
                size: Vector2::new(3.0, 8.0),
                score: 0,
            },
            ball: Ball {
                position: Vector2::new(dimension.x as f64 / 2.0, dimension.y as f64 / 2.0),
                velocity: Vector2::zero(),
            },
            state: States::Start,
            winner: 0,
            round: 0,
            collid_count: 0,
            bot_difficulty: 1,
            stream_handle,
        }
    }

    fn setup(&mut self) {
        self.random_ball_velocity();

        self.left_player.position = Vector2::new(0.0, self.dimension.y as f64 / 2.0);
        self.right_player.position = Vector2::new(
            self.dimension.x as f64 - self.right_player.size.x,
            self.dimension.y as f64 / 2.0,
        );
    }

    fn update_ball(&mut self) {
        self.ball.position.x += self.ball.velocity.x;
        self.ball.position.y += self.ball.velocity.y;
    }

    fn update_entities(&mut self) {
        self.left_player.position.x += self.left_player.velocity.x;
        self.left_player.position.y += self.left_player.velocity.y;

        self.left_player.position.y = self.left_player.position.y.clamp(
            0.0 + self.left_player.size.y / 2.0,
            self.dimension.y as f64 - self.left_player.size.y / 2.0,
        );

        self.left_player.velocity.y = 0.0;

        self.right_player.position.x += self.right_player.velocity.x;
        self.right_player.position.y += self.right_player.velocity.y;

        self.right_player.position.y = self.right_player.position.y.clamp(
            0.0 + self.right_player.size.y / 2.0,
            self.dimension.y as f64 - self.right_player.size.y / 2.0,
        );

        self.right_player.velocity.y = 0.0;
    }

    fn update_bot(&mut self, player_bot: bool) {
        if self.ball.position.x > self.dimension.x as f64 / 2.0 {
            if self.ball.position.y < self.right_player.position.y {
                self.right_player.velocity.y = -self.bot_difficulty as f64;
            }

            if self.ball.position.y > self.right_player.position.y {
                self.right_player.velocity.y = self.bot_difficulty as f64;
            }
        } else {
            if self.right_player.position.y < self.dimension.y as f64 / 2.0 - 1.0 {
                self.right_player.velocity.y = self.bot_difficulty as f64;
            }

            if self.right_player.position.y > self.dimension.y as f64 / 2.0 + 1.0 {
                self.right_player.velocity.y = -self.bot_difficulty as f64;
            }
        }

        if !player_bot {
            return;
        }

        if self.ball.position.x < self.dimension.x as f64 / 2.0 {
            if self.ball.position.y < self.left_player.position.y {
                self.left_player.velocity.y = -self.bot_difficulty as f64;
            }

            if self.ball.position.y > self.left_player.position.y {
                self.left_player.velocity.y = self.bot_difficulty as f64;
            }
        } else {
            if self.left_player.position.y < self.dimension.y as f64 / 2.0 - 1.0 {
                self.left_player.velocity.y = self.bot_difficulty as f64;
            }

            if self.left_player.position.y > self.dimension.y as f64 / 2.0 + 1.0 {
                self.left_player.velocity.y = -self.bot_difficulty as f64;
            }
        }
    }

    fn check_collisions(&mut self) {
        if self.ball.position.y <= 0.0 {
            self.ball.velocity.y *= -1.0;
        }

        if self.ball.position.y >= (self.dimension.y - 1) as f64 {
            self.ball.velocity.y *= -1.0;
        }

        if self.ball.position.x <= self.left_player.position.x + self.left_player.size.x
            && self.ball.position.y <= self.left_player.position.y + self.left_player.size.y / 2.0
            && self.ball.position.y >= self.left_player.position.y - self.left_player.size.y / 2.0
        {
            self.ball.velocity.x *= -1.0;
            self.collid_count += 1;

            self.play_sound("bounce");
        }

        if self.ball.position.x >= self.right_player.position.x - 1.0
            && self.ball.position.y <= self.right_player.position.y + self.right_player.size.y / 2.0
            && self.ball.position.y >= self.right_player.position.y - self.right_player.size.y / 2.0
        {
            self.ball.velocity.x *= -1.0;
            self.collid_count += 1;

            self.play_sound("bounce");
        }
    }

    fn check_scored(&mut self) {
        let mut scored = false;

        if self.ball.position.x <= 0.0 {
            self.right_player.score += 1;
            scored = true;
        }

        if self.ball.position.x >= (self.dimension.x - 1) as f64 {
            self.left_player.score += 1;
            scored = true;
        }

        if scored {
            self.ball.position.x = self.dimension.x as f64 / 2.0;
            self.ball.position.y = self.dimension.y as f64 / 2.0;

            self.random_ball_velocity();
            self.state = States::NextRound;

            self.round += 1;
            self.collid_count = 0;
            self.bot_difficulty = 1;

            self.left_player.position.y = self.dimension.y as f64 / 2.0;
            self.right_player.position.y = self.dimension.y as f64 / 2.0;

            self.play_sound("explosion");
        }
    }

    fn check_win(&mut self) {
        if self.left_player.score >= 3 || self.right_player.score >= 3 {
            self.state = States::Win;

            if self.left_player.score >= 3 {
                self.winner = 1;
            }

            if self.right_player.score >= 3 {
                self.winner = 2;
            }

            self.ball.position.x = self.dimension.x as f64 / 2.0;
            self.ball.position.y = self.dimension.y as f64 / 2.0;

            self.random_ball_velocity();

            self.left_player.position.y = self.dimension.y as f64 / 2.0;
            self.right_player.position.y = self.dimension.y as f64 / 2.0;

            self.left_player.score = 0;
            self.right_player.score = 0;
            self.round = 0;
            self.collid_count = 0;
            self.bot_difficulty = 1;

            self.play_sound("win");
        }
    }

    fn random_ball_velocity(&mut self) {
        let mut rng = rand::thread_rng();

        let x: bool = rng.gen();
        let y: bool = rng.gen();

        self.ball.velocity =
            Vector2::new(if x { 1.0 } else { -1.0 } * 2.0, if y { -1.0 } else { 1.0 });
    }

    fn play_sound(&mut self, sound: &str) {
        self.stream_handle
            .play_raw(
                Decoder::new(Cursor::new(match sound {
                    "win" => WIN,
                    "bounce" => BOUNCE,
                    "explosion" => EXPLOSION,
                    "start" => START,
                    _ => b"",
                }))
                .unwrap()
                .convert_samples(),
            )
            .unwrap();
    }
}

fn main() {
    let mut app = App::config(Config { fps: 60 });
    let mut fps_counter = FPSCounter::default();

    let mut show_infos = false;
    let mut player_bot = false;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut game = Game::new(app.window().size(), stream_handle);
    game.setup();

    app.run(|state: &mut State, window: &mut Window| {
        for key in state.keyboard().last_key_events() {
            match key {
                KeyEvent::Pressed(Key::Esc) => state.stop(),
                KeyEvent::Pressed(Key::Q) => state.stop(),
                KeyEvent::Pressed(Key::F3) => show_infos = !show_infos,
                KeyEvent::Pressed(Key::B) => player_bot = !player_bot,
                KeyEvent::Pressed(Key::Space) => match game.state {
                    States::Start => {
                        game.state = States::Playing;
                        game.play_sound("start");
                    }
                    States::Paused | States::NextRound | States::Win => {
                        game.state = States::Playing;
                    }
                    States::Playing => {
                        game.state = States::Paused;
                    }
                },
                KeyEvent::Pressed(Key::F) => match game.state {
                    States::Playing => {
                        game.left_player.score = 3;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if !player_bot {
            for key in state.keyboard().get_keys_down() {
                match key {
                    Key::Up | Key::K => game.left_player.velocity.y = -1.0,
                    Key::Down | Key::J => game.left_player.velocity.y = 1.0,
                    _ => {}
                }
            }
        }

        fps_counter.update();

        let mut scoreboard = format!(
            " LEFT {}     {} RIGHT",
            game.left_player.score, game.right_player.score
        );

        match game.state {
            States::Playing => {
                game.update_ball();
                game.update_entities();
                game.update_bot(player_bot);
                game.check_collisions();
                game.check_scored();
                game.check_win();

                if game.collid_count == 5 {
                    game.ball.velocity.x *= 1.5;
                    game.ball.velocity.y *= 1.5;

                    game.bot_difficulty += 1;
                    game.collid_count += 1;
                }

                if game.collid_count == 15 {
                    game.ball.velocity.x *= 1.5;
                    game.ball.velocity.y *= 1.5;

                    game.bot_difficulty += 1;
                    game.collid_count += 1;
                }
            }
            States::Start => {
                scoreboard = "Press space to start".to_string();
            }
            States::Paused => {
                scoreboard = "Game paused".to_string();
            }
            States::NextRound => {
                scoreboard = "Next round".to_string();
            }
            States::Win => {
                if game.winner == 1 {
                    scoreboard = "Red win".to_string();
                }

                if game.winner == 2 {
                    scoreboard = "Blue win".to_string();
                }
            }
        }

        let mut pencil = Pencil::new(window.canvas_mut());

        pencil.set_foreground(Color::White);
        pencil.set_style(Style::Plain);
        pencil.draw_rect(
            &RectCharset::simple_round_lines(),
            Vec2::zero(),
            game.dimension,
        );

        pencil.set_foreground(Color::White);
        pencil.set_style(Style::Plain);
        pencil.draw_vline('|', Vec2::xy(game.dimension.x / 2, 2), game.dimension.y - 4);

        pencil.set_foreground(Color::Red);
        pencil.set_style(Style::Plain);
        pencil.draw_rect(
            &RectCharset::simple_round_lines(),
            Vec2::xy(
                game.left_player.position.x,
                game.left_player.position.y - game.left_player.size.y / 2.0,
            ),
            Vec2::xy(game.left_player.size.x, game.left_player.size.y),
        );

        pencil.set_foreground(Color::Blue);
        pencil.set_style(Style::Plain);
        pencil.draw_rect(
            &RectCharset::simple_round_lines(),
            Vec2::xy(
                game.right_player.position.x,
                game.right_player.position.y - game.right_player.size.y / 2.0,
            ),
            Vec2::xy(game.right_player.size.x, game.right_player.size.y),
        );

        pencil.set_foreground(Color::Yellow);
        pencil.set_style(Style::Bold);
        pencil.draw_char('⬤', Vec2::xy(game.ball.position.x, game.ball.position.y));

        pencil.set_foreground(Color::Yellow);
        pencil.set_style(Style::Bold);
        pencil.draw_text(
            &scoreboard,
            Vec2::xy(game.dimension.x / 2 - scoreboard.len() as i32 / 2, 1),
        );

        match game.state {
            States::Win => {
                let label = "Press space to restart the game !";

                pencil.set_foreground(Color::Green);
                pencil.set_style(Style::Bold);
                pencil.draw_text(
                    &label,
                    Vec2::xy(
                        game.dimension.x / 2 - label.len() as i32 / 2,
                        game.dimension.y / 2,
                    ),
                );
            }
            _ => {}
        }

        if show_infos {
            pencil.set_foreground(Color::White);
            pencil.set_style(Style::Plain);
            pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(2, 1));
            pencil.draw_text(&format!("ROUND: {}", game.round), Vec2::xy(2, 2));
            pencil.draw_text(&format!("COLLID: {}", game.collid_count), Vec2::xy(2, 3));
            pencil.draw_text(
                &format!("DIFFICULTY: {}", game.bot_difficulty),
                Vec2::xy(2, 4),
            );
            pencil.draw_text(&format!("PLAYER_BOT: {}", player_bot), Vec2::xy(2, 4));
        }
    });
}
