#![feature(test)]

use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: Read>(reader: BufReader<R>) -> String {
    let mut line_count: u32 = 0;
    let mut totals: Vec<u32> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if i == 0 {
            totals.resize(line.len(), 0);
        }
        line_count += 1;
        for (i, ch) in line.chars().enumerate() {
            totals[i] += ch.to_digit(10).unwrap();
        }
    }
    let mask = (2 << totals.len() - 1) - 1;
    let mut gamma = 0;
    let average = line_count / 2;
    for (i, val) in totals.iter().rev().enumerate() {
        if *val > average {
            gamma |= 1 << i;
        }
    }
    let epsilon = (-1 ^ gamma) & mask;
    return (gamma * epsilon).to_string();
}

fn part2<R: Read>(reader: BufReader<R>) -> String {
    let mut values: Vec<u16> = Vec::new();
    let mut columns = 0;
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if i == 0 {
            columns = line.len() as u16;
        }
        let value = u16::from_str_radix(line.as_str(), 2).unwrap();
        values.push(value);
    }
    let oxygen = calculate_rating(&values, columns, |dom| {
        match dom {
            DominantDigit::Ones => 1,
            DominantDigit::Zeros => 0,
            DominantDigit::Equal => 1,
        }
    });
    let co2 = calculate_rating(&values, columns, |dom| {
        match dom {
            DominantDigit::Ones => 0,
            DominantDigit::Zeros => 1,
            DominantDigit::Equal => 0,
        }
    });
    return ((oxygen as u32) * (co2 as u32)).to_string();
}

enum DominantDigit {
    Ones,
    Zeros,
    Equal,
}

fn calculate_rating(values: &Vec<u16>, columns: u16, keeper: fn(DominantDigit) -> u16) -> u16 {
    let mut considering = values.clone();
    for i in 0..columns {
        let minus_i = (columns - i - 1) as u16;
        let keep = keeper(find_dominant_digit(&considering, minus_i));
        let mut next_considering = Vec::new();
        for consider in considering {
            let digit = (consider & (1 << minus_i)) >> minus_i;
            if digit == keep {
                next_considering.push(consider);
            }
        }
        considering = next_considering;
        if considering.len() == 1 {
            break;
        }
    }
    return *considering.iter().next().unwrap();
}

fn find_dominant_digit(values: &Vec<u16>, i: u16) -> DominantDigit {
    let mut total = 0;
    let half = values.len() / 2;
    let ceil_half = if half * 2 == values.len() { half } else { half + 1 };
    for v in values {
        if (v & (1 << i)) >> i == 1 {
            total += 1;
        }
    }
    match total {
        total if total > half => DominantDigit::Ones,
        total if total < ceil_half => DominantDigit::Zeros,
        _ => DominantDigit::Equal,
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
