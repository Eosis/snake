pub mod apples;
pub mod debug_mesh;
mod helpers;
pub mod snake;

use crate::pretty_rendering::debug_mesh::DebugMesh;
use crate::snake::{Direction, Snake};
use crate::{pretty_rendering, Apple, Game};
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

struct MainState {
    window_size: (f32, f32),
    game: Game,
    last_advance: Instant,
}

impl MainState {
    fn new(window_size: (f32, f32)) -> ggez::GameResult<MainState> {
        let mut apples = HashSet::new();
        for apple in &[(1, 0), (2, 0), (3, 0), (4, 0)] {
            apples.insert(Apple { location: *apple });
        }
        let s = MainState {
            window_size,
            game: Game {
                over: false,
                snake: Snake {
                    direction: Direction::Left,
                    lengthening: false,
                    body: VecDeque::from(vec![(10, 10), (10, 11), (10, 12), (10, 13), (10, 14)]),
                    confines: (20, 20),
                    confines_size: (window_size.0 - 60.0, window_size.1 - 60.0),
                },
                apples,
                width: 20,
                height: 20,
            },
            last_advance: Instant::now(),
        };
        Ok(s)
    }

    fn draw_border(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let (w, h) = self.window_size;
        let points = [
            na::Point2::new(15.0, 15.0),
            na::Point2::new(w - 15.0, 15.0),
            na::Point2::new(w - 15.0, h - 15.0),
            na::Point2::new(15.0, h - 15.0),
            na::Point2::new(15.0, 15.0),
        ];
        let border = graphics::Mesh::new_line(ctx, &points, 5.0, graphics::WHITE)?;
        graphics::draw(ctx, &border, (na::Point2::new(0.0, 0.0),))?;
        Ok(())
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

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        if self.last_advance.elapsed().as_secs_f32() >= 0.5 && !self.game.over {
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
                if let Some(new_dir) =
                    get_snake_direction_from_keypress(keycode, self.game.snake.direction)
                {
                    self.game.snake.direction = new_dir;
                }
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
