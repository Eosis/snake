pub mod apples;
pub mod debug_mesh;
mod helpers;
pub mod snake;

use crate::pretty_rendering::debug_mesh::DebugMesh;
use crate::snake::{Direction, Snake};
use crate::{pretty_rendering, Game};
use ggez::conf::WindowMode;
use ggez::event;
use ggez::event::quit;
use ggez::graphics;
use ggez::graphics::{DrawParam, Drawable};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::nalgebra as na;
use std::collections::{HashSet, VecDeque};
use std::time::Instant;

const DEBUG: bool = true;
const SECONDS_BETWEEN_FRAMES: f32 = 0.2;

struct MainState {
    window_size: (f32, f32),
    game: Game,
    last_advance: Instant,
}

impl MainState {
    fn new(window_size: (f32, f32)) -> ggez::GameResult<MainState> {
        let s = MainState {
            window_size,
            game: get_starting_game(window_size),
            last_advance: Instant::now(),
        };
        Ok(s)
    }

    /// Draw our arena wall (collision with which means a dead Snake).
    fn draw_border(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let (w, h) = self.window_size;
        let border = ggez::graphics::Rect::new(30.0, 30.0, w - 60.0, h - 60.0);
        let line_width = 5.0;

        let points = [
            na::Point2::new(border.x, border.y),
            na::Point2::new(border.x + border.w, border.y),
            na::Point2::new(border.x + border.w, border.y + border.h),
            na::Point2::new(border.x, border.y + border.h),
            na::Point2::new(border.x, border.y - line_width / 2.0), // Fill in remaining 'notch' in top left corner
        ];

        let border = graphics::Mesh::new_line(ctx, &points, line_width, graphics::WHITE)?;
        graphics::draw(ctx, &border, (na::Point2::new(0.0, 0.0),))?;
        Ok(())
    }

    fn draw_score(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let text = ggez::graphics::Text::new(format!("{:03}", self.game.score));
        graphics::draw(ctx, &text, (na::Point2::new(self.window_size.0 - 100.0, 0.0 as f32),))
    }
}

fn get_snake_direction_from_keypress(
    input: KeyCode,
    current_direction: Direction,
) -> Option<Direction> {
    match input {
        KeyCode::Up => match current_direction {
            Direction::Down => None,
            _ => Some(Direction::Up),
        },
        KeyCode::Right => match current_direction {
            Direction::Left => None,
            _ => Some(Direction::Right),
        },
        KeyCode::Down => match current_direction {
            Direction::Up => None,
            _ => Some(Direction::Down),
        },
        KeyCode::Left => match current_direction {
            Direction::Right => None,
            _ => Some(Direction::Left),
        },
        _ => None,
    }
}

fn get_starting_game(window_size: (f32, f32)) -> Game {
    let mut game = Game {
        over: false,
        snake: Snake {
            direction: Direction::Left,
            lengthening: false,
            body: VecDeque::from(vec![(10, 10), (10, 11), (10, 12), (10, 13), (10, 14)]),
            confines: (20, 20),
            confines_size: (window_size.0 * 9.0 / 10.0, window_size.1 * 9.0 / 10.0),
        },
        apples: HashSet::new(),
        width: 20,
        height: 20,
        score: 0,
        direction_changed_this_tick: false,
    };
    game.add_new_apple();
    game
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        if self.last_advance.elapsed().as_secs_f32() >= SECONDS_BETWEEN_FRAMES && !self.game.over {
            self.game.advance();
            if self.game.snake.dead() {
                self.game.over = true;
            }
            self.last_advance = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let play_area = na::Point2::new(30.0, 30.0);
        let container = graphics::Rect::new(30.0, 30.0, 540.0, 540.0);
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        self.game
            .snake
            .draw(ctx, DrawParam::default().dest(play_area))?;
        let apples = pretty_rendering::apples::Apples::new(
            &self.game.apples,
            container,
            self.game.snake.confines,
        );
        apples.draw(ctx, DrawParam::default().dest(play_area))?;

        self.draw_border(ctx)?;
        self.draw_score(ctx)?;
        if DEBUG {
            let mesh = DebugMesh {
                rows: self.game.snake.confines.0 as usize,
                columns: self.game.snake.confines.1 as usize,
                container,
            };
            mesh.draw(ctx, DrawParam::default().dest(play_area))?;
        }
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Up | KeyCode::Right | KeyCode::Down | KeyCode::Left => {
                if self.game.direction_changed_this_tick {
                    return;
                }
                if let Some(new_dir) =
                    get_snake_direction_from_keypress(keycode, self.game.snake.direction)
                {
                    self.game.snake.direction = new_dir;
                    self.game.direction_changed_this_tick = true;
                }
            }
            KeyCode::R => {
                self.game = get_starting_game(self.window_size);
            }
            KeyCode::Escape => {
                quit(ctx);
            }
            _ => (),
        }
    }
}

pub fn ggez_main() -> ggez::GameResult {
    let window_size = (600.0, 600.0);
    let cb = ggez::ContextBuilder::new("snakin'", "Rups").window_mode(WindowMode {
        width: window_size.0,
        height: window_size.1,
        ..Default::default()
    });
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(window_size)?;
    event::run(ctx, event_loop, state)
}
