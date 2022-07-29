//! <https://adventofcode.com/2015/day/8>
#![feature(iter_advance_by)]

use std::time::Instant;

use aoc::input::by_line_vector;

const INPUT: &str = include_str!("../../data/2015/08.txt");

fn main() {
    let input = INPUT.trim();
    let input_vec: Vec<String> = by_line_vector(input);

    let part_1_timer = Instant::now();
    let part_1 = part_1(&input_vec);
    let part_1_elapsed = part_1_timer.elapsed();

    println!("[2015/08/01]");
    println!("Solution: {part_1:#?}");
    println!("Time: {part_1_elapsed:#?}");

    let part_2_timer = Instant::now();
    let part_2 = part_2(&input_vec);
    let part_2_elapsed = part_2_timer.elapsed();

    println!("[2015/08/02]");
    println!("Solution: {part_2:#?}");
    println!("Time: {part_2_elapsed:#?}");
}

pub fn part_1(input_vec: &[String]) -> usize {
    let mut characters = 0;
    let mut characters_memory = 0;

    for input in input_vec {
        characters += input.len();

        let mut chars = input.chars().peekable();
        while let Some(char) = chars.next() {
            if char == '"' {
                continue;
            }

            if char == '\\' && let Some(next) = chars.next() && next == 'x' {
                chars.advance_by(2).unwrap();
            }

            characters_memory += 1;
        }
    }

    characters - characters_memory
}

pub fn part_2(input_vec: &[String]) -> usize {
    let mut characters = 0;
    let mut characters_encoded = 0;

    for input in input_vec {
        characters += input.len();

        let mut string = String::new();
        for char in input.chars() {
            match char {
                '"' => string.push_str(r#"\""#),
                '\\' => string.push_str(r#"\\"#),
                _ => string.push(char),
            }
        }

        characters_encoded += 2 + string.len();
    }

    characters_encoded - characters
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case(r#""""#, 2)]
    #[case(r#""abc""#, 2)]
    #[case(r#""aaa\"aaa""#, 3)]
    #[case(r#""\x27""#, 5)]
    fn test_part_1(#[case] input: &str, #[case] expected: usize) {
        let item = input.to_string();
        assert_eq!(expected, part_1(&[item]));
    }

    #[test]
    fn test_part_1_solution() {
        let input = INPUT.trim();
        let input_vec: Vec<String> = by_line_vector(input);
        assert_eq!(1_371, part_1(&input_vec));
    }

    #[rstest]
    #[case(r#""""#, 4)]
    #[case(r#""abc""#, 4)]
    #[case(r#""aaa\"aaa""#, 6)]
    #[case(r#""\x27""#, 5)]
    fn test_part_2(#[case] input: &str, #[case] expected: usize) {
        let item = input.to_string();
        assert_eq!(expected, part_2(&[item]));
    }

    #[test]
    fn test_part_2_solution() {
        let input = INPUT.trim();
        let input_vec: Vec<String> = by_line_vector(input);
        assert_eq!(2_117, part_2(&input_vec));
    }
}
