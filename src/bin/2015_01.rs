//! <https://adventofcode.com/2015/day/1>

use std::time::Instant;

const INPUT: &str = include_str!("../../data/2015/01.txt");

fn main() {
    let input = INPUT.trim();

    let part_1_timer = Instant::now();
    let part_1 = part_1(input);
    let part_1_elapsed = part_1_timer.elapsed();

    println!("[2015/01/01]");
    println!("Solution: {part_1:#?}");
    println!("Time: {part_1_elapsed:#?}");

    let part_2_timer = Instant::now();
    let part_2 = part_2(input);
    let part_2_elapsed = part_2_timer.elapsed();

    println!("[2015/01/02]");
    println!("Solution: {part_2:#?}");
    println!("Time: {part_2_elapsed:#?}");
}

fn part_1(input: &str) -> isize {
    input
        .chars()
        .fold(0, |sum, character| match character {
            '(' => sum + 1,
            ')' => sum - 1,
            _ => sum,
        })
}

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
    use rstest::*;

    use super::*;

    #[rstest]
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("(((", 3)]
    #[case("(()(()(", 3)]
    #[case("))(((((", 3)]
    #[case("())", -1)]
    #[case("))(", -1)]
    #[case(")))", -3)]
    #[case(")())())", -3)]
    fn test_part_1(#[case] input: &str, #[case] expected: isize) {
        assert_eq!(expected, part_1(input));
    }

    #[test]
    fn test_part_1_solution() {
        let input = INPUT.trim();
        assert_eq!(74, part_1(input));
    }

    #[rstest]
    #[case(")", 1)]
    #[case("()())", 5)]
    fn test_part_2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, part_2(input));
    }

    #[test]
    fn test_part_2_solution() {
        let input = INPUT.trim();
        assert_eq!(1_795, part_2(input));
    }
}
