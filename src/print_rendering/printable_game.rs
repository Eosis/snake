use crate::print_rendering::Printable;
use crate::Game;

impl Printable for Game {
    fn print(&mut self, rendered: &mut Vec<Vec<char>>) {
        self.render(rendered);
        println!("{}", "-".repeat(self.width + 2));
        for row in rendered.iter() {
            println!("|{}|", row.iter().collect::<String>());
        }
        println!("{}", "-".repeat(self.width + 2));
    }
}

impl Game {
    fn render(&mut self, rendered: &mut Vec<Vec<char>>) {
        self.clear(rendered);
        self.apples.print(rendered);
        self.snake.print(rendered);
    }

    fn clear(&mut self, rendered: &mut Vec<Vec<char>>) {
        for row in rendered.iter_mut() {
            for elem in row.iter_mut() {
                *elem = ' ';
            }
        }
    }

    #[cfg(test)]
    pub fn render_to_string(&mut self, rendered: &mut Vec<Vec<char>>) -> String {
        self.render(rendered);
        rendered
            .iter()
            .map(|row| row.iter().collect::<String>())
            .fold(String::new(), |mut init, add| {
                init.push_str(&add);
                init.push_str("\n");
                init
            })
    }
}
