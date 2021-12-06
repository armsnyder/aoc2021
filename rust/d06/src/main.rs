#![feature(test)]

use std::fs::{File};
use std::io::{BufRead, BufReader};

const MAX_DAYS: usize = 256;

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    solve_days(reader, 80)
}

fn part2<R: BufRead>(reader: R) -> String {
    solve_days(reader, 256)
}

fn solve_days<R: BufRead>(reader: R, days: i32) -> String {
    let mut memo: [u64; MAX_DAYS] = [0; MAX_DAYS];
    reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .fold(0, |accum, offset| { accum + solve_single_fish(days - offset, &mut memo) })
        .to_string()
}

fn solve_single_fish(days: i32, memo: &mut [u64; MAX_DAYS]) -> u64 {
    if days <= 0 {
        1
    } else if memo[days as usize] != 0 {
        memo[days as usize]
    } else {
        let mut countdown_days = days;
        let mut result = 1;
        while countdown_days > 0 {
            result += solve_single_fish(countdown_days - 9, memo);
            countdown_days -= 7;
        }
        memo[days as usize] = result;
        result
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
        assert_eq!(part1(BufReader::new(BASIC)), "5934")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "26984457539")
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
