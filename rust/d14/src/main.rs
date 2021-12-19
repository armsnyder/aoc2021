#![feature(test)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, AddAssign, Index, IndexMut};

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

fn solve<R: BufRead>(reader: R, steps: u8) -> String {
    let element_count = count_elements_after_steps(reader, steps);
    subtract_most_from_least_common_element(element_count).to_string()
}

fn count_elements_after_steps<R: BufRead>(reader: R, steps: u8) -> ElementCount {
    let (template, rules) = parse_input(reader);

    let mut memo: HashMap<MemoKey, ElementCount> = HashMap::new();
    let mut element_count = ElementCount::default();

    for i in 0..template.len() - 1 {
        let pair = [template[i], template[i + 1]];
        element_count += count_elements(pair, steps, &rules, &mut memo);
    }

    // Need to add the last template character manually since it was not counted.
    element_count[template[template.len() - 1]] += 1;

    element_count
}

fn subtract_most_from_least_common_element(element_count: ElementCount) -> u64 {
    let mut least = 0;
    let mut most = 0;

    for i in 0..NUM_LETTERS {
        if element_count[i] > element_count[most] {
            most = i;
        }

        if element_count[least] == 0 || (element_count[i] != 0 && element_count[i] < element_count[least]) {
            least = i;
        }
    }

    element_count[most] - element_count[least]
}

fn count_elements(pair: [char; 2], steps: u8, rules: &HashMap<[char; 2], char>, memo: &mut HashMap<MemoKey, ElementCount>) -> ElementCount {
    let steps = steps - 1;
    let memo_key = MemoKey { steps, pair };

    match memo.get(&memo_key) {
        Some(v) => *v,
        None => if steps == 0 {
            let mut element_count = ElementCount::default();
            let inserted_element = rules[&pair];

            element_count[pair[0]] += 1;
            element_count[inserted_element] += 1;

            element_count
        } else {
            let inserted_element = rules[&pair];
            let left_pair = [pair[0], inserted_element];
            let right_pair = [inserted_element, pair[1]];

            let element_count = count_elements(left_pair, steps, rules, memo) +
                count_elements(right_pair, steps, rules, memo);

            memo.insert(memo_key, element_count);

            element_count
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct MemoKey {
    steps: u8,
    pair: [char; 2],
}

const NUM_LETTERS: usize = 26;

#[derive(Default, Copy, Clone)]
struct ElementCount([u64; NUM_LETTERS]);

impl AddAssign for ElementCount {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..NUM_LETTERS {
            self[i] += rhs[i];
        }
    }
}

impl Add for ElementCount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut output = ElementCount::default();
        for i in 0..NUM_LETTERS {
            output[i] = self[i] + rhs[i];
        }
        output
    }
}

impl Index<usize> for ElementCount {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for ElementCount {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<char> for ElementCount {
    type Output = u64;

    fn index(&self, index: char) -> &Self::Output {
        &self[(index as u8 - 'A' as u8) as usize]
    }
}

impl IndexMut<char> for ElementCount {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        &mut self[(index as u8 - 'A' as u8) as usize]
    }
}

fn parse_input<R: BufRead>(reader: R) -> (Vec<char>, HashMap<[char; 2], char>) {
    let mut lines = reader.lines().map(Result::unwrap);
    let template = lines.next().unwrap().chars().collect::<Vec<_>>();

    lines.next();

    let mut rules = HashMap::new();

    for line in lines {
        let mut split = line.split(" -> ");
        let mut key = [char::default(); 2];
        for (i, c) in split.next().unwrap().chars().enumerate() {
            key[i] = c;
        }
        let value = split.next().unwrap().chars().next().unwrap();
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
