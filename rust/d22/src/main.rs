#![feature(test)]

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    let steps = parse_input(reader);
    let mut grid: [[[bool; 101]; 101]; 101] = [[[false; 101]; 101]; 101];
    let max_area = Area {
        x: RangeInclusive(-50, 50),
        y: RangeInclusive(-50, 50),
        z: RangeInclusive(-50, 50),
    };
    for step in steps {
        if !step.area.within(&max_area) {
            continue;
        }
        for x in step.area.x.0 + 50..=step.area.x.1 + 50 {
            for y in step.area.y.0 + 50..=step.area.y.1 + 50 {
                for z in step.area.z.0 + 50..=step.area.z.1 + 50 {
                    grid[x as usize][y as usize][z as usize] = step.on;
                }
            }
        }
    }

    let mut total = 0u64;

    for i in 0..=100 {
        for j in 0..=100 {
            for k in 0..=100 {
                if grid[i][j][k] {
                    total += 1;
                }
            }
        }
    }

    total.to_string()
}

fn part2<R: BufRead>(reader: R) -> String {
    String::new()
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Step> {
    reader.lines()
        .map(Result::unwrap)
        .map(|s: String| {
            let mut split = s.splitn(2, " ");
            let mut step = Step::default();
            step.on = split.next().unwrap() == "on";
            let mut split = split.next().unwrap().splitn(3, ",");
            step.area.x = parse_range(split.next().unwrap());
            step.area.y = parse_range(split.next().unwrap());
            step.area.z = parse_range(split.next().unwrap());
            step
        })
        .collect()
}

fn parse_range(s: &str) -> RangeInclusive {
    let s = s.splitn(2, "=").nth(1).unwrap();
    let mut split = s.splitn(2, "..");
    let i = split.next().unwrap().parse().unwrap();
    let j = split.next().unwrap().parse().unwrap();
    RangeInclusive(i, j)
}

#[derive(Default)]
struct Step {
    area: Area,
    on: bool,
}

#[derive(Default)]
struct Area {
    x: RangeInclusive,
    y: RangeInclusive,
    z: RangeInclusive,
}

impl Area {
    fn within(&self, other: &Self) -> bool {
        self.x.within(&other.x) && self.y.within(&other.y) && self.z.within(&other.z)
    }
}

#[derive(Default)]
struct RangeInclusive(i32, i32);

impl RangeInclusive {
    fn within(&self, other: &Self) -> bool {
        self.0 >= other.0 && self.1 <= other.1
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
