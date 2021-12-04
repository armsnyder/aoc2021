#![feature(test)]

use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    let mut prev = i32::MAX;
    let mut total = 0;
    for line in reader.lines() {
        let cur = line.unwrap().parse().unwrap();
        if cur > prev {
            total += 1;
        }
        prev = cur;
    }
    return total.to_string();
}

fn part2<R: BufRead>(reader: R) -> String {
    let mut total = 0;
    let mut buf = [0; 3];
    for (i, line) in reader.lines().enumerate() {
        let cur = line.unwrap().parse().unwrap();
        if i >= 3 && cur > buf[i % 3] {
            total += 1;
        }
        buf[i % 3] = cur;
    }
    return total.to_string();
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
        assert_eq!(part1(BufReader::new(BASIC)), "7")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "5")
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
