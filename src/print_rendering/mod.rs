pub mod printable_apples;
pub mod printable_game;
pub mod printable_snake;

pub trait Printable {
    fn print(&mut self, rendered: &mut Vec<Vec<char>>);
}
