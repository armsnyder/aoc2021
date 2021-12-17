#![feature(test)]

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::packet::{Length, LengthType, Literal, PacketHeader, PacketType, ReadFrom};
use crate::reader::HexReader;

mod reader;
mod packet;

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    let mut reader = HexReader::from(reader);
    let mut version_sum = 0;
    let mut stack = vec![State::OUTER_PACKET];

    while let Some(state) = stack.last() {
        if match state {
            State::TerminalBitIndex(i) => *i <= reader.head(),
            State::RemainingPackets(n) => *n == 0,
        } {
            stack.pop();
            continue;
        }

        if let State::RemainingPackets(n) = state {
            let last = stack.len() - 1;
            stack[last] = State::RemainingPackets(n - 1);
        }

        let header = PacketHeader::read_from(&mut reader);

        version_sum += header.version.as_u32();

        match header.packet_type {
            PacketType::Literal => {
                Literal::read_from(&mut reader);
            }
            PacketType::Operation => {
                let length = Length::read_from(&mut reader);
                stack.push(State::new(length, reader.head()));
            }
        };
    }

    version_sum.to_string()
}

fn part2<R: BufRead>(reader: R) -> String {
    reader.lines()
        .map(Result::unwrap);

    return String::new();
}


enum State {
    TerminalBitIndex(u32),
    RemainingPackets(u32),
}

impl State {
    const OUTER_PACKET: State = State::RemainingPackets(1);

    fn new(length: Length, head: u32) -> Self {
        match length.length_type {
            LengthType::Bits => State::TerminalBitIndex(head + length.length),
            LengthType::Packets => State::RemainingPackets(length.length),
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
    use crate::packet::BitReader;

    use super::*;

    const BASIC: &[u8] = include_str!("testdata/basic.txt").as_bytes();

    #[test]
    fn test_part1() {
        assert_eq!(part1(BufReader::new("D2FE28".as_bytes())), "6");
        assert_eq!(part1(BufReader::new("38006F45291200".as_bytes())), "9");
        assert_eq!(part1(BufReader::new("8A004A801A8002F478".as_bytes())), "16");
        assert_eq!(part1(BufReader::new("620080001611562C8802118E34".as_bytes())), "12");
        assert_eq!(part1(BufReader::new("C0015000016115A2E0802F182340".as_bytes())), "23");
        assert_eq!(part1(BufReader::new("A0016C880162017C3686B18A3D4780".as_bytes())), "31");
    }

    #[test]
    fn test_hex_reader() {
        let mut reader = HexReader::from(BufReader::new("D2FE28".as_bytes()));
        assert_eq!(reader.read(3), Some(6));
        assert_eq!(reader.read(3), Some(4));
        assert_eq!(reader.read(3), Some(5));
        assert_eq!(reader.read(8), Some(252));
        assert_eq!(reader.read(4), Some(5));
        assert_eq!(reader.read(2), Some(0));
        assert_eq!(reader.read(1), Some(0));
        assert_eq!(reader.read(1), None);
        let mut reader = HexReader::from(BufReader::new("D2FE28".as_bytes()));
        assert_eq!(reader.read(20), Some(864226));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "")
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
