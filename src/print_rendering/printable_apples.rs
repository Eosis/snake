use crate::print_rendering::Printable;
use crate::snake::Direction;
use crate::Apple;
use crate::Game;
use crate::Snake;
use std::collections::HashSet;

impl Printable for HashSet<Apple> {
    fn print(&mut self, rendered: &mut Vec<Vec<char>>) {
        for apple in self.iter() {
            let (y, x) = apple.location;
            rendered[y][x] = 'O';
        }
    }
}
