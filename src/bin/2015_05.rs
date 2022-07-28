//! <https://adventofcode.com/2015/day/3>
#![feature(array_windows, array_chunks)]

use aoc::input::by_line_vector;
use aoc_macros::aoc;

const INPUT: &str = include_str!("../../data/2015/05.txt");

fn main() {
    let input = INPUT.trim();
    let input_vec: Vec<String> = by_line_vector(input);

    part_1(&input_vec);
    part_2(&input_vec);
}

#[aoc("[2015/05/01]")]
fn part_1(input_vec: &[String]) -> usize {
    input_vec
        .iter()
        .filter(|input| is_nice_string_part_1(input))
        .count()
}

#[aoc("[2015/05/02]")]
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
    use super::*;

    #[test]
    fn test_part_1() {
        let cases: Vec<(Vec<String>, usize)> = vec![
            (vec![String::from("ugknbfddgicrmopn")], 1),
            (vec![String::from("aaa")], 1),
            (vec![String::from("jchzalrnumimnmhp")], 0),
            (vec![String::from("haegwjzuvuyypxyu")], 0),
            (vec![String::from("dvszwmarrgswjxmb")], 0),
        ];

        for (input, expected) in cases {
            assert_eq!(expected, part_1(&input));
        }
    }

    #[test]
    fn test_part_1_solution() {
        let input = INPUT.trim();
        let input_vec: Vec<String> = by_line_vector(input);

        assert_eq!(238, part_1(&input_vec));
    }

    #[test]
    fn test_part_2() {
        let cases: Vec<(Vec<String>, usize)> = vec![
            (vec![String::from("qjhvhtzxzqqjkmpb")], 1),
            (vec![String::from("xxyxx")], 1),
            (vec![String::from("uurcxstgmygtbstg")], 0),
            (vec![String::from("ieodomkazucvgmuy")], 0),
            (vec![String::from("aaa")], 0),
        ];

        for (input, expected) in cases {
            assert_eq!(expected, part_2(&input));
        }
    }

    #[test]
    fn test_part_2_solution() {
        let input = INPUT.trim();
        let input_vec: Vec<String> = by_line_vector(input);

        assert_eq!(69, part_2(&input_vec));
    }
}
