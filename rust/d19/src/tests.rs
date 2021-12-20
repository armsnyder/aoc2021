extern crate test;

use std::fs;
use test::Bencher;

use super::*;

const BASIC: &[u8] = include_str!("testdata/basic.txt").as_bytes();

#[test]
fn test_part1() {
    assert_eq!(part1(BufReader::new(BASIC)), "79")
}

#[test]
fn test_part2() {
    assert_eq!(part2(BufReader::new(BASIC)), "3621")
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
