//! <https://adventofcode.com/2015/day/3>

use aoc_macros::aoc;

const INPUT: &str = include_str!("../../data/2015/04.txt");

fn main() {
    let input = INPUT.trim();
    part_1(input);
    part_2(input);
}

#[aoc("[2015/04/01]")]
fn part_1(input: &str) -> usize {
    brute_force(input, "00000")
}

#[aoc("[2015/04/02]")]
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
