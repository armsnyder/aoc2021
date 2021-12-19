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

    let min_vy = target_area.y.start;
    let max_vy = -target_area.y.start - 1;
    // Using the quadratic formula to solve for "n" in the Triangular number formula.
    // See: https://en.wikipedia.org/wiki/Triangular_number#Formula
    // This is a minor optimization over just using "0" as the min x velocity.
    let min_vx = (-1 + (((1 + 8 * target_area.x.start) as f32).sqrt() as i32)) / 2;
    let max_vx = target_area.x.end;

    (min_vy..=max_vy)
        .map(|vy| simulate_y_get_steps(vy, &target_area.y))
        .map(|steps| (min_vx..=max_vx)
            .filter(|&vx| simulate_x(vx, &target_area.x, &steps))
            .count())
        .sum::<usize>()
        .to_string()
}

fn simulate_y_get_steps(vy: i32, target_y: &RangeInclusive) -> Vec<i32> {
    let mut output = Vec::new();
    let mut vy = vy;
    let mut py = 0;
    let mut steps = 0;

    while py > target_y.start {
        steps += 1;
        py += vy;
        vy -= 1;

        if target_y.contains(py) {
            output.push(steps)
        }
    }

    output
}

fn simulate_x(vx: i32, target_x: &RangeInclusive, steps: &Vec<i32>) -> bool {
    let mut vx = vx;
    let mut px = 0;
    let mut step = 0;

    for want_step in steps {
        while step < *want_step {
            step += 1;
            px += vx;
            vx = match vx.cmp(&0) {
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Less => vx + 1,
                std::cmp::Ordering::Greater => vx - 1,
            }
        }

        if target_x.contains(px) {
            return true;
        }
    }

    false
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

impl RangeInclusive {
    fn contains(&self, n: i32) -> bool {
        self.start <= n && n <= self.end
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
