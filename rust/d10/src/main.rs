#![feature(test)]

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    let mut stack = Vec::new();

    reader
        .lines()
        .map(Result::unwrap)
        .filter_map(|line| { autocomplete_line(&line, &mut stack).err() })
        .map(syntax_error_score)
        .sum::<u32>()
        .to_string()
}

fn part2<R: BufRead>(reader: R) -> String {
    let mut stack = Vec::new();

    let mut scores = reader
        .lines()
        .map(Result::unwrap)
        .filter_map(|line| { autocomplete_line(&line, &mut stack).ok() })
        .map(autocomplete_tool_score)
        .collect::<Vec<u64>>();

    scores.sort();

    scores[scores.len() / 2].to_string()
}

// autocomplete_line returns Ok with the autocompleted characters if the line is valid but
// incomplete, or Err with the first invalid character if the line is invalid.
fn autocomplete_line(line: &str, stack: &mut Vec<char>) -> Result<String, char> {
    stack.clear();

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => {
                stack.push(c);
            }
            _ => {
                match stack.pop() {
                    Some(p) => {
                        if p != invert(c) {
                            return Err(c);
                        }
                    }
                    None => {
                        return Err(c);
                    }
                }
            }
        }
    }

    let mut result = String::new();

    while let Some(c) = stack.pop() {
        result.push(invert(c));
    }

    return Ok(result);
}

fn invert(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("illegal char {}", c)
    }
}

fn syntax_error_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("illegal char {}", c)
    }
}

fn autocomplete_tool_score(remainder: String) -> u64 {
    remainder
        .chars()
        .fold(0, |acc, c| {
            acc * 5 + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("illegal char {}", c)
            }
        })
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
        assert_eq!(part1(BufReader::new(BASIC)), "26397")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "288957")
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
