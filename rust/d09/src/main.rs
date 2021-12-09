#![feature(test)]

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    let grid = reader.lines()
        .map(Result::unwrap)
        .map(|line| {
            line
                .chars()
                .map(|c| { (c as u8) - ('0' as u8) })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    (0..grid.len())
        .map(|i| {
            (0..grid[1].len())
                .filter(|&j| {
                    (i == 0 || grid[i - 1][j] > grid[i][j]) &&
                        (i == grid.len() - 1 || grid[i + 1][j] > grid[i][j]) &&
                        (j == 0 || grid[i][j - 1] > grid[i][j]) &&
                        (j == grid[0].len() - 1 || grid[i][j + 1] > grid[i][j])
                })
                .map(|j| { (grid[i][j] as u32) + 1 })
                .sum::<u32>()
        })
        .sum::<u32>()
        .to_string()
}

fn part2<R: BufRead>(reader: R) -> String {
    let grid = reader.lines()
        .map(Result::unwrap)
        .map(|line| {
            line
                .chars()
                .map(|c| { (c as u8) - ('0' as u8) })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    let mut basins = (0..grid.len())
        .map(|i| {
            (0..grid[0].len())
                .map(|j| { flood(&grid, &mut visited, i, j) })
                .filter(|&v| { v > 0 })
                .collect::<Vec<u32>>()
        })
        .flatten()
        .collect::<Vec<u32>>();

    basins.sort();

    basins[basins.len() - 3..basins.len()]
        .iter()
        .fold(1, |acc, v| { acc * v })
        .to_string()
}

fn flood(grid: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) -> u32 {
    if visited[i][j] || grid[i][j] == 9 {
        return 0;
    }

    let mut result = 0u32;

    let mut queue: Vec<(usize, usize)> = Vec::new();

    queue.push((i, j));

    while let Some(pos) = queue.pop() {
        let (i, j) = pos;

        if visited[i][j] || grid[i][j] == 9 {
            continue;
        }

        visited[i][j] = true;
        result += 1;

        if i > 0 {
            queue.push((i - 1, j));
        }
        if i < grid.len() - 1 {
            queue.push((i + 1, j));
        }
        if j > 0 {
            queue.push((i, j - 1));
        }
        if j < grid[0].len() - 1 {
            queue.push((i, j + 1));
        }
    }

    result
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
        assert_eq!(part1(BufReader::new(BASIC)), "15")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "1134")
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
