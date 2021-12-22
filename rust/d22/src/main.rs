#![feature(test)]

use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Sum;
use std::ops::Sub;

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    let initialization_volume = Volume(
        Range(-50, 51),
        Range(-50, 51),
        Range(-50, 51),
    );

    let steps = parse_input(reader).into_iter()
        .filter(|step| initialization_volume.contains(&step.volume))
        .collect();

    count_cubes_after_steps(steps).to_string()
}

fn part2<R: BufRead>(reader: R) -> String {
    let steps = parse_input(reader);
    count_cubes_after_steps(steps).to_string()
}

fn count_cubes_after_steps(steps: Vec<Step>) -> usize {
    let mut processed_steps = Vec::new();

    for step in steps {
        processed_steps.append(&mut processed_steps.iter()
            .filter_map(|processed| *processed - step)
            .collect());

        if step.on {
            processed_steps.push(step);
        }
    }

    processed_steps.iter().sum()
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Step> {
    reader.lines()
        .map(Result::unwrap)
        .map(|s: String| {
            let (on, rest) = s.split_once(' ').unwrap();
            let ranges = rest.splitn(3, ',')
                .map(|s| {
                    let (start, end) = s[2..].split_once("..").unwrap();
                    Range(start.parse().unwrap(), end.parse::<i32>().unwrap() + 1)
                })
                .collect::<Vec<Range>>();
            Step { on: on == "on", volume: Volume(ranges[0], ranges[1], ranges[2]) }
        })
        .collect()
}

#[derive(Copy, Clone)]
struct Step {
    volume: Volume,
    on: bool,
}

impl Sub for Step {
    type Output = Option<Step>;

    fn sub(self, rhs: Self) -> Self::Output {
        let volume = self.volume - rhs.volume;
        if volume.size() == 0 {
            None
        } else {
            Some(Step {
                volume,
                on: if self.on == rhs.on { !rhs.on } else { rhs.on },
            })
        }
    }
}

impl<'a> Sum<&'a Step> for usize {
    fn sum<I: Iterator<Item=&'a Step>>(iter: I) -> Self {
        iter.fold(0, |acc, step| {
            if step.on {
                acc + step.volume.size()
            } else {
                acc - step.volume.size()
            }
        })
    }
}

#[derive(Copy, Clone)]
struct Volume(Range, Range, Range);

impl Volume {
    fn contains(&self, other: &Self) -> bool {
        self.0.contains(&other.0) && self.1.contains(&other.1) && self.2.contains(&other.2)
    }

    fn size(&self) -> usize {
        self.0.len() * self.1.len() * self.2.len()
    }
}

impl Sub for Volume {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Volume(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
        )
    }
}

#[derive(Copy, Clone)]
struct Range(i32, i32);

impl Sub for Range {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Range(min(self.1, max(self.0, rhs.0)), max(self.0, min(self.1, rhs.1)))
    }
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        other.0 >= self.0 && other.1 <= self.1
    }

    fn len(&self) -> usize {
        (self.1 - self.0) as usize
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

    const SMALL: &[u8] = include_str!("testdata/small.txt").as_bytes();
    const MEDIUM: &[u8] = include_str!("testdata/medium.txt").as_bytes();
    const LARGE: &[u8] = include_str!("testdata/large.txt").as_bytes();

    #[test]
    fn test_part1_small() {
        assert_eq!(part1(BufReader::new(SMALL)), "39")
    }

    #[test]
    fn test_part1_medium() {
        assert_eq!(part1(BufReader::new(MEDIUM)), "590784")
    }

    #[test]
    fn test_part1_large() {
        assert_eq!(part1(BufReader::new(LARGE)), "474140")
    }

    #[test]
    fn test_part2_large() {
        assert_eq!(part2(BufReader::new(LARGE)), "2758514936282235")
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
