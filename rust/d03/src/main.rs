#![feature(test)]

use std::cmp::Ordering;
use std::convert::identity;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: Read>(reader: BufReader<R>) -> String {
    let (numbers, num_digits) = parse_input(reader);

    let gamma = (0..num_digits).fold(0, |accum, index_rl| {
        if is_majority_ones_at_index(&numbers, index_rl) {
            accum | 1 << index_rl
        } else {
            accum
        }
    });

    let epsilon = bitwise_invert(gamma, num_digits);

    return (gamma * epsilon).to_string();
}

fn part2<R: Read>(reader: BufReader<R>) -> String {
    let (mut numbers, num_digits) = parse_input(reader);

    numbers.sort();

    let oxygen = calculate_rating(&numbers, num_digits, Ordering::is_gt);
    let co2 = calculate_rating(&numbers, num_digits, Ordering::is_le);

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

fn is_majority_ones_at_index(numbers: &Vec<u32>, index_rl: u32) -> bool {
    let num_ones = numbers.iter().fold(0, |accum, &number| {
        accum + (number >> index_rl & 1)
    });

    (num_ones * 2) as usize > numbers.len()
}

fn bitwise_invert(value: u32, num_digits: u32) -> u32 {
    (u32::MAX ^ value) & (1 << num_digits) - 1
}

fn calculate_rating(numbers: &Vec<u32>, num_digits: u32, should_follow_zeros: fn(zeros_compared_to_ones: Ordering) -> bool) -> u32 {
    let mut candidates = &numbers[..];
    let mut mask = 1 << num_digits - 1;

    while candidates.len() > 1 {
        let ones_seek = mask + 1 >> 1;
        let ones_index = candidates
            .binary_search_by(|number| { (number & mask).cmp(&ones_seek) })
            .unwrap_or_else(identity);

        if ones_index < candidates.len() {
            let zeros_compared_to_ones = (ones_index * 2).cmp(&candidates.len());

            candidates = if should_follow_zeros(zeros_compared_to_ones) {
                &candidates[0..ones_index]
            } else {
                &candidates[ones_index..candidates.len()]
            };
        }

        mask >>= 1;
    }

    candidates[0]
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

    #[test]
    fn test_part2_dense() {
        assert_eq!(part2(BufReader::new(include_str!("testdata/dense.txt").as_bytes())), "12")
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
