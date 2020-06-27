pub mod pretty_rendering;
pub mod snake;

use crate::snake::{Direction, Snake};
use std::collections::{HashSet, VecDeque};
use std::io::stdin;

struct Game {
    pub snake: Snake,
    pub apples: HashSet<Apple>,
    pub width: usize,
    pub last_advance: Instant,
    #[allow(dead_code)]
    height: usize,
    #[allow(dead_code)]
    rendered: Vec<Vec<char>>,
}

#[derive(PartialEq, Eq, Hash)]
struct Apple {
    location: (usize, usize),
}

impl Game {
    pub fn new(width: usize, height: usize, snake_body: &[(i32, i32)]) -> Game {
        Game {
            snake: Snake {
                body: VecDeque::from(
                    Vec::from(snake_body)
                        .iter()
                        .map(|(y, x)| (*y as usize, *x as usize))
                        .collect::<Vec<_>>(),
                ),
                lengthening: false,
                direction: Snake::head_direction(
                    snake_body
                        .iter()
                        .take(2)
                        .map(|(y, x)| (*y as usize, *x as usize))
                        .collect::<Vec<_>>()
                        .iter(),
                ),
                confines: (height, width),
                confines_size: (550.0, 550.0),
            },
            apples: HashSet::new(),
            rendered: vec![vec![' '; width]; height],
            width,
            height,
            last_advance: Instant::now(),
        }
    }

    pub fn advance(&mut self) {
        self.snake.advance();
        let head = self.snake.body[0];
        if self.apples.contains(&Apple { location: head }) {
            self.snake.lengthening = true;
            self.apples.remove(&Apple { location: head });
        }
    }

    pub fn render_with_border(&mut self) {
        self.render();
        println!("{}", "-".repeat(self.width + 2));
        for row in self.rendered.iter() {
            println!("|{}|", row.iter().collect::<String>());
        }
        println!("{}", "-".repeat(self.width + 2));
    }

    fn render(&mut self) {
        self.clear();
        self.render_apples();
        self.render_snake();
    }

    fn clear(&mut self) {
        for row in self.rendered.iter_mut() {
            for elem in row.iter_mut() {
                *elem = ' ';
            }
        }
    }

    fn render_snake(&mut self) {
        self.render_snake_head();
        self.render_snake_body();
        self.render_snake_tail();
    }

    fn render_snake_head(&mut self) {
        let direction = Snake::head_direction(self.snake.body.iter());
        let (y, x) = self.snake.body[0];
        let glyph = match direction {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        };
        self.rendered[y][x] = glyph;
    }

    fn render_snake_body(&mut self) {
        for window in Vec::from(self.snake.body.clone()).windows(3) {
            let to = Snake::direction(window[0], window[1]);
            let from = Snake::direction(window[1], window[2]);
            let joining_glyph = Snake::get_body_glyph_from_directions(to, from);
            let (y_to_set, x_to_set) = window[1];
            self.rendered[y_to_set][x_to_set] = joining_glyph;
        }
    }

    fn render_snake_tail(&mut self) {
        let relevant_points: Vec<_> = self
            .snake
            .body
            .iter()
            .rev()
            .take(2)
            .rev()
            .cloned()
            .collect();
        let direction = Snake::direction(relevant_points[0], relevant_points[1]);
        let (y, x) = self.snake.body.back().unwrap();
        let glyph = match direction {
            Direction::Up | Direction::Down => '║',
            Direction::Right | Direction::Left => '═',
        };
        self.rendered[*y][*x] = glyph;
    }

    fn render_apples(&mut self) {
        for apple in &self.apples {
            let (y, x) = apple.location;
            self.rendered[y][x] = 'O';
        }
    }

    fn set_snake_direction_from_input(input: char) -> Option<Direction> {
        match input {
            'w' => Some(Direction::Up),
            'a' => Some(Direction::Left),
            's' => Some(Direction::Down),
            'd' => Some(Direction::Right),
            _ => None,
        }
    }

    fn get_snake_direction_from_keypress(input: KeyCode) -> Option<Direction> {
        match input {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Right => Some(Direction::Right),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            _ => None,
        }
    }

    #[cfg(test)]
    pub fn render_to_string(&mut self) -> String {
        self.render();
        self.rendered
            .iter()
            .map(|row| row.iter().collect::<String>())
            .fold(String::new(), |mut init, add| {
                init.push_str(&add);
                init.push_str("\n");
                init
            })
    }
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let mut game = Game::new(20, 20, &[(10, 10), (10, 9), (10, 8), (10, 7), (10, 6)]);
//     game.apples = HashSet::new();
//     game.apples.insert(Apple { location: (4, 4) });
//     game.apples.insert(Apple { location: (9, 9) });
//     game.apples.insert(Apple { location: (9, 10) });
//
//     loop_game(game);
//     #[allow(unreachable_code)]
//     Ok(())
// }

fn loop_game(mut game: Game) -> ! {
    let mut line = String::new();
    loop {
        game.render_with_border();
        let _ = stdin().read_line(&mut line).unwrap();
        if !line.is_empty() {
            if let Some(direction) =
                Game::set_snake_direction_from_input(line.chars().next().unwrap())
            {
                game.snake.direction = direction;
            }
            line.clear();
        }
        game.advance();
    }
}

#[cfg(test)]
fn count_board_squares(game: Game) -> usize {
    game.rendered.iter().fold(0, |tot, row| tot + row.len())
}

#[test]
fn test_creating_boards() {
    assert_eq!(
        count_board_squares(Game::new(
            20,
            20,
            &[(10, 10), (10, 9), (10, 8), (10, 7), (10, 6)]
        )),
        20 * 20
    );
    assert_eq!(
        count_board_squares(Game::new(
            50,
            1,
            &[(10, 10), (10, 9), (10, 8), (10, 7), (10, 6)]
        )),
        50
    );
}

#[test]
fn test_direction() {
    let upwards_body = vec![(9, 10), (10, 10)];
    let downwards_body = vec![(11, 10), (10, 10)];
    let rightwards_body = vec![(10, 11), (10, 10)];
    let leftwards_body = vec![(10, 9), (10, 10)];
    assert_eq!(Snake::head_direction(upwards_body.iter()), Direction::Up);
    assert_eq!(
        Snake::head_direction(downwards_body.iter()),
        Direction::Down
    );
    assert_eq!(
        Snake::head_direction(rightwards_body.iter()),
        Direction::Right
    );
    assert_eq!(
        Snake::head_direction(leftwards_body.iter()),
        Direction::Left
    );
}

#[test]
fn test_drawing_simple_snakes() {
    #[rustfmt::skip]
    let correct_upwards = concat!(" ^ \n",
                                        " ║ \n",
                                        " ║ \n");
    #[rustfmt::skip]
    let correct_rightwards = concat!("   \n",
                                           "══>\n",
                                           "   \n");
    #[rustfmt::skip]
    let correct_leftwards = concat!("   \n",
                                          "<══\n",
                                          "   \n");
    #[rustfmt::skip]
    let correct_downwards = concat!(" ║ \n",
                                          " ║ \n",
                                          " v \n");
    let mut upwards_game = Game::new(3, 3, &[(0, 1), (1, 1), (2, 1)]);
    assert_eq!(upwards_game.render_to_string(), correct_upwards);

    let mut correct_rightwards_game = Game::new(3, 3, &[(1, 2), (1, 1), (1, 0)]);
    assert_eq!(
        correct_rightwards_game.render_to_string(),
        correct_rightwards
    );

    let mut leftwards_game = Game::new(3, 3, &[(1, 0), (1, 1), (1, 2)]);
    assert_eq!(leftwards_game.render_to_string(), correct_leftwards);

    let mut downwards_game = Game::new(3, 3, &[(2, 1), (1, 1), (0, 1)]);
    assert_eq!(downwards_game.render_to_string(), correct_downwards);
}

#[test]
fn test_drawing_turning_snakes() {
    #[rustfmt::skip]
    let correct_up_rightwards = concat!(" ╔>\n",
                                              " ║ \n",
                                              " ║ \n");
    let up_rightwards_body = vec![(0, 2), (0, 1), (1, 1), (2, 1)];
    let mut up_rightwards_game = Game::new(3, 3, &up_rightwards_body);
    assert_eq!(up_rightwards_game.render_to_string(), correct_up_rightwards);
    #[rustfmt::skip]
    let correct_up_leftwards = concat!("<╗ \n",
                                             " ║ \n",
                                             " ║ \n");
    let up_leftwards_body = vec![(0, 0), (0, 1), (1, 1), (2, 1)];
    let mut up_leftwards_game = Game::new(3, 3, &up_leftwards_body);
    assert_eq!(up_leftwards_game.render_to_string(), correct_up_leftwards);
    #[rustfmt::skip]
    let correct_right_upwards = concat!("  ^\n",
                                              "══╝\n",
                                              "   \n");
    let right_upwards_body = vec![(0, 2), (1, 2), (1, 1), (1, 0)];
    let mut right_upwards_game = Game::new(3, 3, &right_upwards_body);
    assert_eq!(right_upwards_game.render_to_string(), correct_right_upwards);
    #[rustfmt::skip]
    let correct_right_downwards = concat!("   \n",
                                                "══╗\n",
                                                "  v\n");
    let right_downwards_body = vec![(2, 2), (1, 2), (1, 1), (1, 0)];
    let mut right_downwards_game = Game::new(3, 3, &right_downwards_body);
    assert_eq!(
        right_downwards_game.render_to_string(),
        correct_right_downwards
    );
    #[rustfmt::skip]
    let correct_left_upwards = concat!("^  \n",
                                             "╚══\n",
                                             "   \n");
    let left_upwards_body = vec![(0, 0), (1, 0), (1, 1), (1, 2)];
    let mut left_upwards_game = Game::new(3, 3, &left_upwards_body);
    assert_eq!(left_upwards_game.render_to_string(), correct_left_upwards);
    #[rustfmt::skip]
    let correct_left_downwards = concat!("   \n",
                                               "╔══\n",
                                               "v  \n");
    let left_downwards_body = vec![(2, 0), (1, 0), (1, 1), (1, 2)];
    let mut left_downwards_game = Game::new(3, 3, &left_downwards_body);
    assert_eq!(
        left_downwards_game.render_to_string(),
        correct_left_downwards
    );
    #[rustfmt::skip]
    let correct_down_leftwards = concat!(" ║ \n",
                                               " ║ \n",
                                               "<╝ \n");
    let down_leftwards_body = vec![(2, 0), (2, 1), (1, 1), (0, 1)];
    let mut down_leftwards_game = Game::new(3, 3, &down_leftwards_body);
    assert_eq!(
        down_leftwards_game.render_to_string(),
        correct_down_leftwards
    );
    #[rustfmt::skip]
    let correct_down_rightwards = concat!(" ║ \n",
                                                " ║ \n",
                                                " ╚>\n");
    let down_rightwards_body = vec![(2, 2), (2, 1), (1, 1), (0, 1)];
    let mut down_rightwards_game = Game::new(3, 3, &down_rightwards_body);
    assert_eq!(
        down_rightwards_game.render_to_string(),
        correct_down_rightwards
    );
}

use crate::pretty_rendering::debug_mesh::DebugMesh;
use ggez::conf::WindowMode;
use ggez::event;
use ggez::event::quit;
use ggez::graphics;
use ggez::graphics::{DrawParam, Drawable};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::nalgebra as na;
use std::time::Instant;

struct MainState {
    window_size: (f32, f32),
    game: Game,
}

impl MainState {
    fn new(window_size: (f32, f32)) -> ggez::GameResult<MainState> {
        let s = MainState {
            window_size,
            game: Game {
                snake: Snake {
                    direction: Direction::Left,
                    lengthening: false,
                    body: VecDeque::from(vec![(10, 10), (10, 11), (10, 12), (10, 13), (10, 14)]),
                    confines: (20, 20),
                    confines_size: (window_size.0 - 60.0, window_size.1 - 60.0),
                },
                apples: HashSet::new(),
                width: 20,
                height: 20,
                rendered: vec![],
                last_advance: Instant::now(),
            },
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

const DEBUG: bool = true;

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        if self.game.last_advance.elapsed().as_secs_f32() >= 2.0 {
            self.game.advance();
            self.game.last_advance = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let play_area = na::Point2::new(30.0, 30.0);
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        self.game
            .snake
            .draw(ctx, DrawParam::default().dest(play_area));
        self.draw_border(ctx)?;
        if DEBUG {
            let mesh = DebugMesh {
                rows: self.game.snake.confines.0,
                columns: self.game.snake.confines.1,
                container: graphics::Rect::new(30.0, 30.0, 540.0, 540.0),
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
                let new_dir = Game::get_snake_direction_from_keypress(keycode).unwrap();
                self.game.snake.direction = new_dir;
            }
            KeyCode::Escape => {
                quit(ctx);
            }
            _ => (),
        }
    }
}

pub fn main() -> ggez::GameResult {
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
