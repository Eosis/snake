pub mod pretty_rendering;
pub mod print_rendering;
pub mod snake;

use crate::pretty_rendering::ggez_main;
use crate::print_rendering::stringy_main;
use crate::snake::{Direction, Snake};
use std::collections::{HashSet, VecDeque};
use std::env;

use ggez::input::keyboard::KeyCode;
use std::time::Instant;

struct Game {
    pub snake: Snake,
    pub apples: HashSet<Apple>,
    pub width: usize,
    pub last_advance: Instant,
    #[allow(dead_code)]
    height: usize,
}

#[derive(PartialEq, Eq, Hash)]
pub struct Apple {
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
}

pub fn main() -> Result<(), ()> {
    if env::args().any(|x| x == "string") {
        stringy_main()
    } else {
        match ggez_main() {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
