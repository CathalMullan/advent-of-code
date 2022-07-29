//! <https://adventofcode.com/2015/day/2>

use std::convert::Infallible;
use std::str::FromStr;
use std::time::Instant;

use aoc::input::by_line_vector;
use aoc::sort::const_bubblesort;

const INPUT: &str = include_str!("../../data/2015/02.txt");

fn main() {
    let input = INPUT.trim();
    let input_vec: Vec<Box> = by_line_vector(input);

    let part_1_timer = Instant::now();
    let part_1 = part_1(&input_vec);
    let part_1_elapsed = part_1_timer.elapsed();

    println!("[2015/02/01]");
    println!("Solution: {part_1:#?}");
    println!("Time: {part_1_elapsed:#?}");

    let part_2_timer = Instant::now();
    let part_2 = part_2(&input_vec);
    let part_2_elapsed = part_2_timer.elapsed();

    println!("[2015/02/02]");
    println!("Solution: {part_2:#?}");
    println!("Time: {part_2_elapsed:#?}");
}

fn part_1(input_vec: &[Box]) -> usize {
    input_vec
        .iter()
        .fold(0, |sum, input| sum + input.total_wrapping_paper())
}

fn part_2(input_vec: &[Box]) -> usize {
    input_vec
        .iter()
        .fold(0, |sum, input| sum + input.total_feet_ribbon())
}

#[derive(Debug)]
struct Box {
    length: usize,
    width: usize,
    height: usize,
}

impl Box {
    pub const fn lw(&self) -> usize {
        self.length * self.width
    }

    pub const fn wh(&self) -> usize {
        self.width * self.height
    }

    pub const fn hl(&self) -> usize {
        self.height * self.length
    }

    pub const fn total_wrapping_paper(&self) -> usize {
        let sorted_sides = const_bubblesort([self.lw(), self.wh(), self.hl()]);
        (2 * self.lw()) + (2 * self.wh()) + (2 * self.hl()) + sorted_sides[0]
    }

    pub const fn total_feet_ribbon(&self) -> usize {
        let sorted_dimensions = const_bubblesort([self.length, self.width, self.height]);
        (2 * sorted_dimensions[0]) + (2 * sorted_dimensions[1]) + self.length * self.width * self.height
    }
}

impl FromStr for Box {
    type Err = Infallible;

    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        let input_chunks: Vec<&str> = input_str.splitn(3, 'x').collect();
        Ok(Self {
            length: input_chunks[0].parse().unwrap(),
            width: input_chunks[1].parse().unwrap(),
            height: input_chunks[2].parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let cases: Vec<(Vec<Box>, usize)> = vec![
            (
                vec![Box {
                    length: 2,
                    width: 3,
                    height: 4,
                }],
                58,
            ),
            (
                vec![Box {
                    length: 1,
                    width: 1,
                    height: 10,
                }],
                43,
            ),
        ];

        for (input_vec, expected) in cases {
            assert_eq!(expected, part_1(&input_vec));
        }
    }

    #[test]
    fn test_part_1_solution() {
        let input = INPUT.trim();
        let input_vec: Vec<Box> = by_line_vector(input);

        assert_eq!(1_606_483, part_1(&input_vec));
    }

    #[test]
    fn test_part_2() {
        let cases: Vec<(Vec<Box>, usize)> = vec![
            (
                vec![Box {
                    length: 2,
                    width: 3,
                    height: 4,
                }],
                34,
            ),
            (
                vec![Box {
                    length: 1,
                    width: 1,
                    height: 10,
                }],
                14,
            ),
        ];

        for (input_vec, expected) in cases {
            assert_eq!(expected, part_2(&input_vec));
        }
    }

    #[test]
    fn test_part_2_solution() {
        let input = INPUT.trim();
        let input_vec: Vec<Box> = by_line_vector(input);

        assert_eq!(3_842_356, part_2(&input_vec));
    }
}
