use std::collections::VecDeque;

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Snake {
    pub direction: Direction,
    pub lengthening: bool,
    pub body: VecDeque<(usize, usize)>,
    pub confines: (usize, usize),
    pub confines_size: (f32, f32),
}

impl Snake {
    pub fn intervals(&self) -> (f32, f32) {
        (
            self.confines_size.0 / self.confines.0 as f32,
            self.confines_size.1 / self.confines.1 as f32,
        )
    }

    #[allow(dead_code)]
    pub fn from_body(body: &[(usize, usize)]) -> Self {
        Snake {
            body: VecDeque::from(Vec::from(body)),
            direction: Snake::head_direction(body.iter()),
            confines: (20, 20),
            lengthening: false,
            confines_size: (550.0, 550.0),
        }
    }

    pub fn advance(&mut self) {
        let (dy, dx) = Snake::advancement_to_add(&self.direction);
        let first = self.body.front().unwrap();
        let (y, x) = (first.0 as i32, first.1 as i32);
        let new = ((y + dy) as usize, (x + dx) as usize);
        self.body.push_front(new);
        if self.lengthening {
            self.lengthening = false;
        } else {
            self.body.pop_back();
        }
    }

    pub fn dead(&self) -> bool {
        let (y, x) = self.body.front().unwrap();
        let (y, x) = (*y as i32, *x as i32);
        self.dead_at((y, x))
    }

    fn advancement_to_add(direction: &Direction) -> (i32, i32) {
        match direction {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }

    fn dead_at(&self, (y, x): (i32, i32)) -> bool {
        y < 0
            || x < 0
            || y >= self.confines.0 as i32
            || x >= self.confines.1 as i32
            || self
                .body
                .iter()
                .skip(1)
                .any(|pos| (pos.0 as i32, pos.1 as i32) == (y, x))
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

    pub fn head_direction<'a, T: Iterator<Item = &'a (usize, usize)>>(body_iter: T) -> Direction {
        let start_copy: Vec<_> = body_iter.take(2).collect();
        Snake::direction(*start_copy[0], *start_copy[1])
    }

    pub fn direction(
        (now_y, now_x): (usize, usize),
        (then_y, then_x): (usize, usize),
    ) -> Direction {
        let (now_y, now_x) = Self::point_helper((now_y, now_x));
        let (then_y, then_x) = Self::point_helper((then_y, then_x));
        match (now_y - then_y, now_x - then_x) {
            (-1, 0) => Direction::Up,
            (1, 0) => Direction::Down,
            (0, 1) => Direction::Right,
            (0, -1) => Direction::Left,
            x => panic!("Invalid direction determined: {:?}", x),
        }
    }

    fn point_helper(point: (usize, usize)) -> (i32, i32) {
        (point.0 as i32, point.1 as i32)
    }
}
