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
    pub fn new(width: usize, height: usize, snake_body: &[(i32, i32)]) -> Game {
        Game {
            snake: Snake {
                body: VecDeque::from(
                    Vec::from(snake_body)
                        .iter()
                        .map(|(y, x)| (*y as usize, *x as usize))
                        .collect::<Vec<_>>(),
                ),
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
        self.render_snake_head();
        self.render_snake_body();
        self.render_snake_tail();
    }

    fn render_snake_head(&mut self) {
        let direction = self.snake.head_direction();
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
            Direction::Right | Direction::Left => '=',
        };
        self.rendered[*y][*x] = glyph;
    }

    fn render_apples(&mut self) {
        for apple in &self.apples {
            let (y, x) = apple.location;
            self.rendered[y][x] = 'O';
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

impl Snake {
    #[cfg(test)]
    pub fn from_body(body: &[(usize, usize)]) -> Self {
        Snake {
            body: VecDeque::from(Vec::from(body)),
            confines: (20, 20),
        }
    }

    pub fn advance(&mut self) {
        let direction = self.head_direction();
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

    fn get_body_glyph_from_directions(to: Direction, from: Direction) -> char {
        match (to, from) {
            (Direction::Up, Direction::Up) => '║',
            (Direction::Up, Direction::Right) => '╝',
            (Direction::Up, Direction::Down) => panic!("Not possible"),
            (Direction::Up, Direction::Left) => '╚',
            (Direction::Right, Direction::Up) => '╔',
            (Direction::Right, Direction::Right) => '=',
            (Direction::Right, Direction::Down) => '╚',
            (Direction::Right, Direction::Left) => panic!("Not possible"),
            (Direction::Down, Direction::Up) => panic!("Not possible"),
            (Direction::Down, Direction::Right) => '╗',
            (Direction::Down, Direction::Down) => '║',
            (Direction::Down, Direction::Left) => '╔',
            (Direction::Left, Direction::Up) => '╗',
            (Direction::Left, Direction::Right) => panic!("Not possible"),
            (Direction::Left, Direction::Down) => '╝',
            (Direction::Left, Direction::Left) => '=',
        }
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
    fn head_direction(&self) -> Direction {
        Snake::direction(self.body[0], self.body[1])
    }

    fn direction((now_y, now_x): (usize, usize), (then_y, then_x): (usize, usize)) -> Direction {
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new(20, 20, &[(10, 10), (10, 9), (10, 8), (10, 7), (10, 6)]);
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
    assert_eq!(
        Snake::from_body(&upwards_body).head_direction(),
        Direction::Up
    );
    assert_eq!(
        Snake::from_body(&downwards_body).head_direction(),
        Direction::Down
    );
    assert_eq!(
        Snake::from_body(&rightwards_body).head_direction(),
        Direction::Right
    );
    assert_eq!(
        Snake::from_body(&leftwards_body).head_direction(),
        Direction::Left
    );
}

#[test]
fn test_drawing_correct_snakes() {
    let correct_upwards = concat!(" ^ \n", " ║ \n", " ║ \n");
    let correct_rightwards = concat!("   \n", "==>\n", "   \n");
    let correct_leftwards = concat!("   \n", "<==\n", "   \n");
    let correct_downwards = concat!(" ║ \n", " ║ \n", " v \n");
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
