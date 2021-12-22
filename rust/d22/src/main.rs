#![feature(test)]

use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    let initialization_volume = Volume {
        0: Range(-50, 51),
        1: Range(-50, 51),
        2: Range(-50, 51),
    };

    let mut reactor = Reactor::default();

    for step in parse_input(reader) {
        if !initialization_volume.contains(&step.volume) {
            continue;
        }

        if step.on {
            reactor.add(step.volume);
        } else {
            reactor.sub(step.volume);
        }
    }

    reactor.count().to_string()
}

fn part2<R: BufRead>(reader: R) -> String {
    let mut reactor = Reactor::default();

    for step in parse_input(reader) {
        if step.on {
            reactor.add(step.volume);
        } else {
            reactor.sub(step.volume);
        }
    }

    reactor.count().to_string()
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Step> {
    reader.lines()
        .map(Result::unwrap)
        .map(|s: String| {
            let mut split = s.splitn(2, " ");
            let mut step = Step::default();
            step.on = split.next().unwrap() == "on";
            let mut split = split.next().unwrap().splitn(3, ",");
            step.volume.0 = parse_range(split.next().unwrap());
            step.volume.1 = parse_range(split.next().unwrap());
            step.volume.2 = parse_range(split.next().unwrap());
            step
        })
        .collect()
}

fn parse_range(s: &str) -> Range {
    let s = s.splitn(2, "=").nth(1).unwrap();
    let mut split = s.splitn(2, "..");
    let i = split.next().unwrap().parse::<i32>().unwrap();
    let j = split.next().unwrap().parse::<i32>().unwrap() + 1;
    Range(i, j)
}

#[derive(Default)]
struct Step {
    volume: Volume,
    on: bool,
}

#[derive(Default, Copy, Clone)]
struct Volume(Range, Range, Range);

impl Volume {
    fn contains(&self, other: &Self) -> bool {
        self.0.contains(&other.0) && self.1.contains(&other.1) && self.2.contains(&other.2)
    }

    fn sub(self, other: &Self) -> Vec<Self> {
        self.0.trisect(&other.0).into_iter().enumerate()
            .filter(|(_, i_range)| i_range.len() > 0)
            .flat_map(|(i, i_range)| {
                self.1.trisect(&other.1).into_iter().enumerate()
                    .filter(|(_, j_range)| j_range.len() > 0)
                    .flat_map(move |(j, j_range)| {
                        self.2.trisect(&other.2).into_iter().enumerate()
                            .filter(|(_, k_range)| k_range.len() > 0)
                            .filter(move |(k, _)| !(i == 1 && j == 1 && *k == 1))
                            .map(move |(_, k_range)| Volume(i_range, j_range, k_range))
                    })
            })
            .collect()
    }

    fn volume(&self) -> usize {
        self.0.len() * self.1.len() * self.2.len()
    }
}

#[derive(Default, Copy, Clone)]
struct Range(i32, i32);

impl Range {
    fn contains(&self, other: &Self) -> bool {
        other.0 >= self.0 && other.1 <= self.1
    }

    fn trisect(self, other: &Self) -> [Range; 3] {
        let a = min(self.1, max(self.0, other.0));
        let b = max(self.0, min(self.1, other.1));
        [
            Range(self.0, a),
            Range(a, b),
            Range(b, self.1),
        ]
    }

    fn len(&self) -> usize {
        (self.1 - self.0) as usize
    }
}

#[derive(Default)]
struct Reactor(Vec<Volume>);

impl Reactor {
    fn add(&mut self, volume: Volume) {
        if !self.0.iter().any(|v| v.contains(&volume)) {
            self.sub(volume);
            self.0.push(volume);
        }
    }

    fn sub(&mut self, volume: Volume) {
        self.0 = self.0.iter().flat_map(|v| v.sub(&volume)).collect();
    }

    fn count(&self) -> usize {
        self.0.iter().map(Volume::volume).sum()
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

    // #[bench]
    // fn bench_part2(b: &mut Bencher) {
    //     let input = fs::read_to_string("input.txt").unwrap();
    //     let input = input.as_bytes();
    //     b.iter(|| part2(BufReader::new(input)))
    // }
}
