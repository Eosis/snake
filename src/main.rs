use std::collections::VecDeque;
use std::error::Error;
use std::io::stdin;

struct Game {
    pub snake: Snake,
    pub apples: Vec<Apple>,
    pub width: usize,
    #[allow(dead_code)]
    height: usize,
    rendered: Vec<Vec<char>>,
}

struct Snake {
    body: VecDeque<(usize, usize)>,
    confines: (usize, usize),
}

struct Apple {
    location: (usize, usize),
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        Game {
            snake: Snake {
                body: VecDeque::from(vec![(10, 10), (10, 9), (10, 8), (10, 7), (10, 6)]),
                confines: (height, width),
            },
            apples: vec![],
            rendered: vec![vec![' '; width]; height],
            width,
            height,
        }
    }

    pub fn advance(&mut self) {
        self.snake.advance();
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
        self.render_snake();
        self.render_apples();
    }

    fn clear(&mut self) {
        for row in self.rendered.iter_mut() {
            for elem in row.iter_mut() {
                *elem = ' ';
            }
        }
    }

    fn render_snake(&mut self) {
        let mut body_iter = self.snake.body.iter();
        let (y, x) = body_iter.next().unwrap();
        self.rendered[*y][*x] = 'X';
        for point in body_iter {
            let (y, x) = point;
            self.rendered[*y][*x] = '=';
        }
    }

    fn render_apples(&mut self) {
        for apple in &self.apples {
            let (y, x) = apple.location;
            self.rendered[y][x] = 'O';
        }
    }
}

impl Snake {
    #[cfg(test)]
    pub fn from_body(body: Vec<(usize, usize)>) -> Self {
        Snake {
            body: VecDeque::from(body),
            confines: (20, 20),
        }
    }

    pub fn advance(&mut self) {
        let direction = self.direction();
        let (dy, dx) = Snake::advancement_to_add(direction);
        let first = self.body.front().unwrap();
        let (y, x) = (first.0 as i32, first.1 as i32);
        let new = (y + dy, x + dx);
        if !self.dead_at(new) {
            let new = ((y + dy) as usize, (x + dx) as usize);
            self.body.push_front(new);
            self.body.pop_back();
        };
    }

    fn advancement_to_add(direction: Direction) -> (i32, i32) {
        match direction {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }

    fn dead_at(&self, (y, x): (i32, i32)) -> bool {
        y < 0 || x < 0 || y >= self.confines.0 as i32 || x >= self.confines.1 as i32
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Snake {
    fn direction(&self) -> Direction {
        let head = Self::point_helper(self.body[0]);
        let second = Self::point_helper(self.body[1]);
        match (head.0 - second.0, head.1 - second.1) {
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new(20, 20);
    game.apples = vec![Apple { location: (4, 4) }];

    loop_game(game);
    #[allow(unreachable_code)]
    Ok(())
}

fn loop_game(mut game: Game) -> ! {
    let mut line = String::new();
    loop {
        game.render_with_border();
        let _ = stdin().read_line(&mut line).unwrap();
        game.advance();
    }
}

#[cfg(test)]
fn count_board_squares(game: Game) -> usize {
    game.rendered.iter().fold(0, |tot, row| tot + row.len())
}

#[test]
fn test_creating_boards() {
    assert_eq!(count_board_squares(Game::new(20, 20)), 20 * 20);
    assert_eq!(count_board_squares(Game::new(50, 1)), 50);
}

#[test]
fn test_direction() {
    let upwards_body = vec![(9, 10), (10, 10)];
    let downwards_body = vec![(11, 10), (10, 10)];
    let rightwards_body = vec![(10, 11), (10, 10)];
    let leftwards_body = vec![(10, 9), (10, 10)];
    assert_eq!(Snake::from_body(upwards_body).direction(), Direction::Up);
    assert_eq!(
        Snake::from_body(downwards_body).direction(),
        Direction::Down
    );
    assert_eq!(
        Snake::from_body(rightwards_body).direction(),
        Direction::Right
    );
    assert_eq!(
        Snake::from_body(leftwards_body).direction(),
        Direction::Left
    );
}
