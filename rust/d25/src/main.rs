#![feature(test)]

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", part1(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    let mut region = Region::from(reader);
    let mut step = 1;
    while let StepResult::Moved = region.step() {
        step += 1;
    }
    step.to_string()
}

fn read_input() -> BufReader<File> {
    BufReader::new(File::open("input.txt").unwrap())
}

#[derive(Clone, PartialEq)]
enum Cucumber {
    Right,
    Down,
    None,
}

impl From<char> for Cucumber {
    fn from(c: char) -> Self {
        match c {
            '.' => Cucumber::None,
            '>' => Cucumber::Right,
            'v' => Cucumber::Down,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq)]
enum StepResult {
    Moved,
    Stopped,
}

struct Region {
    cucumbers: Vec<Vec<Cucumber>>,
    buffer: Vec<Vec<Cucumber>>,
    shape: (usize, usize),
}

impl<R: BufRead> From<R> for Region {
    fn from(reader: R) -> Self {
        let cucumbers = reader.lines().map(Result::unwrap)
            .map(|line| line.chars().map(Cucumber::from).collect())
            .collect::<Vec<Vec<Cucumber>>>();
        let shape = (cucumbers.len(), cucumbers[0].len());
        let buffer = vec![];
        Region { cucumbers, buffer, shape }
    }
}

impl Region {
    fn step(&mut self) -> StepResult {
        let right_result = self.step_right();
        let down_result = self.step_down();
        if right_result == StepResult::Stopped && down_result == StepResult::Stopped {
            StepResult::Stopped
        } else {
            StepResult::Moved
        }
    }

    fn step_right(&mut self) -> StepResult {
        self.buffer.clone_from(&self.cucumbers);

        let mut result = StepResult::Stopped;

        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 - 1 {
                if self.buffer[i][j] == Cucumber::Right && self.buffer[i][j + 1] == Cucumber::None {
                    self.cucumbers[i][j] = Cucumber::None;
                    self.cucumbers[i][j + 1] = Cucumber::Right;
                    result = StepResult::Moved;
                }
            }

            if self.buffer[i][self.shape.1 - 1] == Cucumber::Right && self.buffer[i][0] == Cucumber::None {
                self.cucumbers[i][self.shape.1 - 1] = Cucumber::None;
                self.cucumbers[i][0] = Cucumber::Right;
                result = StepResult::Moved;
            }
        }

        result
    }

    fn step_down(&mut self) -> StepResult {
        self.buffer.clone_from(&self.cucumbers);

        let mut result = StepResult::Stopped;

        for i in 0..self.shape.0 - 1 {
            for j in 0..self.shape.1 {
                if self.buffer[i][j] == Cucumber::Down && self.buffer[i + 1][j] == Cucumber::None {
                    self.cucumbers[i][j] = Cucumber::None;
                    self.cucumbers[i + 1][j] = Cucumber::Down;
                    result = StepResult::Moved;
                }
            }
        }

        for j in 0..self.shape.1 {
            if self.buffer[self.shape.0 - 1][j] == Cucumber::Down && self.buffer[0][j] == Cucumber::None {
                self.cucumbers[self.shape.0 - 1][j] = Cucumber::None;
                self.cucumbers[0][j] = Cucumber::Down;
                result = StepResult::Moved;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use std::fs;
    use test::Bencher;

    use super::*;

    const BASIC: &[u8] = include_str!("testdata/basic.txt").as_bytes();

    #[test]
    fn test_part1() {
        assert_eq!(part1(BufReader::new(BASIC)), "58")
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = fs::read_to_string("input.txt").unwrap();
        let input = input.as_bytes();
        b.iter(|| part1(BufReader::new(input)))
    }
}
