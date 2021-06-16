use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Snake {
    /// Current direction of the Snake.
    pub direction: Direction,
    /// If we have just swallowed an Apple or not.
    pub lengthening: bool,
    /// The locations of the Snake's Body
    pub body: VecDeque<(i32, i32)>,
    /// The size of the game area (height, width), used in determining if we have crashed into a wall.
    pub confines: (i32, i32),
    /// The size of the confines as required for the pretty rendering of the game. This should be moved into a
    /// wrapper type in the pretty rendering module rather than being in this struct (it is only used by this module).
    pub confines_size: (f32, f32),
}

impl Snake {
    #[allow(dead_code)]
    pub fn from_body(body: &[(i32, i32)]) -> Self {
        Snake {
            body: VecDeque::from(Vec::from(body)),
            direction: Snake::head_direction(body.iter()),
            confines: (20, 20),
            lengthening: false,
            confines_size: (550.0, 550.0), // Hard coded to match the walls we are drawing in pretty rendering mode.
        }
    }

    pub fn advance(&mut self) {
        let (dy, dx) = Snake::advancement_to_add(&self.direction);
        let (y, x) = self.body.front().unwrap();
        let new = ((*y + dy), (*x + dx));
        self.body.push_front(new);
        if self.lengthening {
            self.lengthening = false;
        } else {
            self.body.pop_back();
        }
    }

    pub fn dead(&self) -> bool {
        let (y, x) = self.body.front().unwrap();
        *y < 0
            || *x < 0
            || *y >= self.confines.0 as i32
            || *x >= self.confines.1 as i32
            || self
            .body
            .iter()
            .skip(1)
            .any(|pos| *pos == (*y, *x))
    }

    fn advancement_to_add(direction: &Direction) -> (i32, i32) {
        match direction {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }

    pub fn get_body_glyph_from_directions(to: Direction, from: Direction) -> char {
        match (to, from) {
            (Direction::Up, Direction::Up) => '║',
            (Direction::Up, Direction::Right) => '╝',
            (Direction::Up, Direction::Down) => panic!("Not possible"),
            (Direction::Up, Direction::Left) => '╚',
            (Direction::Right, Direction::Up) => '╔',
            (Direction::Right, Direction::Right) => '═',
            (Direction::Right, Direction::Down) => '╚',
            (Direction::Right, Direction::Left) => panic!("Not possible"),
            (Direction::Down, Direction::Up) => panic!("Not possible"),
            (Direction::Down, Direction::Right) => '╗',
            (Direction::Down, Direction::Down) => '║',
            (Direction::Down, Direction::Left) => '╔',
            (Direction::Left, Direction::Up) => '╗',
            (Direction::Left, Direction::Right) => panic!("Not possible"),
            (Direction::Left, Direction::Down) => '╝',
            (Direction::Left, Direction::Left) => '═',
        }
    }

    pub fn head_direction<'a, T: Iterator<Item = &'a (i32, i32)>>(body_iter: T) -> Direction {
        let start_copy: Vec<_> = body_iter.take(2).collect();
        Snake::direction(*start_copy[0], *start_copy[1])
    }

    pub fn direction((now_y, now_x): (i32, i32), (then_y, then_x): (i32, i32)) -> Direction {
        match (now_y - then_y, now_x - then_x) {
            (-1, 0) => Direction::Up,
            (1, 0) => Direction::Down,
            (0, 1) => Direction::Right,
            (0, -1) => Direction::Left,
            x => panic!("Invalid direction determined: {:?}", x),
        }
    }
}
