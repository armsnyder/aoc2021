#![feature(test)]

use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::AddAssign;

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

#[derive(PartialEq, Clone, Copy)]
struct Point(i16, i16);

impl FromIterator<i16> for Point {
    fn from_iter<T: IntoIterator<Item=i16>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Point(iter.next().unwrap(), iter.next().unwrap())
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Clone)]
struct Line(Point, Point);

impl FromIterator<Point> for Line {
    fn from_iter<T: IntoIterator<Item=Point>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Line(iter.next().unwrap(), iter.next().unwrap())
    }
}

fn parse_line(s: String) -> Line {
    s
        .split(" -> ")
        .map(|point| {
            point
                .split(",")
                .map(str::parse)
                .map(Result::unwrap)
                .collect::<Point>()
        })
        .collect::<Line>()
}

fn no_diagonals(line: &Line) -> bool {
    line.0.0 == line.1.0 || line.0.1 == line.1.1
}

fn draw_line(line: &Line, grid: &mut Grid) {
    let delta = calculate_delta(&line);

    let mut cur = line.0.clone();

    grid[cur.0 as usize][cur.1 as usize] += 1;

    while cur != line.1 {
        cur += delta;
        grid[cur.0 as usize][cur.1 as usize] += 1;
    }
}

fn calculate_delta(line: &Line) -> Point {
    Point(
        calculate_delta_coord(line.1.0.cmp(&line.0.0)),
        calculate_delta_coord(line.1.1.cmp(&line.0.1)),
    )
}

fn calculate_delta_coord(ordering: Ordering) -> i16 {
    match ordering {
        Ordering::Equal => 0,
        Ordering::Greater => 1,
        Ordering::Less => -1,
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
