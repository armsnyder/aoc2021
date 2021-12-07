#![feature(test)]

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    solve(reader, get_total_linear_move_cost)
}

fn part2<R: BufRead>(reader: R) -> String {
    solve(reader, get_total_triangular_move_cost)
}

fn solve<R: BufRead>(reader: R, cost_fn: impl Fn(&Vec<i32>, i32) -> i32) -> String {
    descend_gradient(&parse_input(reader), cost_fn).to_string()
}

fn parse_input<R: BufRead>(reader: R) -> Vec<i32> {
    reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<i32>>()
}

fn get_total_linear_move_cost(positions: &Vec<i32>, target_pos: i32) -> i32 {
    positions.iter().fold(0, |accum, pos| {
        accum + (pos - target_pos).abs()
    })
}

fn get_total_triangular_move_cost(positions: &Vec<i32>, target_pos: i32) -> i32 {
    positions.iter().fold(0, |accum, pos| {
        let n = (pos - target_pos).abs();
        accum + (n * (n + 1) / 2)
    })
}

fn descend_gradient(v: &Vec<i32>, cost_fn: impl Fn(&Vec<i32>, i32) -> i32) -> i32 {
    let mut start = 0;
    let mut end = *v.iter().max().unwrap();
    loop {
        let search = start + (end - start) / 2;
        let cost = cost_fn(&v, search);
        let next_cost = cost_fn(&v, search + 1);
        let last_cost = cost_fn(&v, search - 1);
        if cost <= last_cost && cost <= next_cost {
            return cost;
        }
        if last_cost < cost {
            end = search;
        } else {
            start = search + 1;
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
        assert_eq!(part1(BufReader::new(BASIC)), "37")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "168")
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
