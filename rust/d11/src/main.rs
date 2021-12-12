#![feature(test)]

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    let mut state = parse_input(reader);
    let mut total_flashes = 0;

    for _ in 0..100 {
        total_flashes += state.tick();
    }

    total_flashes.to_string()
}

fn part2<R: BufRead>(reader: R) -> String {
    let mut state = parse_input(reader);
    let mut i = 0;
    while !state.all_zero() {
        state.tick();
        i += 1;
    }
    i.to_string()
}

fn parse_input<R: BufRead>(reader: R) -> State {
    State(reader
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.chars()
                .map(|c| { (c as u8) - ('0' as u8) })
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[u8; SIZE]>>()
        .try_into()
        .unwrap())
}

const SIZE: usize = 10;

struct State([[u8; SIZE]; SIZE]);

impl State {
    fn tick(&mut self) -> u32 {
        for i in 0..SIZE {
            for j in 0..SIZE {
                self.0[i][j] += 1;
            }
        }

        let mut flashes = 0u32;

        let mut flashing = true;

        while flashing {
            flashing = false;

            for i in 0..SIZE {
                for j in 0..SIZE {
                    if self.0[i][j] < 10 {
                        continue;
                    }

                    self.0[i][j] = 0;
                    flashes += 1;
                    flashing = true;

                    for i2 in if i > 0 { i - 1 } else { i }..if i < SIZE - 1 { i + 2 } else { i + 1 } {
                        for j2 in if j > 0 { j - 1 } else { j }..if j < SIZE - 1 { j + 2 } else { j + 1 } {
                            if (i, j) == (i2, j2) {
                                continue;
                            }

                            if self.0[i2][j2] == 0 {
                                continue;
                            }

                            self.0[i2][j2] += 1;
                        }
                    }
                }
            }
        }

        flashes
    }

    fn all_zero(&self) -> bool {
        (0..SIZE).all(|i| (0..SIZE).all(|j| { self.0[i][j] == 0 }))
    }
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
        assert_eq!(part1(BufReader::new(BASIC)), "1656")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "195")
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
