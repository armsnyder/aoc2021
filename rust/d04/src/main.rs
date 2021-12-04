#![feature(test)]

use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    let (mut boards, numbers) = parse_input(reader);

    for n in numbers {
        for b in &mut boards {
            b.mark(n);
            if b.has_bingo() {
                return (b.sum_unmarked() * n).to_string();
            }
        }
    }

    panic!("no result");
}

fn part2<R: BufRead>(reader: R) -> String {
    let (mut boards, numbers) = parse_input(reader);

    for n in numbers {
        let final_board = boards.len() == 1;

        for b in &mut boards {
            b.mark(n);
            if final_board && b.has_bingo() {
                return (b.sum_unmarked() * n).to_string();
            }
        }

        boards = boards
            .into_iter()
            .filter(|b| { !b.has_bingo() })
            .collect();
    }

    panic!("no result");
}

const SIZE: usize = 5;

type Number = i32;

#[derive(PartialEq, Eq, Hash)]
struct Board {
    numbers: [[Number; SIZE]; SIZE],
    marked: [[bool; SIZE]; SIZE],
}

impl Board {
    fn mark(&mut self, n: Number) {
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.numbers[i][j] == n {
                    self.marked[i][j] = true;
                }
            }
        }
    }

    fn has_bingo(&self) -> bool {
        (0..SIZE).any(|i| {
            (0..SIZE).all(|j| { self.marked[i][j] }) ||
                (0..SIZE).all(|j| { self.marked[j][i] })
        })
    }

    fn sum_unmarked(&self) -> Number {
        (0..SIZE)
            .flat_map(|i| {
                (0..SIZE)
                    .filter(move |&j| { !self.marked[i][j] })
                    .map(move |j| { self.numbers[i][j] })
            })
            .sum()
    }
}

fn parse_input<R: BufRead>(reader: R) -> (Vec<Board>, Vec<Number>) {
    let mut lines = reader.lines();

    let numbers = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<Number>>();

    let _ = lines.next();

    let mut boards = Vec::new();

    while let Some(board) = next_board(&mut lines) {
        boards.push(board);
    }

    (boards, numbers)
}

fn next_board<T: Iterator<Item=Result<String, D>>, D: Debug>(it: &mut T) -> Option<Board> {
    match it
        .map(Result::unwrap)
        .take_while(|s| { !s.is_empty() })
        .map(parse_row)
        .collect::<Vec<[Number; SIZE]>>()
        .as_slice()
        .try_into() {
        Err(_) => None,
        Ok(numbers) => Some(Board { numbers, marked: [[false; SIZE]; SIZE] })
    }
}

fn parse_row(row: String) -> [Number; SIZE] {
    row
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<Number>>()
        .as_slice()
        .try_into()
        .unwrap()
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
        assert_eq!(part1(BufReader::new(BASIC)), "4512")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "1924")
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
