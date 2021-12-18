#![feature(test)]

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    let target_area = parse_input(reader);
    (0..-target_area.y.start).fold(0, |acc, i| acc + i).to_string()
}

fn part2<R: BufRead>(reader: R) -> String {
    let target_area = parse_input(reader);
    let max_initial_vel_y = -target_area.y.start;
    let min_initial_vel_y = 0;

    // let initial_vel_y = -target_area.y.start - 1;
    // let num_steps = (-target_area.y.start) * 2;

    return String::new();
}

fn parse_input<R: BufRead>(reader: R) -> Area {
    Area::from(reader.lines().next().unwrap().unwrap().strip_prefix("target area: ").unwrap())
}

struct Area {
    x: RangeInclusive,
    y: RangeInclusive,
}

impl From<&str> for Area {
    fn from(s: &str) -> Self {
        let (x_expr, y_expr) = s.split_once(", ").unwrap();
        Area {
            x: RangeInclusive::from(x_expr.strip_prefix("x=").unwrap()),
            y: RangeInclusive::from(y_expr.strip_prefix("y=").unwrap()),
        }
    }
}

struct RangeInclusive {
    start: i32,
    end: i32,
}

impl From<&str> for RangeInclusive {
    fn from(s: &str) -> Self {
        let (start, end) = s.split_once("..").unwrap();
        RangeInclusive {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
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
        assert_eq!(part1(BufReader::new(BASIC)), "45")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "112")
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
