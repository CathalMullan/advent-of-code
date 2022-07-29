//! <https://adventofcode.com/2015/day/3>
#![feature(const_mut_refs)]

use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("../../data/2015/03.txt");

fn main() {
    let input = INPUT.trim();

    let part_1_timer = Instant::now();
    let part_1 = part_1(input);
    let part_1_elapsed = part_1_timer.elapsed();

    println!("[2015/03/01]");
    println!("Solution: {part_1:#?}");
    println!("Time: {part_1_elapsed:#?}");

    let part_2_timer = Instant::now();
    let part_2 = part_2(input);
    let part_2_elapsed = part_2_timer.elapsed();

    println!("[2015/03/02]");
    println!("Solution: {part_2:#?}");
    println!("Time: {part_2_elapsed:#?}");
}

fn part_1(input: &str) -> usize {
    let mut position: (isize, isize) = (0, 0);
    let mut matrix: HashMap<(isize, isize), usize> = HashMap::from([(position, 1)]);

    input.chars().for_each(|character| {
        perform_move(character, &mut position);
        *matrix.entry(position).or_insert(0) += 1;
    });

    matrix.len()
}

fn part_2(input: &str) -> usize {
    let mut santa_position: (isize, isize) = (0, 0);
    let mut santa_matrix: HashMap<(isize, isize), usize> = HashMap::from([(santa_position, 1)]);

    let mut robo_position: (isize, isize) = (0, 0);
    let mut robo_matrix: HashMap<(isize, isize), usize> = HashMap::from([(robo_position, 1)]);

    input
        .chars()
        .enumerate()
        .for_each(|(index, character)| {
            if index % 2 == 0 {
                perform_move(character, &mut santa_position);
                *santa_matrix
                    .entry(santa_position)
                    .or_insert(0) += 1;
            } else {
                perform_move(character, &mut robo_position);
                *robo_matrix
                    .entry(robo_position)
                    .or_insert(0) += 1;
            }
        });

    santa_matrix.extend(robo_matrix);
    santa_matrix.len()
}

const fn perform_move(character: char, position: &mut (isize, isize)) {
    match character {
        '^' => position.1 += 1,
        '>' => position.0 += 1,
        'v' => position.1 -= 1,
        '<' => position.0 -= 1,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let cases: Vec<(&str, usize)> = vec![(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)];

        for (input, expected) in cases {
            assert_eq!(expected, part_1(input));
        }
    }

    #[test]
    fn test_part_1_solution() {
        let input = INPUT.trim();
        assert_eq!(2_565, part_1(input));
    }

    #[test]
    fn test_part_2() {
        let cases: Vec<(&str, usize)> = vec![("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)];

        for (input, expected) in cases {
            assert_eq!(expected, part_2(input));
        }
    }

    #[test]
    fn test_part_2_solution() {
        let input = INPUT.trim();
        assert_eq!(2_639, part_2(input));
    }
}
