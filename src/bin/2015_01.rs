//! <https://adventofcode.com/2015/day/1>

use aoc_macros::aoc;

const INPUT: &str = include_str!("../../data/2015/01.txt");

fn main() {
    let input = INPUT.trim();
    part_1(input);
    part_2(input);
}

#[aoc("[2015/01/01]")]
fn part_1(input: &str) -> isize {
    input
        .chars()
        .fold(0, |sum, character| match character {
            '(' => sum + 1,
            ')' => sum - 1,
            _ => sum,
        })
}

#[aoc("[2015/01/02]")]
fn part_2(input: &str) -> usize {
    let mut sum = 0;

    for (index, character) in input.chars().enumerate() {
        match character {
            '(' => sum += 1,
            ')' => sum -= 1,
            _ => unreachable!(),
        }

        if sum == -1 {
            return index + 1;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let cases: Vec<(&str, isize)> = vec![
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];

        for (input, expected) in cases {
            assert_eq!(expected, part_1(input));
        }
    }

    #[test]
    fn test_part_1_solution() {
        let input = INPUT.trim();
        assert_eq!(74, part_1(input));
    }

    #[test]
    fn test_part_2() {
        let cases: Vec<(&str, usize)> = vec![(")", 1), ("()())", 5)];

        for (input, expected) in cases {
            assert_eq!(expected, part_2(input));
        }
    }

    #[test]
    fn test_part_2_solution() {
        let input = INPUT.trim();
        assert_eq!(1_795, part_2(input));
    }
}
