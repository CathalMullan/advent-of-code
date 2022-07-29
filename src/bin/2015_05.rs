//! <https://adventofcode.com/2015/day/5>
#![feature(array_windows)]

use std::time::Instant;

use aoc::input::by_line_vector;

const INPUT: &str = include_str!("../../data/2015/05.txt");

fn main() {
    let input = INPUT.trim();
    let input_vec: Vec<String> = by_line_vector(input);

    let part_1_timer = Instant::now();
    let part_1 = part_1(&input_vec);
    let part_1_elapsed = part_1_timer.elapsed();

    println!("[2015/05/01]");
    println!("Solution: {part_1:#?}");
    println!("Time: {part_1_elapsed:#?}");

    let part_2_timer = Instant::now();
    let part_2 = part_2(&input_vec);
    let part_2_elapsed = part_2_timer.elapsed();

    println!("[2015/05/02]");
    println!("Solution: {part_2:#?}");
    println!("Time: {part_2_elapsed:#?}");
}

fn part_1(input_vec: &[String]) -> usize {
    input_vec
        .iter()
        .filter(|input| is_nice_string_part_1(input))
        .count()
}

fn part_2(input_vec: &[String]) -> usize {
    input_vec
        .iter()
        .filter(|input| is_nice_string_part_2(input))
        .count()
}

fn is_nice_string_part_1(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();

    let mut vowels = 0;
    for char in &chars {
        if ['a', 'e', 'i', 'o', 'u'].contains(char) {
            vowels += 1;
        }
    }

    let mut twice = false;
    for [first_char, second_char] in chars.array_windows() {
        if first_char == second_char {
            twice = true;
        }

        let joined = format!("{first_char}{second_char}");
        if ["ab", "cd", "pq", "xy"].contains(&joined.as_ref()) {
            return false;
        }
    }

    (vowels >= 3) && twice
}

fn is_nice_string_part_2(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();

    let mut pair = false;
    for [first_char, second_char] in chars.array_windows() {
        let joined = format!("{first_char}{second_char}");
        if input.matches(&joined).count() > 1 {
            pair = true;
        }
    }

    let mut repeat = false;
    for [front, _, end] in chars.array_windows() {
        if front == end {
            repeat = true;
            break;
        }
    }

    pair && repeat
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case("ugknbfddgicrmopn", 1)]
    #[case("aaa", 1)]
    #[case("jchzalrnumimnmhp", 0)]
    #[case("haegwjzuvuyypxyu", 0)]
    #[case("dvszwmarrgswjxmb", 0)]
    fn test_part_1(#[case] input: &str, #[case] expected: usize) {
        let item = input.to_string();
        assert_eq!(expected, part_1(&[item]));
    }

    #[test]
    fn test_part_1_solution() {
        let input = INPUT.trim();
        let input_vec: Vec<String> = by_line_vector(input);

        assert_eq!(238, part_1(&input_vec));
    }

    #[rstest]
    #[case("qjhvhtzxzqqjkmpb", 1)]
    #[case("xxyxx", 1)]
    #[case("uurcxstgmygtbstg", 0)]
    #[case("ieodomkazucvgmuy", 0)]
    #[case("aaa", 0)]
    fn test_part_2(#[case] input: &str, #[case] expected: usize) {
        let item = input.to_string();
        assert_eq!(expected, part_2(&[item]));
    }

    #[test]
    fn test_part_2_solution() {
        let input = INPUT.trim();
        let input_vec: Vec<String> = by_line_vector(input);

        assert_eq!(69, part_2(&input_vec));
    }
}
