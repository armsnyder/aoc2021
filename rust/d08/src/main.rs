#![feature(test)]

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    solve(reader, count_unique_segment_outputs)
}

fn part2<R: BufRead>(reader: R) -> String {
    solve(reader, unscramble_output)
}

fn solve<R: BufRead>(reader: R, line_fn: impl Fn(&str) -> u32) -> String {
    reader
        .lines()
        .map(Result::unwrap)
        .map(|line| { line_fn(&line) })
        .sum::<u32>()
        .to_string()
}

fn count_unique_segment_outputs(line: &str) -> u32 {
    line
        .split(" | ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .filter(|pattern| {
            match pattern.len() {
                2 | 4 | 3 | 7 => true,
                _ => false,
            }
        })
        .count() as u32
}

fn unscramble_output(line: &str) -> u32 {
    let mut line = line.split(" | ");

    let scrambled_frequencies = line
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<CompleteDistribution>();

    line
        .next()
        .unwrap()
        .split_whitespace()
        .map(|pattern| { solve_scrambled_pattern(pattern, &scrambled_frequencies) })
        .fold(0u32, |acc, n| { acc * 10 + n })
}

const NUM_SEGMENTS: usize = 7;

// How frequently a segment occurs, given each total lit segments. The index refers to the number of
// lit segments, 0-indexed. NB: The first two indices are never utilized, and the last index is
// always 1.
#[derive(PartialEq, Default)]
struct FrequencyDistribution([u8; NUM_SEGMENTS + 1]);

impl FrequencyDistribution {
    fn solve_char_index(&self) -> usize {
        for i in 0..NUM_SEGMENTS {
            if *self == CompleteDistribution::WELL_KNOWN.0[i] {
                return i;
            }
        }
        panic!("no solution for distribution")
    }
}

// All display segments, in terms of how frequently they occur for a number with a given number of
// total lit segments.
#[derive(Default)]
struct CompleteDistribution([FrequencyDistribution; NUM_SEGMENTS]);

impl<'a> FromIterator<&'a str> for CompleteDistribution {
    fn from_iter<T: IntoIterator<Item=&'a str>>(iter: T) -> Self {
        let mut output = CompleteDistribution::default();

        iter.into_iter().for_each(|pattern| {
            pattern.chars().for_each(|ch| {
                output.0[char_to_index(ch)].0[pattern.len()] += 1
            })
        });

        output
    }
}

impl CompleteDistribution {
    // The frequency distributions with unscrambled settings.
    const WELL_KNOWN: CompleteDistribution = CompleteDistribution([
        FrequencyDistribution([0, 0, 0, 1, 0, 3, 3, 1]), // a
        FrequencyDistribution([0, 0, 0, 0, 1, 1, 3, 1]), // b
        FrequencyDistribution([0, 0, 1, 1, 1, 2, 2, 1]), // c
        FrequencyDistribution([0, 0, 0, 0, 1, 3, 2, 1]), // d
        FrequencyDistribution([0, 0, 0, 0, 0, 1, 2, 1]), // e
        FrequencyDistribution([0, 0, 1, 1, 1, 2, 3, 1]), // f
        FrequencyDistribution([0, 0, 0, 0, 0, 3, 3, 1]), // g
    ]);
}

fn char_to_index(ch: char) -> usize {
    ((ch as u8) - ('a' as u8)) as usize
}

fn solve_scrambled_pattern(pattern: &str, scrambled_frequencies: &CompleteDistribution) -> u32 {
    let mut display = 0u8;

    for ch in pattern.chars() {
        display |= 1 << (NUM_SEGMENTS - 1 - scrambled_frequencies.0[char_to_index(ch)].solve_char_index());
    }

    match display {
        0b1110111 => 0,
        0b0010010 => 1,
        0b1011101 => 2,
        0b1011011 => 3,
        0b0111010 => 4,
        0b1101011 => 5,
        0b1101111 => 6,
        0b1010010 => 7,
        0b1111111 => 8,
        0b1111011 => 9,
        _ => panic!("illegal display {}", display)
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
    const SINGLE: &[u8] = include_str!("testdata/single.txt").as_bytes();

    #[test]
    fn test_part1() {
        assert_eq!(part1(BufReader::new(BASIC)), "26")
    }

    #[test]
    fn test_part1_single() {
        assert_eq!(part1(BufReader::new(SINGLE)), "0")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "61229")
    }

    #[test]
    fn test_part2_single() {
        assert_eq!(part2(BufReader::new(SINGLE)), "5353")
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
