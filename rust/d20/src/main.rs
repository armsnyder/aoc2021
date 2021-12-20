#![feature(test)]

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    solve(reader, 2)
}

fn part2<R: BufRead>(reader: R) -> String {
    solve(reader, 50)
}

fn solve<R: BufRead>(reader: R, n: usize) -> String {
    let (mut image, algorithm) = parse_input(reader);

    for _ in 0..n {
        image = image.enhance(&algorithm);
    }

    image.count_pixels().to_string()
}

#[derive(Hash, Eq, PartialEq)]
struct Index(i32, i32);

struct Image {
    chunk: HashSet<Index>,
    chunk_origin: Index,
    chunk_size: Index,
    border: bool,
}

impl Image {
    fn enhance(&self, algorithm: &Vec<bool>) -> Self {
        let mut output_chunk = HashSet::new();

        let min_i = self.chunk_origin.0;
        let max_i = self.chunk_origin.0 + self.chunk_size.0 - 1;
        let min_j = self.chunk_origin.1;
        let max_j = self.chunk_origin.1 + self.chunk_size.1 - 1;

        for i in min_i - 1..=max_i + 1 {
            for j in min_j - 1..=max_j + 1 {
                let mut lookup_key = 0usize;

                for i in i - 1..=i + 1 {
                    for j in j - 1..=j + 1 {
                        lookup_key <<= 1;
                        let is_past_border = i < min_i || i > max_i || j < min_j || j > max_j;
                        if (self.border && is_past_border) || self.chunk.contains(&Index(i, j)) {
                            lookup_key |= 1;
                        }
                    }
                }

                if algorithm[lookup_key] {
                    output_chunk.insert(Index(i, j));
                }
            }
        }

        Image {
            chunk: output_chunk,
            chunk_origin: Index(min_i - 1, min_j - 1),
            chunk_size: Index(self.chunk_size.0 + 2, self.chunk_size.1 + 2),
            border: if self.border { algorithm[0b111111111] } else { algorithm[0] },
        }
    }

    fn count_pixels(&self) -> usize {
        if self.border {
            panic!("infinite pixels")
        }
        self.chunk.len()
    }
}

fn parse_input<R: BufRead>(reader: R) -> (Image, Vec<bool>) {
    let lines = &mut reader.lines();

    let algorithm = lines.next().unwrap().unwrap().chars().map(|c| match c {
        '#' => true,
        '.' => false,
        _ => unreachable!(),
    }).collect::<Vec<bool>>();

    lines.next();

    let mut chunk = HashSet::new();
    let mut chunk_size = Index(0, 0);
    for (i, line) in lines.enumerate() {
        for (j, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                chunk.insert(Index(i as i32, j as i32));
            }
            chunk_size.1 = j as i32;
        }
        chunk_size.0 = i as i32;
    }

    chunk_size = Index(chunk_size.0 + 1, chunk_size.1 + 1);

    let image = Image {
        chunk,
        chunk_origin: Index(0, 0),
        chunk_size,
        border: false,
    };

    (image, algorithm)
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
    const INFINITY: &[u8] = include_str!("testdata/infinity.txt").as_bytes();

    #[test]
    fn test_part1() {
        assert_eq!(part1(BufReader::new(BASIC)), "35")
    }

    #[test]
    fn test_part1_infinity() {
        assert_eq!(part1(BufReader::new(INFINITY)), "7")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "3351")
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
