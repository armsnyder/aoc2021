#![feature(test)]

use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

use pathfinding::directed::astar::astar;

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    solve(reader, 1)
}

fn part2<R: BufRead>(reader: R) -> String {
    solve(reader, 5)
}

fn solve<R: BufRead>(reader: R, factor: usize) -> String {
    let mut cavern = parse_input(reader);

    if factor > 1 {
        let i_window = cavern.len();
        let j_window = cavern[0].len();

        for i in 0..i_window {
            cavern[i].resize(j_window * factor, 0);

            for j in j_window..cavern[i].len() {
                cavern[i][j] = cavern[i][j - j_window] + 1;
                if cavern[i][j] > 9 {
                    cavern[i][j] = 1;
                }
            }
        }

        cavern.resize(i_window * factor, vec![]);

        for i in i_window..cavern.len() {
            cavern[i] = cavern[i - i_window]
                .iter()
                .map(|&v| if v < 9 { v + 1 } else { 1 })
                .collect()
        }
    }

    let start = Pos(0, 0);
    let goal = Pos((cavern.len() - 1) as i16, (cavern[0].len() - 1) as i16);

    let successors = |p: &Pos| -> Vec<(Pos, u16)> {
        vec![Pos(p.0 - 1, p.1), Pos(p.0 + 1, p.1), Pos(p.0, p.1 - 1), Pos(p.0, p.1 + 1)]
            .into_iter()
            .filter(|p| p.0 >= 0 && (p.0 as usize) < cavern.len() && p.1 >= 0 && (p.1 as usize) < cavern[0].len())
            .map(|p| (p, cavern[p.0 as usize][p.1 as usize] as u16))
            .collect()
    };

    let heuristic = |p: &Pos| ((goal.0 - p.0) as u16) + ((goal.1 - p.1) as u16);

    let success = |p: &Pos| *p == goal;

    astar(&start, successors, heuristic, success).unwrap().1.to_string()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Pos(i16, i16);

fn parse_input<R: BufRead>(reader: R) -> Vec<Vec<u8>> {
    reader
        .lines()
        .map(Result::unwrap)
        .map(|line: String| {
            line
                .into_bytes()
                .into_iter()
                .map(|c| { c - '0' as u8 })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
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
        assert_eq!(part1(BufReader::new(BASIC)), "40")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "315")
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
