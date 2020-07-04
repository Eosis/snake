pub mod pretty_rendering;
pub mod print_rendering;
pub mod snake;
use crate::pretty_rendering::ggez_main;
use crate::print_rendering::stringy_main;
use crate::snake::Snake;
use std::collections::{HashSet, VecDeque};
use std::env;

struct Game {
    pub over: bool,
    pub snake: Snake,
    pub apples: HashSet<Apple>,
    pub width: usize,
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
            over: false,
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
