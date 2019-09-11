fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn part_1(lines: &Vec<String>) -> usize {
    1
}

fn part_2(lines: &Vec<String>) -> usize {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
    }

    #[test]
    fn test_part_2() {
    }
}
