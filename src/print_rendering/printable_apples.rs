use crate::print_rendering::Printable;
use crate::Apple;
use std::collections::HashSet;

impl Printable for HashSet<Apple> {
    fn print(&mut self, rendered: &mut Vec<Vec<char>>) {
        for apple in self.iter() {
            let (y, x) = apple.location;
            rendered[y as usize][x as usize] = 'O';
        }
    }
}
