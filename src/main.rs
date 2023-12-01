mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: {% day %}");
    println!("  part 1: {}", part_1(get_input(Input::Real)));
    println!("  part 2: {}", part_2(get_input(Input::Real)));
}

fn part_1(lines: Vec<String>) -> usize {
    1
}

fn part_2(lines: Vec<String>) -> usize {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_input(Input::Test1)), 1);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_input(Input::Test1)), 2);
    }
}
