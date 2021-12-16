#![feature(test)]

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    let (mut paper, instructions) = parse_input(reader);
    paper.fold(instructions[0]);
    paper.count_dots().to_string()
}

fn part2<R: BufRead>(reader: R) -> String {
    let (mut paper, instructions) = parse_input(reader);
    for instruction in instructions {
        paper.fold(instruction);
    }
    return String::from(&paper);
}

#[derive(Default, Copy, Clone, PartialEq)]
struct Dot {
    x: u16,
    y: u16,
}

#[derive(Copy, Clone)]
enum Instruction {
    X(u16),
    Y(u16),
}

#[derive(Default)]
struct Paper(Vec<Dot>);

impl Paper {
    fn count_dots(&self) -> usize {
        self.0.len()
    }

    fn fold(&mut self, instruction: Instruction) {
        for i in 0..self.0.len() {
            self.0[i] = Paper::fold_dot(self.0[i], instruction);
        }
        self.remove_duplicates();
    }

    fn fold_dot(dot: Dot, instruction: Instruction) -> Dot {
        match instruction {
            Instruction::X(fold_x) => Dot {
                x: match dot.x.cmp(&fold_x) {
                    std::cmp::Ordering::Greater => fold_x - (dot.x - fold_x),
                    _ => dot.x,
                },
                y: dot.y,
            },
            Instruction::Y(fold_y) => Dot {
                x: dot.x,
                y: match dot.y.cmp(&fold_y) {
                    std::cmp::Ordering::Greater => fold_y - (dot.y - fold_y),
                    _ => dot.y,
                },
            },
        }
    }

    fn remove_duplicates(&mut self) {
        self.0.sort_by(|a, b| {
            match a.x.cmp(&b.x) {
                std::cmp::Ordering::Equal => a.y.cmp(&b.y),
                ord => ord,
            }
        });

        let mut next_dots = Vec::with_capacity(self.0.len());

        next_dots.push(self.0[0]);

        for i in 1..self.0.len() {
            if self.0[i] != self.0[i - 1] {
                next_dots.push(self.0[i]);
            }
        }

        self.0 = next_dots;
    }

    fn size(&self) -> Dot {
        Dot {
            x: self.0.iter().map(|dot| { dot.x }).max().unwrap() + 1,
            y: self.0.iter().map(|dot| { dot.y }).max().unwrap() + 1,
        }
    }
}

impl From<&Paper> for String {
    fn from(paper: &Paper) -> Self {
        let size = paper.size();
        let mut output = vec![' ' as u8; ((size.x + 1) * size.y) as usize];
        for i in 1..size.y + 1 {
            output[((size.x + 1) * i - 1) as usize] = '\n' as u8;
        }
        for dot in &paper.0 {
            output[(dot.y * (size.x + 1) + dot.x) as usize] = '#' as u8;
        }
        String::from_utf8(output).unwrap()
    }
}

fn parse_input<R: BufRead>(reader: R) -> (Paper, Vec<Instruction>) {
    let mut paper = Paper::default();
    let mut instructions = Vec::new();

    let mut paper_done = false;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.len() == 0 {
            paper_done = true;
            continue;
        }

        if !paper_done {
            let mut split = line.split(",");
            let mut dot = Dot::default();
            dot.x = split.next().unwrap().parse().unwrap();
            dot.y = split.next().unwrap().parse().unwrap();
            paper.0.push(dot);
        } else {
            let mut split = line.strip_prefix("fold along ").unwrap().split("=");
            let axis = split.next().unwrap();
            let value = split.next().unwrap().parse().unwrap();
            instructions.push(match axis {
                "x" => Instruction::X(value),
                "y" => Instruction::Y(value),
                _ => panic!("illegal axis"),
            })
        }
    }

    (paper, instructions)
}

fn read_input() -> BufReader<File> {
    BufReader::new(File::open("input.txt").unwrap())
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
        assert_eq!(part1(BufReader::new(BASIC)), "17")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "#####
#   #
#   #
#   #
#####
")
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = fs::read_to_string("input.txt").unwrap();
        let input = input.as_bytes();
        b.iter(|| part1(BufReader::new(input)))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = fs::read_to_string("input.txt").unwrap();
        let input = input.as_bytes();
        b.iter(|| part2(BufReader::new(input)))
    }
}
