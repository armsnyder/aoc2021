#![feature(test)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Sum;
use std::ops::Add;

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    reader.lines()
        .map(Result::unwrap)
        .map(Number::from)
        .sum::<Number>()
        .magnitude()
        .to_string()
}

fn part2<R: BufRead>(reader: R) -> String {
    let numbers = reader.lines()
        .map(Result::unwrap)
        .map(Number::from)
        .collect::<Vec<Number>>();

    let mut max_magnitude = 0;

    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                let magnitude = (numbers[i].clone() + numbers[j].clone()).magnitude();
                if magnitude > max_magnitude {
                    max_magnitude = magnitude;
                }
            }
        }
    }

    max_magnitude.to_string()
}

#[derive(Debug, PartialEq, Clone)]
enum Number {
    Single(i32),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    fn reduce(&mut self) {
        while self.reduce_explode_once() || self.reduce_split_once() {}
    }

    fn reduce_explode_once(&mut self) -> bool {
        self.reduce_explode_once_depth(0).is_some()
    }

    fn reduce_explode_once_depth(&mut self, depth: i32) -> Option<(i32, i32)> {
        let depth = depth + 1;
        match self {
            Number::Pair(l, r) => {
                if depth == 5 {
                    let explosion = Some(self.get_exploding_numbers());
                    *self = Number::Single(0);
                    explosion
                } else if let Some(explosion) = l.reduce_explode_once_depth(depth) {
                    r.propagate_explosion_left(explosion.1);
                    Some((explosion.0, 0))
                } else if let Some(explosion) = r.reduce_explode_once_depth(depth) {
                    l.propagate_explosion_right(explosion.0);
                    Some((0, explosion.1))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn propagate_explosion_left(&mut self, explosion: i32) {
        if explosion > 0 {
            match self {
                Number::Single(n) => *self = Number::Single(*n + explosion),
                Number::Pair(l, _) => l.propagate_explosion_left(explosion),
            }
        }
    }

    fn propagate_explosion_right(&mut self, explosion: i32) {
        if explosion > 0 {
            match self {
                Number::Single(n) => *self = Number::Single(*n + explosion),
                Number::Pair(_, r) => r.propagate_explosion_right(explosion),
            }
        }
    }

    fn get_exploding_numbers(&self) -> (i32, i32) {
        match self {
            Number::Pair(l, r) => (
                match **l {
                    Number::Single(n) => n,
                    _ => unreachable!(),
                },
                match **r {
                    Number::Single(n) => n,
                    _ => unreachable!(),
                },
            ),
            _ => unreachable!(),
        }
    }

    fn reduce_split_once(&mut self) -> bool {
        match self {
            Number::Single(n) => {
                let n = *n;
                if n > 9 {
                    let half = n / 2;
                    let remainder = n % 2;
                    *self = Number::Pair(
                        Box::new(Number::Single(half)),
                        Box::new(Number::Single(half + remainder)),
                    );
                    true
                } else {
                    false
                }
            }
            Number::Pair(l, r) => l.reduce_split_once() || r.reduce_split_once()
        }
    }

    fn magnitude(&self) -> i32 {
        match self {
            Number::Single(n) => *n,
            Number::Pair(l, r) => l.magnitude() * 3 + r.magnitude() * 2,
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Number::Pair(Box::new(self), Box::new(rhs));
        output.reduce();
        output
    }
}

impl Sum for Number {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.reduce(|acc, cur| acc + cur).unwrap()
    }
}

impl From<&str> for Number {
    fn from(s: &str) -> Self {
        if s.chars().next().unwrap() == '[' {
            let mut depth = 0;
            let mut comma_index = 0;
            for (i, c) in s.chars().enumerate() {
                match c {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' => if depth == 1 {
                        comma_index = i;
                        break;
                    },
                    _ => (),
                }
            }
            Number::Pair(
                Box::new(Number::from(&s[1..comma_index])),
                Box::new(Number::from(&s[comma_index + 1..s.len() - 1])),
            )
        } else {
            Number::Single(s.parse().unwrap())
        }
    }
}

impl From<String> for Number {
    fn from(s: String) -> Self {
        Number::from(s.as_str())
    }
}

fn read_input() -> BufReader<File> {
    BufReader::new(File::open("input.txt").unwrap())
}

#[cfg(test)]
mod tests;
