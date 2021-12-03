#![feature(test)]

use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: Read>(reader: BufReader<R>) -> String {
    let (numbers, num_digits) = parse_input(reader);

    let gamma = (0..num_digits).fold(0, |accum, index_rl| {
        if compare_ones_occurrences(&numbers, index_rl).is_gt() {
            accum | 1 << index_rl
        } else {
            accum
        }
    });

    let epsilon = bitwise_invert(gamma, num_digits);

    return (gamma * epsilon).to_string();
}

fn part2<R: Read>(reader: BufReader<R>) -> String {
    let (numbers, num_digits) = parse_input(reader);

    let oxygen = calculate_rating(&numbers, num_digits, Ordering::is_ge);
    let co2 = calculate_rating(&numbers, num_digits, Ordering::is_lt);

    return (oxygen * co2).to_string();
}

fn parse_input<R: Read>(reader: BufReader<R>) -> (Vec<u32>, u32) {
    let mut num_digits = 0;

    let numbers: Vec<u32> = reader.lines()
        .map(|line| { line.unwrap() })
        .inspect(|line| { num_digits = line.len() as u32 })
        .map(|line| { u32::from_str_radix(line.as_str(), 2).unwrap() })
        .collect();

    (numbers, num_digits)
}

fn compare_ones_occurrences(numbers: &Vec<u32>, index_rl: u32) -> Ordering {
    let num_ones = numbers.iter().fold(0, |accum, &number| {
        accum + (number >> index_rl & 1)
    });

    (num_ones * 2).cmp(&(numbers.len() as u32))
}

fn bitwise_invert(value: u32, num_digits: u32) -> u32 {
    (u32::MAX ^ value) & (1 << num_digits) - 1
}

fn calculate_rating(numbers: &Vec<u32>, num_digits: u32, is_one_desired: fn(ones_ordering: Ordering) -> bool) -> u32 {
    let mut candidates = numbers.clone();

    for index_lr in (0..num_digits).rev() {
        let ones_ordering = compare_ones_occurrences(&candidates, index_lr);
        let desired_digit: u32 = if is_one_desired(ones_ordering) { 1 } else { 0 };
        candidates = candidates.into_iter()
            .filter(|&candidate| {
                (candidate & 1 << index_lr) >> index_lr == desired_digit
            })
            .collect();
        if candidates.len() <= 1 {
            break;
        }
    }

    candidates[0].into()
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
        assert_eq!(part1(BufReader::new(BASIC)), "198")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "230")
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
