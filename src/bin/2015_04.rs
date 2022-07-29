//! <https://adventofcode.com/2015/day/4>

use std::time::Instant;

const INPUT: &str = include_str!("../../data/2015/04.txt");

fn main() {
    let input = INPUT.trim();

    let part_1_timer = Instant::now();
    let part_1 = part_1(input);
    let part_1_elapsed = part_1_timer.elapsed();

    println!("[2015/04/01]");
    println!("Solution: {part_1:#?}");
    println!("Time: {part_1_elapsed:#?}");

    let part_2_timer = Instant::now();
    let part_2 = part_2(input);
    let part_2_elapsed = part_2_timer.elapsed();

    println!("[2015/04/02]");
    println!("Solution: {part_2:#?}");
    println!("Time: {part_2_elapsed:#?}");
}

fn part_1(input: &str) -> usize {
    brute_force(input, "00000")
}

fn part_2(input: &str) -> usize {
    brute_force(input, "000000")
}

fn brute_force(input: &str, prefix: &str) -> usize {
    let mut index: usize = 0;

    loop {
        let digest = md5::compute(format!("{input}{index}"));

        let hex = format!("{digest:X}");
        if hex.starts_with(prefix) {
            return index;
        }

        index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let cases: Vec<(&str, usize)> = vec![("abcdef", 609_043), ("pqrstuv", 1_048_970)];

        for (input, expected) in cases {
            assert_eq!(expected, part_1(input));
        }
    }

    #[test]
    fn test_part_1_solution() {
        let input = INPUT.trim();
        assert_eq!(346_386, part_1(input));
    }

    #[test]
    fn test_part_2_solution() {
        let input = INPUT.trim();
        assert_eq!(9_958_218, part_2(input));
    }
}
