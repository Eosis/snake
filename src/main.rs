use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::stdin;

struct Game {
    pub snake: Snake,
    pub apples: HashSet<Apple>,
    pub width: usize,
    #[allow(dead_code)]
    height: usize,
    rendered: Vec<Vec<char>>,
}

struct Snake {
    pub direction: Direction,
    pub lengthening: bool,
    body: VecDeque<(usize, usize)>,
    confines: (usize, usize),
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

impl Snake {
    #[cfg(test)]
    #[allow(dead_code)]
    pub fn from_body(body: &[(usize, usize)]) -> Self {
        Snake {
            body: VecDeque::from(Vec::from(body)),
            direction: Snake::head_direction(body.iter()),
            confines: (20, 20),
            lengthening: false,
        }
    }

    pub fn advance(&mut self) {
        let (dy, dx) = Snake::advancement_to_add(&self.direction);
        let first = self.body.front().unwrap();
        let (y, x) = (first.0 as i32, first.1 as i32);
        let new = (y + dy, x + dx);
        if !self.dead_at(new) {
            let new = ((y + dy) as usize, (x + dx) as usize);
            self.body.push_front(new);
            if self.lengthening {
                self.lengthening = false;
            } else {
                self.body.pop_back();
            }
        };
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
        y < 0 || x < 0 || y >= self.confines.0 as i32 || x >= self.confines.1 as i32
    }

    fn get_body_glyph_from_directions(to: Direction, from: Direction) -> char {
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
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Snake {
    fn head_direction<'a, T: Iterator<Item = &'a (usize, usize)>>(body_iter: T) -> Direction {
        let start_copy: Vec<_> = body_iter.take(2).collect();
        Snake::direction(*start_copy[0], *start_copy[1])
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
    game.apples = HashSet::new();
    game.apples.insert(Apple { location: (4, 4) });
    game.apples.insert(Apple { location: (9, 9) });
    game.apples.insert(Apple { location: (9, 10) });

    loop_game(game);
    #[allow(unreachable_code)]
    Ok(())
}

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
