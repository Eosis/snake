pub mod apple;
pub mod pretty_rendering;
pub mod print_rendering;
pub mod snake;
use crate::apple::Apple;
use crate::pretty_rendering::ggez_main;
use crate::print_rendering::stringy_main;
use crate::snake::Snake;
use rand::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::env;

struct Game {
    pub over: bool,
    pub snake: Snake,
    pub apples: HashSet<Apple>,
    pub width: usize,
    pub score: u32,
    #[allow(dead_code)]
    height: usize,
}

struct AvailableSpaces<'a> {
    width: usize,
    occupied: &'a VecDeque<(i32, i32)>,
    offset: usize,
    available: usize,
}

impl<'a> AvailableSpaces<'a> {
    fn new(width: usize, height: usize, occupied: &'a VecDeque<(i32, i32)>) -> Self {
        Self {
            width,
            occupied,
            offset: 0,
            available: width * height - occupied.len(),
        }
    }
}

impl<'a> Iterator for AvailableSpaces<'a> {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<(i32, i32)> {
        while self.offset != self.available {
            let y: i32 = (self.offset / self.width) as i32;
            let x: i32 = (self.offset % self.width) as i32;
            self.offset += 1;
            if !self.occupied.contains(&(y, x)) {
                return Some((y, x));
            }
        }
        None
    }
}

impl Game {
    pub fn new(width: usize, height: usize, snake_body: &[(i32, i32)]) -> Game {
        Game {
            over: false,
            snake: Snake {
                body: VecDeque::from(Vec::from(snake_body)),
                lengthening: false,
                direction: Snake::head_direction(snake_body.iter().take(2)),
                confines: (height as i32, width as i32),
                confines_size: (550.0, 550.0),
            },
            score: 0,
            apples: HashSet::new(),
            width,
            height,
        }
    }

    pub fn advance(&mut self) {
        self.snake.advance();
        let head = self.snake.body[0];
        if self.apples.contains(&Apple { location: head }) {
            self.score += 1;
            self.snake.lengthening = true;
            self.apples.remove(&Apple { location: head });
        }
        if self.apples.is_empty() {
            self.add_new_apple();
        }
    }

    fn add_new_apple(&mut self) {
        let spaces = AvailableSpaces::new(self.width, self.height, &self.snake.body);
        let mut rng = rand::thread_rng();
        let location = spaces.choose(&mut rng).unwrap();
        let location = (location.0 as i32, location.1 as i32);
        self.apples.insert(Apple { location });
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
