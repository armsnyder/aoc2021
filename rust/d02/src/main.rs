#![feature(test)]

use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: Read>(reader: BufReader<R>) -> String {
    let mut distance = 0;
    let mut depth = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.split_whitespace();
        let direction = line.next().unwrap();
        let value = line.next().unwrap();
        let value: i32 = value.parse().unwrap();
        match direction {
            "forward" => distance += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => (),
        }
    }
    (distance * depth).to_string()
}

fn part2<R: Read>(reader: BufReader<R>) -> String {
    let mut aim = 0;
    let mut horiz = 0;
    let mut depth = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.split_whitespace();
        let direction = line.next().unwrap();
        let value = line.next().unwrap();
        let value: i32 = value.parse().unwrap();
        match direction {
            "forward" => {
                horiz += value;
                depth += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => (),
        }
    }
    (horiz * depth).to_string()
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
        assert_eq!(part1(BufReader::new(BASIC)), "150")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "900")
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
