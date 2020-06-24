pub mod pretty_rendering;
pub mod snake;

use crate::snake::{Direction, Snake};
use std::collections::{HashSet, VecDeque};
use std::io::stdin;

struct Game {
    pub snake: Snake,
    pub apples: HashSet<Apple>,
    pub width: usize,
    #[allow(dead_code)]
    height: usize,
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
            },
            apples: HashSet::new(),
            rendered: vec![vec![' '; width]; height],
            width,
            height,
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

use ggez::event;
use ggez::graphics;
use ggez::graphics::{DrawParam, Drawable};
use ggez::nalgebra as na;

struct MainState {
    pos_x: f32,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        draw_border(ctx)?;
        let snake =
            Snake::from_body(&[(10, 10), (10, 9), (9, 9), (9, 8), (10, 8), (10, 7), (10, 6)]);
        let my_dest = na::Point2::new(0.0, 0.0);
        snake.draw(ctx, DrawParam::default().dest(my_dest))?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn draw_border(ctx: &mut ggez::Context) -> ggez::GameResult {
    let top = graphics::Mesh::new_line(
        ctx,
        &[
            na::Point2::new(0.0 + 25.0, 25.0),
            na::Point2::new(800.0 - 25.0, 25.0),
        ],
        5.0,
        graphics::WHITE,
    )?;
    let right = graphics::Mesh::new_line(
        ctx,
        &[
            na::Point2::new(800.0 - 25.0, 0.0 + 25.0),
            na::Point2::new(800.0 - 25.0, 600.0 - 25.0),
        ],
        5.0,
        graphics::WHITE,
    )?;
    let bottom = graphics::Mesh::new_line(
        ctx,
        &[
            na::Point2::new(0.0 + 25.0, 600.0 - 25.0),
            na::Point2::new(800.0 - 25.0, 600.0 - 25.0),
        ],
        5.0,
        graphics::WHITE,
    )?;
    let left = graphics::Mesh::new_line(
        ctx,
        &[
            na::Point2::new(0.0 + 25.0, 0.0 + 25.0 - 2.5),
            na::Point2::new(0.0 + 25.0, 600.0 - 25.0 + 2.5),
        ],
        5.0,
        graphics::WHITE,
    )?;
    graphics::draw(ctx, &top, (na::Point2::new(0.0, 0.0),))?;
    graphics::draw(ctx, &right, (na::Point2::new(0.0, 0.0),))?;
    graphics::draw(ctx, &bottom, (na::Point2::new(0.0, 0.0),))?;
    graphics::draw(ctx, &left, (na::Point2::new(0.0, 0.0),))?;
    Ok(())
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("snakin'", "Rups");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
