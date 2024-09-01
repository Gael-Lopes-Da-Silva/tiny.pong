use ruscii::app::*;
use ruscii::drawing::*;
use ruscii::gui::*;
use ruscii::keyboard::*;
use ruscii::spatial::*;
use ruscii::terminal::*;

use rand::prelude::*;

enum States {
    Start,
    Playing,
    Paused,
    NextRound,
    Win,
}

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
    ball: Ball,
    state: States,
    winner: i8,
    round: i32,
    collid_count: i32,
    bot_difficulty: i32,
}

impl Game {
    fn new(screen: Vec2) -> Self {
        Self {
            dimension: screen,
            left_player: Entity {
                position: Vec2::zero(),
                velocity: Vec2::zero(),
                size: Vec2::xy(3, 8),
                score: 0,
            },
            right_player: Entity {
                position: Vec2::zero(),
                velocity: Vec2::zero(),
                size: Vec2::xy(3, 8),
                score: 0,
            },
            ball: Ball {
                position: screen / 2,
                velocity: Vec2::xy(0, 0),
            },
            state: States::Start,
            winner: 0,
            round: 0,
            collid_count: 0,
            bot_difficulty: 1,
        }
    }

    fn update(&mut self) {
        self.ball.position += self.ball.velocity;

        self.left_player.position += self.left_player.velocity;
        self.left_player.position.y = self.left_player.position.y.clamp(
            0 + self.left_player.size.y / 2,
            self.dimension.y - self.left_player.size.y / 2,
        );

        self.right_player.position += self.right_player.velocity;
        self.right_player.position.y = self.right_player.position.y.clamp(
            0 + self.right_player.size.y / 2,
            self.dimension.y - self.right_player.size.y / 2,
        );

        self.check_collisions();

        self.left_player.velocity.y = 0;
        self.right_player.velocity.y = 0;
    }

    fn update_bot(&mut self) {
        if self.ball.position.x > self.dimension.x / 2 {
            if self.ball.position.y < self.right_player.position.y {
                self.right_player.velocity.y = -self.bot_difficulty;
            }

            if self.ball.position.y > self.right_player.position.y {
                self.right_player.velocity.y = self.bot_difficulty;
            }
        } else {
            if self.right_player.position.y < self.dimension.y / 2 {
                self.right_player.velocity.y = self.bot_difficulty;
            }

            if self.right_player.position.y > self.dimension.y / 2 {
                self.right_player.velocity.y = -self.bot_difficulty;
            }
        }
    }

    fn check_collisions(&mut self) {
        if self.ball.position.y <= 0 {
            self.ball.velocity.y *= -1;
        }

        if self.ball.position.y >= self.dimension.y - 1 {
            self.ball.velocity.y *= -1;
        }

        if self.ball.position.x <= self.left_player.position.x + self.left_player.size.x
            && self.ball.position.y <= self.left_player.position.y + self.left_player.size.y / 2
            && self.ball.position.y >= self.left_player.position.y - self.left_player.size.y / 2
        {
            self.ball.velocity.x *= -1;
            self.collid_count += 1;
        }

        if self.ball.position.x >= self.right_player.position.x - 1
            && self.ball.position.y <= self.right_player.position.y + self.right_player.size.y / 2
            && self.ball.position.y >= self.right_player.position.y - self.right_player.size.y / 2
        {
            self.ball.velocity.x *= -1;
            self.collid_count += 1;
        }

        if self.ball.position.x <= 0 {
            self.ball.position = self.dimension / 2;
            self.random_ball_velocity();
            self.state = States::NextRound;

            self.right_player.score += 1;
            self.round += 1;
            self.collid_count = 0;
            self.bot_difficulty = 1;

            self.left_player.position.y = self.dimension.y / 2;
            self.right_player.position.y = self.dimension.y / 2;
        }

        if self.ball.position.x >= self.dimension.x - 1 {
            self.ball.position = self.dimension / 2;
            self.random_ball_velocity();
            self.state = States::NextRound;

            self.left_player.score += 1;
            self.round += 1;
            self.collid_count = 0;
            self.bot_difficulty = 1;

            self.left_player.position.y = self.dimension.y / 2;
            self.right_player.position.y = self.dimension.y / 2;
        }
    }

    fn random_ball_velocity(&mut self) {
        let mut rng = rand::thread_rng();

        let x: bool = rng.gen();
        let y: bool = rng.gen();

        self.ball.velocity = Vec2::xy(if x { 1 } else { -1 } * 2, if y { -1 } else { 1 });
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

            self.ball.position = self.dimension / 2;
            self.random_ball_velocity();

            self.left_player.position.y = self.dimension.y / 2;
            self.right_player.position.y = self.dimension.y / 2;

            self.left_player.score = 0;
            self.right_player.score = 0;
            self.round = 0;
            self.collid_count = 0;
            self.bot_difficulty = 1;
        }
    }
}

fn main() {
    let mut app = App::default();
    let mut fps = FPSCounter::default();

    let mut show_infos = false;

    let mut game = Game::new(app.window().size());
    game.random_ball_velocity();

    game.left_player.position = Vec2::xy(0, game.dimension.y / 2);
    game.right_player.position = Vec2::xy(
        game.dimension.x - game.right_player.size.x,
        game.dimension.y / 2,
    );

    app.run(|state: &mut State, window: &mut Window| {
        for key in state.keyboard().last_key_events() {
            match key {
                KeyEvent::Pressed(Key::Q) => state.stop(),
                KeyEvent::Pressed(Key::Esc) => state.stop(),
                KeyEvent::Pressed(Key::F3) => show_infos = !show_infos,
                KeyEvent::Pressed(Key::Space) => match game.state {
                    States::Start | States::Paused | States::NextRound | States::Win => {
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

        for key in state.keyboard().get_keys_down() {
            match key {
                Key::Up | Key::K => game.left_player.velocity.y = -1,
                Key::Down | Key::J => game.left_player.velocity.y = 1,
                _ => {}
            }
        }

        fps.update();

        let mut scoreboard = format!(
            " LEFT {}     {} RIGHT",
            game.left_player.score, game.right_player.score
        );

        match game.state {
            States::Playing => {
                game.update();
                game.update_bot();
                game.check_win();

                if game.collid_count == 5 {
                    game.ball.velocity *= 2;
                    game.bot_difficulty += 1;
                    game.collid_count += 1;
                }

                if game.collid_count == 15 {
                    game.ball.velocity *= 2;
                    game.bot_difficulty += 1;
                    game.collid_count += 1;
                }

                if game.collid_count == 35 {
                    game.ball.velocity *= 2;
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
            game.left_player.position - Vec2::y(game.left_player.size.y / 2),
            game.left_player.size,
        );

        pencil.set_foreground(Color::Blue);
        pencil.set_style(Style::Plain);
        pencil.draw_rect(
            &RectCharset::simple_round_lines(),
            game.right_player.position - Vec2::y(game.right_player.size.y / 2),
            game.right_player.size,
        );

        pencil.set_foreground(Color::Yellow);
        pencil.set_style(Style::Bold);
        pencil.draw_char('⬤', game.ball.position);

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
            pencil.draw_text(&format!("FPS: {}", fps.count()), Vec2::xy(2, 1));
            pencil.draw_text(&format!("ROUND: {}", game.round), Vec2::xy(2, 2));
            pencil.draw_text(&format!("COLLID: {}", game.collid_count), Vec2::xy(2, 3));
            pencil.draw_text(
                &format!("DIFFICULTY: {}", game.bot_difficulty),
                Vec2::xy(2, 4),
            );
        }
    });
}
