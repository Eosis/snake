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

/// The entire Game's state
struct Game {
    pub over: bool,
    pub snake: Snake,
    pub apples: HashSet<Apple>,
    pub width: usize,
    pub score: u32,
    #[allow(dead_code)]
    height: usize,
    /// This flag is set when we have changed our direction once in this tick. Further changes will
    /// be ignored. This prevents a bug where two quick key presses would cause the snake to "run into
    /// itself".
    direction_changed_this_tick: bool,
}

/// This struct will be used as an iterator over all the remaining available spaces in the game.
#[derive(Clone)]
struct AvailableSpaces<'a> {
    /// Width to determine the correct space to check ( offset / width, offset % width )
    width: usize,
    /// Height to determine the full size of the board (width * height)
    height: usize,
    /// A VecDeque containing all the current occupied positions on the board.
    occupied: &'a VecDeque<(i32, i32)>,
    /// The current position of our iterator, which represents the current available space being returned.
    offset: usize,
}

impl<'a> AvailableSpaces<'a> {
    fn new(width: usize, height: usize, occupied: &'a VecDeque<(i32, i32)>) -> Self {
        Self {
            width,
            height,
            occupied,
            offset: 0,
        }
    }
}

impl<'a> Iterator for AvailableSpaces<'a> {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<(i32, i32)> {
        while self.offset != self.width * self.height {
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
            direction_changed_this_tick: false,
        }
    }

    /// Advance the game state, usually called by the Game Engine main loop (every tick).
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
        self.direction_changed_this_tick = false;
    }

    /// Add a new apple to the Game when none remain on the board.
    fn add_new_apple(&mut self) {
        // TODO: Improve efficiency of this function (rand's choose is O(N)). I think it could be improved to O(1).
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_spaces() {
        // Checking for the 3 x 3 grid (# = occupied)
        // + - - - +
        // | . . . |
        // | # # # |
        // | . . . |
        // + - - - +
        let occupied_spaces = vec![(1, 0), (1, 1), (1, 2)].into();
        let mut available_spaces = AvailableSpaces::new(3, 3, &occupied_spaces);
        assert_eq!(available_spaces.clone().count(), 6);
        assert_eq!(available_spaces.next(), Some((0, 0)));
        assert_eq!(available_spaces.next(), Some((0, 1)));
        assert_eq!(available_spaces.next(), Some((0, 2)));
        assert_eq!(available_spaces.next(), Some((2, 0)));
        assert_eq!(available_spaces.next(), Some((2, 1)));
        assert_eq!(available_spaces.next(), Some((2, 2)));
        assert_eq!(available_spaces.next(), None);
    }
}
