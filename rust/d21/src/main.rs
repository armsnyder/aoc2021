#![feature(test)]

use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::ops::{Add, Mul};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    let (p1_pos, p2_pos) = parse_input(reader);
    let mut p1_pos = p1_pos as u32;
    let mut p2_pos = p2_pos as u32;
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut num_rolls = 0;
    let mut is_p1_turn = true;

    const MAX_SCORE: u32 = 1000;

    while p1_score < MAX_SCORE && p2_score < MAX_SCORE {
        // Roll next three sequential numbers and sum them.
        let roll = num_rolls * 3 + 6;

        if is_p1_turn {
            p1_pos += roll;
            p1_pos %= 10;
            p1_score += p1_pos + 1;
        } else {
            p2_pos += roll;
            p2_pos %= 10;
            p2_score += p2_pos + 1;
        }

        num_rolls += 3;
        is_p1_turn = !is_p1_turn;
    }

    (min(p1_score, p2_score) * num_rolls).to_string()
}

fn part2<R: BufRead>(reader: R) -> String {
    let (p1_pos, p2_pos) = parse_input(reader);
    let args = State::new(p1_pos, p2_pos);
    let mut memo = HashMap::new();
    let wins = count_multiverse_winners(args, &mut memo);
    max(wins.p1_wins, wins.p2_wins).to_string()
}

fn parse_input<R: BufRead>(reader: R) -> (u8, u8) {
    let lines = &mut reader.lines();

    let p1_pos = read_pos(lines);
    let p2_pos = read_pos(lines);

    (p1_pos, p2_pos)
}

fn read_pos<R: BufRead>(lines: &mut Lines<R>) -> u8 {
    // Position is converted to zero-indexed.
    lines.next().unwrap().unwrap().split(": ").nth(1).unwrap().parse::<u8>().unwrap() - 1
}

fn count_multiverse_winners(state: State, memo: &mut HashMap<State, Wins>) -> Wins {
    if let Some(wins) = memo.get(&state) {
        *wins
    } else {
        const MAX_TURNS: u8 = 21;

        let wins = if state.p1_score >= MAX_TURNS {
            Wins { p1_wins: 1, p2_wins: 0 }
        } else if state.p2_score >= MAX_TURNS {
            Wins { p1_wins: 0, p2_wins: 1 }
        } else {
            // Times that a sum comes up after rolling three three-sided dice.
            const ROLLS: [(u8, u8); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

            ROLLS.iter().fold(Wins::default(), |acc, &(roll, n)| {
                let mut state = state;

                if state.is_p1_turn {
                    state.p1_pos += roll;
                    state.p1_pos %= 10;
                    state.p1_score += state.p1_pos + 1;
                } else {
                    state.p2_pos += roll;
                    state.p2_pos %= 10;
                    state.p2_score += state.p2_pos + 1;
                }

                state.is_p1_turn = !state.is_p1_turn;

                acc + count_multiverse_winners(state, memo) * n
            })
        };

        memo.insert(state, wins);

        wins
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct State {
    p1_score: u8,
    p2_score: u8,
    p1_pos: u8,
    p2_pos: u8,
    is_p1_turn: bool,
}

impl State {
    fn new(p1_pos: u8, p2_pos: u8) -> Self {
        State {
            p1_score: 0,
            p2_score: 0,
            p1_pos,
            p2_pos,
            is_p1_turn: true,
        }
    }
}

#[derive(Default, Clone, Copy)]
struct Wins {
    p1_wins: u64,
    p2_wins: u64,
}

impl Mul<u8> for Wins {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        let rhs = rhs as u64;
        Wins {
            p1_wins: self.p1_wins * rhs,
            p2_wins: self.p2_wins * rhs,
        }
    }
}

impl Add for Wins {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Wins {
            p1_wins: self.p1_wins + rhs.p1_wins,
            p2_wins: self.p2_wins + rhs.p2_wins,
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
        assert_eq!(part1(BufReader::new(BASIC)), "739785")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "444356092776315")
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
