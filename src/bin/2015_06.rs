//! <https://adventofcode.com/2015/day/6>

use std::convert::Infallible;
use std::str::FromStr;
use std::time::Instant;

use aoc::input::by_line_vector;
use ndarray::Array2;

const INPUT: &str = include_str!("../../data/2015/06.txt");

fn main() {
    let input = INPUT.trim();
    let input_vec: Vec<Coordinates> = by_line_vector(input);

    let part_1_timer = Instant::now();
    let part_1 = part_1(&input_vec);
    let part_1_elapsed = part_1_timer.elapsed();

    println!("[2015/06/01]");
    println!("Solution: {part_1:#?}");
    println!("Time: {part_1_elapsed:#?}");

    let part_2_timer = Instant::now();
    let part_2 = part_2(&input_vec);
    let part_2_elapsed = part_2_timer.elapsed();

    println!("[2015/06/02]");
    println!("Solution: {part_2:#?}");
    println!("Time: {part_2_elapsed:#?}");
}

pub fn part_1(input_vec: &[Coordinates]) -> usize {
    let mut array: Array2<usize> = Array2::zeros((1_000, 1_000));

    for coordinates in input_vec {
        match &coordinates.instruction {
            Instruction::TurnOn => {
                for (x, y) in coordinates.all_coordinates() {
                    array[[x, y]] = 1;
                }
            }
            Instruction::TurnOff => {
                for (x, y) in coordinates.all_coordinates() {
                    array[[x, y]] = 0;
                }
            }
            Instruction::Toggle => {
                for (x, y) in coordinates.all_coordinates() {
                    match array[[x, y]] {
                        0 => array[[x, y]] = 1,
                        1 => array[[x, y]] = 0,
                        _ => unreachable!(),
                    };
                }
            }
        }
    }

    array.sum()
}

pub fn part_2(input_vec: &[Coordinates]) -> usize {
    let mut array: Array2<usize> = Array2::zeros((1_000, 1_000));

    for coordinates in input_vec {
        match &coordinates.instruction {
            Instruction::TurnOn => {
                for (x, y) in coordinates.all_coordinates() {
                    array[[x, y]] += 1;
                }
            }
            Instruction::TurnOff => {
                for (x, y) in coordinates.all_coordinates() {
                    if array[[x, y]] == 0 {
                        continue;
                    }

                    array[[x, y]] -= 1;
                }
            }
            Instruction::Toggle => {
                for (x, y) in coordinates.all_coordinates() {
                    array[[x, y]] += 2;
                }
            }
        }
    }

    array.sum()
}

#[derive(Debug)]
pub enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

const TURN_ON: &str = "turn on ";
const TURN_OFF: &str = "turn off ";
const TOGGLE: &str = "toggle ";

#[derive(Debug)]
pub struct Coordinates {
    instruction: Instruction,
    start: (usize, usize),
    end: (usize, usize),
}

impl FromStr for Coordinates {
    type Err = Infallible;

    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        let is_turn_on = input_str.contains(TURN_ON);
        let is_turn_off = input_str.contains(TURN_OFF);
        let is_toggle = input_str.contains(TOGGLE);

        let (instruction, coordinate_string): (Instruction, String) = match (is_turn_on, is_turn_off, is_toggle) {
            (true, _, _) => (Instruction::TurnOn, input_str.replace(TURN_ON, "")),
            (_, true, _) => (Instruction::TurnOff, input_str.replace(TURN_OFF, "")),
            (_, _, true) => (Instruction::Toggle, input_str.replace(TOGGLE, "")),
            (_, _, _) => {
                unreachable!()
            }
        };

        let coordinate_parts: Vec<&str> = coordinate_string
            .splitn(2, " through ")
            .collect();

        let first_coordinate: Vec<usize> = coordinate_parts[0]
            .splitn(2, ',')
            .map(|string| string.parse().unwrap())
            .collect();

        let second_coordinate: Vec<usize> = coordinate_parts[1]
            .splitn(2, ',')
            .map(|string| string.parse().unwrap())
            .collect();

        Ok(Self {
            instruction,
            start: (first_coordinate[0], first_coordinate[1]),
            end: (second_coordinate[0], second_coordinate[1]),
        })
    }
}

impl Coordinates {
    pub fn all_coordinates(&self) -> Vec<(usize, usize)> {
        let mut coordinates = vec![];

        for x in self.start.0 ..= self.end.0 {
            for y in self.start.1 ..= self.end.1 {
                coordinates.push((x, y));
            }
        }

        coordinates
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case("turn on 0,0 through 999,999", 1_000_000)]
    #[case("toggle 0,0 through 999,0", 1_000)]
    #[case("turn off 499,499 through 500,500", 0)]
    fn test_part_1(#[case] input: &str, #[case] expected: usize) {
        let item = Coordinates::from_str(input).unwrap();
        assert_eq!(expected, part_1(&[item]));
    }

    #[test]
    fn test_part_1_solution() {
        let input = INPUT.trim();
        let input_vec: Vec<Coordinates> = by_line_vector(input);

        assert_eq!(543_903, part_1(&input_vec));
    }

    #[rstest]
    #[case("turn on 0,0 through 0,0", 1)]
    #[case("toggle 0,0 through 999,999", 2_000_000)]
    fn test_part_2(#[case] input: &str, #[case] expected: usize) {
        let item = Coordinates::from_str(input).unwrap();
        assert_eq!(expected, part_2(&[item]));
    }

    #[test]
    fn test_part_2_solution() {
        let input = INPUT.trim();
        let input_vec: Vec<Coordinates> = by_line_vector(input);

        assert_eq!(14_687_245, part_2(&input_vec));
    }
}
