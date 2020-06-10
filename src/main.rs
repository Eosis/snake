use std::error::Error;
use std::io::stdin;

struct Game {
    pub snake: Snake,
    pub apples: Vec<Apple>,
    pub width: usize,
    height: usize,
    rendered: Vec<Vec<char>>,
}

struct Snake {
    head: (usize, usize),
    body: Vec<(usize, usize)>,
}

struct Apple {
    location: (usize, usize),
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        Game {
            snake: Snake {
                head: (10, 10),
                body: vec![(10, 10), (10, 9), (10, 8), (10, 7), (10, 6)],
            },
            apples: vec![],
            rendered: vec![vec![' '; width]; height],
            width,
            height,
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
        self.render_snake();
        self.render_apples();
    }

    fn render_snake(&mut self) {
        let (y, x) = self.snake.head;
        self.rendered[y][x] = 'X';
        for point in self.snake.body.iter().skip(1) {
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new(20, 20);
    game.apples = vec![Apple { location: (4, 4) }];
    game.render_with_border();

    loop_game(game);
    Ok(())
}

fn loop_game(mut game: Game) -> ! {
    let mut line = String::new();
    loop {
        game.render_with_border();
        let _ = stdin().read_line(&mut line).unwrap();
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
