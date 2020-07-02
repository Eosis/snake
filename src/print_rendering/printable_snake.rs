use crate::print_rendering::Printable;
use crate::snake::Direction;
use crate::Snake;

impl Printable for Snake {
    fn print(&mut self, rendered: &mut Vec<Vec<char>>) {
        self.render_snake_head(rendered);
        self.render_snake_body(rendered);
        self.render_snake_tail(rendered);
    }
}

impl Snake {
    fn render_snake_head(&mut self, rendered: &mut Vec<Vec<char>>) {
        let direction = Snake::head_direction(self.body.iter());
        let (y, x) = self.body[0];
        let glyph = match direction {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        };
        rendered[y][x] = glyph;
    }

    fn render_snake_body(&mut self, rendered: &mut Vec<Vec<char>>) {
        for window in Vec::from(self.body.clone()).windows(3) {
            let to = Snake::direction(window[0], window[1]);
            let from = Snake::direction(window[1], window[2]);
            let joining_glyph = Snake::get_body_glyph_from_directions(to, from);
            let (y_to_set, x_to_set) = window[1];
            rendered[y_to_set][x_to_set] = joining_glyph;
        }
    }

    fn render_snake_tail(&mut self, rendered: &mut Vec<Vec<char>>) {
        let relevant_points: Vec<_> = self.body.iter().rev().take(2).rev().cloned().collect();
        let direction = Snake::direction(relevant_points[0], relevant_points[1]);
        let (y, x) = self.body.back().unwrap();
        let glyph = match direction {
            Direction::Up | Direction::Down => '║',
            Direction::Right | Direction::Left => '═',
        };
        rendered[*y][*x] = glyph;
    }
}
