pub mod printable_apples;
pub mod printable_game;
pub mod printable_snake;

use crate::{Apple, Game};
use std::collections::HashSet;
use std::io::stdin;

use crate::snake::Direction;

#[cfg(test)]
use crate::snake::Snake;

pub trait Printable {
    fn print(&mut self, rendered: &mut Vec<Vec<char>>);
}

pub fn stringy_main() -> Result<(), ()> {
    let mut game = Game::new(20, 20, &[(10, 10), (10, 9), (10, 8), (10, 7), (10, 6)]);
    game.apples = HashSet::new();
    game.apples.insert(Apple { location: (4, 4) });
    game.apples.insert(Apple { location: (9, 9) });
    game.apples.insert(Apple { location: (9, 10) });

    loop_game(game);
    #[allow(unreachable_code)]
    Ok(())
}

fn get_snake_direction_from_input(input: char) -> Option<Direction> {
    match input {
        'w' => Some(Direction::Up),
        'a' => Some(Direction::Left),
        's' => Some(Direction::Down),
        'd' => Some(Direction::Right),
        _ => None,
    }
}

fn loop_game(mut game: Game) -> ! {
    let mut line = String::new();
    let mut rendered = vec![vec![' '; game.width]; game.height];
    loop {
        game.print(&mut rendered);
        let _ = stdin().read_line(&mut line).unwrap();
        if !line.is_empty() {
            if let Some(direction) =
                get_snake_direction_from_input(line.chars().next().unwrap())
            {
                game.snake.direction = direction;
            }
            line.clear();
        }
        game.advance();
    }
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
    let (default_width, default_height): (usize, usize) = (3, 3);
    let mut render_to = vec![vec![' '; default_width]; default_height];
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
    let mut upwards_game = Game::new(default_width, default_height, &[(0, 1), (1, 1), (2, 1)]);
    assert_eq!(
        upwards_game.render_to_string(&mut render_to),
        correct_upwards
    );

    let mut correct_rightwards_game =
        Game::new(default_width, default_height, &[(1, 2), (1, 1), (1, 0)]);
    assert_eq!(
        correct_rightwards_game.render_to_string(&mut render_to),
        correct_rightwards
    );

    let mut leftwards_game = Game::new(default_width, default_height, &[(1, 0), (1, 1), (1, 2)]);
    assert_eq!(
        leftwards_game.render_to_string(&mut render_to),
        correct_leftwards
    );

    let mut downwards_game = Game::new(default_width, default_height, &[(2, 1), (1, 1), (0, 1)]);
    assert_eq!(
        downwards_game.render_to_string(&mut render_to),
        correct_downwards
    );
}

#[test]
fn test_drawing_turning_snakes() {
    let (default_width, default_height): (usize, usize) = (3, 3);
    let mut render_to = vec![vec![' '; default_width]; default_height];
    #[rustfmt::skip]
    let correct_up_rightwards = concat!(" ╔>\n",
                                              " ║ \n",
                                              " ║ \n");
    let up_rightwards_body = vec![(0, 2), (0, 1), (1, 1), (2, 1)];
    let mut up_rightwards_game = Game::new(default_width, default_height, &up_rightwards_body);
    assert_eq!(
        up_rightwards_game.render_to_string(&mut render_to),
        correct_up_rightwards
    );
    #[rustfmt::skip]
    let correct_up_leftwards = concat!("<╗ \n",
                                             " ║ \n",
                                             " ║ \n");
    let up_leftwards_body = vec![(0, 0), (0, 1), (1, 1), (2, 1)];
    let mut up_leftwards_game = Game::new(default_width, default_height, &up_leftwards_body);
    assert_eq!(
        up_leftwards_game.render_to_string(&mut render_to),
        correct_up_leftwards
    );
    #[rustfmt::skip]
    let correct_right_upwards = concat!("  ^\n",
                                              "══╝\n",
                                              "   \n");
    let right_upwards_body = vec![(0, 2), (1, 2), (1, 1), (1, 0)];
    let mut right_upwards_game = Game::new(default_width, default_height, &right_upwards_body);
    assert_eq!(
        right_upwards_game.render_to_string(&mut render_to),
        correct_right_upwards
    );
    #[rustfmt::skip]
    let correct_right_downwards = concat!("   \n",
                                                "══╗\n",
                                                "  v\n");
    let right_downwards_body = vec![(2, 2), (1, 2), (1, 1), (1, 0)];
    let mut right_downwards_game = Game::new(default_width, default_height, &right_downwards_body);
    assert_eq!(
        right_downwards_game.render_to_string(&mut render_to),
        correct_right_downwards
    );
    #[rustfmt::skip]
    let correct_left_upwards = concat!("^  \n",
                                             "╚══\n",
                                             "   \n");
    let left_upwards_body = vec![(0, 0), (1, 0), (1, 1), (1, 2)];
    let mut left_upwards_game = Game::new(default_width, default_height, &left_upwards_body);
    assert_eq!(
        left_upwards_game.render_to_string(&mut render_to),
        correct_left_upwards
    );
    #[rustfmt::skip]
    let correct_left_downwards = concat!("   \n",
                                               "╔══\n",
                                               "v  \n");
    let left_downwards_body = vec![(2, 0), (1, 0), (1, 1), (1, 2)];
    let mut left_downwards_game = Game::new(default_width, default_height, &left_downwards_body);
    assert_eq!(
        left_downwards_game.render_to_string(&mut render_to),
        correct_left_downwards
    );
    #[rustfmt::skip]
    let correct_down_leftwards = concat!(" ║ \n",
                                               " ║ \n",
                                               "<╝ \n");
    let down_leftwards_body = vec![(2, 0), (2, 1), (1, 1), (0, 1)];
    let mut down_leftwards_game = Game::new(default_width, default_height, &down_leftwards_body);
    assert_eq!(
        down_leftwards_game.render_to_string(&mut render_to),
        correct_down_leftwards
    );
    #[rustfmt::skip]
    let correct_down_rightwards = concat!(" ║ \n",
                                                " ║ \n",
                                                " ╚>\n");
    let down_rightwards_body = vec![(2, 2), (2, 1), (1, 1), (0, 1)];
    let mut down_rightwards_game = Game::new(default_width, default_height, &down_rightwards_body);
    assert_eq!(
        down_rightwards_game.render_to_string(&mut render_to),
        correct_down_rightwards
    );
}
