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
    let PacketResult { version, .. } = evaluate_packet(&mut reader);
    version.to_string()
}

fn part2<R: BufRead>(reader: R) -> String {
    let mut reader = HexReader::from(reader);
    let PacketResult { literal, .. } = evaluate_packet(&mut reader);
    literal.to_string()
}

fn evaluate_packet(reader: &mut HexReader) -> PacketResult {
    let PacketHeader { version, packet_type } = PacketHeader::read_from(reader);

    match packet_type {
        PacketType::Literal => {
            let literal: u64 = Literal::read_from(reader).into();
            let version: u32 = version.into();
            PacketResult { version, literal }
        }

        operation => apply_operation(reader, version.into(), match operation {
            PacketType::Sum => |values: Vec<u64>| values.into_iter().sum(),
            PacketType::Product => |values: Vec<u64>| values.into_iter().product(),
            PacketType::Minimum => |values: Vec<u64>| values.into_iter().min().unwrap(),
            PacketType::Maximum => |values: Vec<u64>| values.into_iter().max().unwrap(),
            PacketType::GreaterThan => |values: Vec<u64>| if values[0] > values[1] { 1 } else { 0 },
            PacketType::LessThan => |values: Vec<u64>| if values[0] < values[1] { 1 } else { 0 },
            PacketType::EqualTo => |values: Vec<u64>| if values[0] == values[1] { 1 } else { 0 },
            _ => unreachable!()
        }),
    }
}

fn apply_operation(reader: &mut HexReader, version: u32, op: impl Fn(Vec<u64>) -> u64) -> PacketResult {
    let Length { length_type, length } = Length::read_from(reader);

    let sub_results = match length_type {
        LengthType::Packets => (0..length).map(|_| evaluate_packet(reader)).collect::<Vec<PacketResult>>(),
        LengthType::Bits => {
            let tail = reader.head() + length;
            let mut sub_results = Vec::new();
            while reader.head() < tail {
                sub_results.push(evaluate_packet(reader));
            }
            sub_results
        }
    };

    let values = sub_results.iter().map(|r| r.literal).collect::<Vec<u64>>();
    let literal = op(values);
    let version = version + sub_results.iter().map(|r| r.version).sum::<u32>();

    PacketResult { literal, version }
}

struct PacketResult {
    version: u32,
    literal: u64,
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
    fn test_part2() {
        assert_eq!(part2(BufReader::new("C200B40A82".as_bytes())), "3");
        assert_eq!(part2(BufReader::new("04005AC33890".as_bytes())), "54");
        assert_eq!(part2(BufReader::new("880086C3E88112".as_bytes())), "7");
        assert_eq!(part2(BufReader::new("CE00C43D881120".as_bytes())), "9");
        assert_eq!(part2(BufReader::new("D8005AC2A8F0".as_bytes())), "1");
        assert_eq!(part2(BufReader::new("F600BC2D8F".as_bytes())), "0");
        assert_eq!(part2(BufReader::new("9C005AC2F8F0".as_bytes())), "0");
        assert_eq!(part2(BufReader::new("9C0141080250320F1802104A08".as_bytes())), "1");
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
