#![feature(test)]
extern crate test;

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use test::Bencher;

fn main() {
    println!("Part 1: {}", part1(read_input()).unwrap());
    println!("Part 2: {}", part2(read_input()).unwrap());
}

fn part1<R: Read>(reader: BufReader<R>) -> Result<String, Error> {
    let mut prev = i32::MAX;
    let mut total = 0;
    for line in reader.lines() {
        let cur = line.unwrap().parse().unwrap();
        if cur > prev {
            total += 1;
        }
        prev = cur;
    }
    return Ok(total.to_string());
}

fn part2<R: Read>(reader: BufReader<R>) -> Result<String, Error> {
    let mut total = 0;
    let mut buf = [0; 3];
    for (i, line) in reader.lines().enumerate() {
        let cur = line.unwrap().parse().unwrap();
        if i >= 3 && cur > buf[i % 3] {
            total += 1;
        }
        buf[i % 3] = cur;
    }
    return Ok(total.to_string());
}

fn read_input() -> BufReader<File> {
    BufReader::new(File::open("input.txt").unwrap())
}

#[bench]
fn bench_part1(b: &mut Bencher) {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.as_bytes();
    b.iter(|| {
        part1(BufReader::new(input)).unwrap()
    })
}

#[bench]
fn bench_part2(b: &mut Bencher) {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.as_bytes();
    b.iter(|| {
        part2(BufReader::new(input)).unwrap()
    })
}
