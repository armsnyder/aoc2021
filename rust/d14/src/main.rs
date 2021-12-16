#![feature(test)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    solve(reader, 10)
}

fn part2<R: BufRead>(reader: R) -> String {
    solve(reader, 40)
}

fn solve<R: BufRead>(reader: R, steps: usize) -> String {
    let (mut template, rules) = parse_input(reader);

    for _ in 0..steps {
        let mut next_template: Vec<u8> = Vec::with_capacity(template.len());
        for i in 0..template.len() - 1 {
            next_template.push(template[i]);
            let key = &template[i..i + 2];
            let option = rules.get(key);
            if let Some(&c) = option {
                next_template.push(c);
            }
        }
        next_template.push(template[template.len() - 1]);
        template = next_template;
    }

    let mut counts: HashMap<u8, usize> = HashMap::new();

    for c in template {
        counts.insert(c, counts.get(&c).unwrap_or(&0) + 1);
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();

    (max - min).to_string()
}

fn parse_input<R: BufRead>(reader: R) -> (Vec<u8>, HashMap<[u8; 2], u8>) {
    let mut lines = reader.lines().map(Result::unwrap);

    let template = lines.next().unwrap().into_bytes();

    lines.next();

    let mut rules = HashMap::new();

    for line in lines {
        let mut split = line.split(" -> ");
        let mut key = [0; 2];
        for (i, c) in split.next().unwrap().chars().enumerate() {
            key[i] = c as u8;
        }
        let value = split.next().unwrap().chars().next().unwrap() as u8;
        rules.insert(key, value);
    }

    (template, rules)
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
        assert_eq!(part1(BufReader::new(BASIC)), "1588")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "2188189693529")
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
