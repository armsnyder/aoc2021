#![feature(test)]

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    solve(reader, Strategy::Largest)
}

fn part2<R: BufRead>(reader: R) -> String {
    solve(reader, Strategy::Smallest)
}

fn solve<R: BufRead>(reader: R, strategy: Strategy) -> String {
    let steps = parse_input(reader);
    let mut inputs = vec![0; 14];

    if !solve_recurse(&steps, &mut inputs, 0, strategy) {
        panic!("no solution")
    }

    inputs.into_iter()
        .fold(0u64, |acc, digit| acc * 10 + digit as u64)
        .to_string()
}

fn solve_recurse(steps: &[Step], inputs: &mut [u8], z: i64, strategy: Strategy) -> bool {
    if steps.len() == 0 {
        z == 0
    } else if steps[0].trigger > 0 {
        strategy.inputs().any(|inp| {
            inputs[0] = inp;
            solve_recurse(&steps[1..], &mut inputs[1..], z * 26 + inp as i64 + steps[0].inc, strategy)
        })
    } else {
        let inp = z % 26 + steps[0].trigger;
        if inp < 1 || inp > 9 {
            false
        } else {
            inputs[0] = inp as u8;
            solve_recurse(&steps[1..], &mut inputs[1..], z / 26, strategy)
        }
    }
}

#[derive(Copy, Clone)]
enum Strategy {
    Largest,
    Smallest,
}

impl Strategy {
    const INPUTS_INC: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    const INPUTS_DEC: [u8; 9] = [9, 8, 7, 6, 5, 4, 3, 2, 1];

    fn inputs(self) -> std::array::IntoIter<u8, 9> {
        match self {
            Strategy::Largest => Self::INPUTS_DEC.into_iter(),
            Strategy::Smallest => Self::INPUTS_INC.into_iter(),
        }
    }
}

#[derive(Default, Clone, Copy)]
struct Step {
    trigger: i64,
    inc: i64,
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Step> {
    let mut steps = Vec::with_capacity(14);
    let mut cur = Step::default();
    let mut last_line_add_y_w = false;
    let mut last_line_div_z = false;

    for line in reader.lines().skip(1).map(Result::unwrap) {
        let extract_digit = || line.split_ascii_whitespace().nth(2).unwrap().parse().unwrap();

        if line.starts_with("inp ") {
            steps.push(cur);
            cur = Step::default();
        } else if last_line_div_z {
            cur.trigger = extract_digit();
        } else if last_line_add_y_w {
            cur.inc = extract_digit();
        }

        last_line_add_y_w = line == "add y w";
        last_line_div_z = line.starts_with("div z ");
    }

    steps.push(cur);

    steps
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
        assert_eq!(part1(BufReader::new(BASIC)), "99598963999971")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "93151411711211")
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
