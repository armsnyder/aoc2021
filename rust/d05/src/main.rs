#![feature(test)]

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    let mut grid: Grid = [[0; SIZE]; SIZE];

    reader.lines()
        .map(Result::unwrap)
        .map(parse_line)
        .filter(no_diagonals)
        .for_each(|line| { draw_line(&line, &mut grid) });

    count_overlaps(grid).to_string()
}

fn part2<R: BufRead>(reader: R) -> String {
    let mut grid: Grid = [[0; SIZE]; SIZE];

    reader.lines()
        .map(Result::unwrap)
        .map(parse_line)
        .for_each(|line| { draw_line(&line, &mut grid) });

    count_overlaps(grid).to_string()
}

const SIZE: usize = 1000;

type Grid = [[u8; SIZE]; SIZE];
type Point = (i16, i16);
type Line = (Point, Point);

fn parse_line(s: String) -> Line {
    let points = s
        .split(" -> ")
        .map(|point| {
            let coords = point
                .split(",")
                .map(str::parse)
                .map(Result::unwrap)
                .collect::<Vec<i16>>();
            (coords[0], coords[1])
        })
        .collect::<Vec<Point>>();
    (points[0], points[1])
}

fn no_diagonals(line: &Line) -> bool {
    return line.0.0 == line.1.0 || line.0.1 == line.1.1;
}

fn draw_line(line: &Line, grid: &mut Grid) {
    let di = match line.1.0.cmp(&line.0.0) {
        core::cmp::Ordering::Equal => 0,
        core::cmp::Ordering::Greater => 1,
        core::cmp::Ordering::Less => -1,
    };

    let dj = match line.1.1.cmp(&line.0.1) {
        core::cmp::Ordering::Equal => 0,
        core::cmp::Ordering::Greater => 1,
        core::cmp::Ordering::Less => -1,
    };

    let mut cur = line.0;

    grid[cur.0 as usize][cur.1 as usize] += 1;
    while cur != line.1 {
        cur = (cur.0 + di, cur.1 + dj);
        grid[cur.0 as usize][cur.1 as usize] += 1;
    }
}

fn count_overlaps(grid: Grid) -> usize {
    grid.iter()
        .flatten()
        .filter(|&&v| { v > 1u8 })
        .count()
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
        assert_eq!(part1(BufReader::new(BASIC)), "5")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "12")
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
