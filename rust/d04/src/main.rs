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
        for board in &mut boards {
            if board.mark(n) {
                return (board.sum_unmarked() * n).to_string();
            }
        }
    }

    panic!("no result");
}

fn part2<R: BufRead>(reader: R) -> String {
    let (mut boards, numbers) = parse_input(reader);

    let mut solved_boards = vec![false; boards.len()];
    let mut remainder = boards.len();

    for n in numbers {
        for i in 0..boards.len() {
            if solved_boards[i] {
                continue;
            }

            let board = &mut boards[i];
            let bingo = board.mark(n);

            if bingo {
                solved_boards[i] = true;
                remainder -= 1;

                if remainder == 0 {
                    return (board.sum_unmarked() * n).to_string();
                }
            }
        }
    }

    panic!("no result");
}

type Number = u32;

const SIZE: usize = 5;

const MARKED: Number = 0;

struct Board {
    numbers: [[Number; SIZE]; SIZE],
}

impl Board {
    fn mark(&mut self, n: Number) -> bool {
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.numbers[i][j] == n {
                    self.numbers[i][j] = MARKED;
                    return (0..SIZE).all(|i| { self.numbers[i][j] == MARKED }) ||
                        (0..SIZE).all(|j| { self.numbers[i][j] == MARKED });
                }
            }
        }

        return false;
    }

    fn sum_unmarked(&self) -> Number {
        (0..SIZE)
            .flat_map(|i| {
                (0..SIZE)
                    .filter(move |&j| { self.numbers[i][j] != MARKED })
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
        Ok(numbers) => Some(Board { numbers })
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
