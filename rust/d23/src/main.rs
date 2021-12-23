#![feature(test)]

use std::fmt::{Display, Formatter, Write};
use std::fs::File;
use std::io::{BufRead, BufReader};

use pathfinding::directed::astar::astar;

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    let start = State::from(reader);
    let (path, cost) = astar(&start, State::successors, State::heuristic, State::success).unwrap();
    for step in path {
        println!("{}", step);
        println!()
    }
    cost.to_string()
}

fn part2<R: BufRead>(reader: R) -> String {
    let mut start = State::from(reader);
    start.unfold();
    let (path, cost) = astar(&start, State::successors, State::heuristic, State::success).unwrap();
    for step in path {
        println!("{}", step);
        println!()
    }
    cost.to_string()
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Amphipod { A, B, C, D }

impl From<char> for Amphipod {
    fn from(c: char) -> Self {
        match c {
            'A' => Amphipod::A,
            'B' => Amphipod::B,
            'C' => Amphipod::C,
            'D' => Amphipod::D,
            _ => unreachable!(),
        }
    }
}

impl Into<char> for Amphipod {
    fn into(self) -> char {
        match self {
            Amphipod::A => 'A',
            Amphipod::B => 'B',
            Amphipod::C => 'C',
            Amphipod::D => 'D',
        }
    }
}

impl Amphipod {
    fn cost(&self) -> usize {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }

    fn own_room(&self) -> usize {
        match self {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3,
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct State {
    rooms: [[Option<Amphipod>; State::MAX_SLOTS]; State::ROOMS],
    hall: [Option<Amphipod>; State::HALL_LEN],
    slot_size: usize,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("#############\n#")?;
        f.write_char(self.hall[0].map(Amphipod::into).unwrap_or('.'))?;
        f.write_char(self.hall[1].map(Amphipod::into).unwrap_or('.'))?;
        f.write_char('.')?;
        f.write_char(self.hall[2].map(Amphipod::into).unwrap_or('.'))?;
        f.write_char('.')?;
        f.write_char(self.hall[3].map(Amphipod::into).unwrap_or('.'))?;
        f.write_char('.')?;
        f.write_char(self.hall[4].map(Amphipod::into).unwrap_or('.'))?;
        f.write_char('.')?;
        f.write_char(self.hall[5].map(Amphipod::into).unwrap_or('.'))?;
        f.write_char(self.hall[6].map(Amphipod::into).unwrap_or('.'))?;
        f.write_str("#\n###")?;
        f.write_char(self.rooms[0][0].map(Amphipod::into).unwrap_or('.'))?;
        f.write_char('#')?;
        f.write_char(self.rooms[1][0].map(Amphipod::into).unwrap_or('.'))?;
        f.write_char('#')?;
        f.write_char(self.rooms[2][0].map(Amphipod::into).unwrap_or('.'))?;
        f.write_char('#')?;
        f.write_char(self.rooms[3][0].map(Amphipod::into).unwrap_or('.'))?;
        f.write_str("###\n")?;
        for slot in 1..self.slot_size {
            f.write_str("  #")?;
            for room in 0..State::ROOMS {
                f.write_char(self.rooms[room][slot].map(Amphipod::into).unwrap_or('.'))?;
                f.write_char('#')?;
            }
            f.write_str("  \n")?;
        }
        f.write_str("  #########  ")
    }
}

impl State {
    const ROOMS: usize = 4;
    const MAX_SLOTS: usize = 4;
    const HALL_LEN: usize = 7;

    fn unfold(&mut self) {
        self.slot_size = 4;

        for room in 0..State::ROOMS {
            self.rooms[room][3] = self.rooms[room][1]
        }

        self.rooms[0][1] = Some(Amphipod::D);
        self.rooms[0][2] = Some(Amphipod::D);
        self.rooms[1][1] = Some(Amphipod::C);
        self.rooms[1][2] = Some(Amphipod::B);
        self.rooms[2][1] = Some(Amphipod::B);
        self.rooms[2][2] = Some(Amphipod::A);
        self.rooms[3][1] = Some(Amphipod::A);
        self.rooms[3][2] = Some(Amphipod::C);
    }

    fn successors(&self) -> Vec<(Self, usize)> {
        for from_hall in 0..State::HALL_LEN {
            if let Some(next) = self.successor_from_hall(from_hall) {
                return vec![next];
            }
        }

        for from_room in 0..State::ROOMS {
            if let Some(next) = self.successor_from_room_direct(from_room) {
                return vec![next];
            }
        }

        (0..State::ROOMS).into_iter()
            .filter_map(|room| self.successors_from_room(room))
            .flatten()
            .collect()
    }

    fn successor_from_hall(&self, hall: usize) -> Option<(Self, usize)> {
        match self.hall[hall] {
            None => None,
            Some(amphipod) => {
                let room = amphipod.own_room();

                if self.rooms[room][0..self.slot_size].iter().any(|room| match room {
                    None => false,
                    Some(other_amphipod) => *other_amphipod != amphipod,
                }) {
                    return None;
                }

                if self.blocked_hall_to_room(hall, room) {
                    None
                } else {
                    let (slot, _) = self.rooms[room][0..self.slot_size].iter()
                        .enumerate()
                        .rev()
                        .find(|(_, v)| v.is_none())
                        .unwrap();
                    let mut next = self.clone();
                    State::move_amphipod(&mut next.hall[hall], &mut next.rooms[room][slot]);
                    Some((next, State::dist_hall_to_room(hall, room, slot) * amphipod.cost()))
                }
            }
        }
    }

    fn successor_from_room_direct(&self, from_room: usize) -> Option<(Self, usize)> {
        if self.rooms[from_room][0..self.slot_size].iter().all(|s| match s {
            None => true,
            Some(amphipod) => amphipod.own_room() == from_room
        }) {
            None
        } else {
            let (from_slot, amphipod) = self.rooms[from_room][0..self.slot_size].iter()
                .enumerate()
                .find_map(|(slot, value)| value.map(|amphipod| (slot, amphipod)))
                .unwrap();

            let to_room = amphipod.own_room();

            if self.rooms[to_room][0..self.slot_size].iter().any(|room| match room {
                None => false,
                Some(other_amphipod) => *other_amphipod != amphipod,
            }) {
                return None;
            }

            if self.blocked_room_to_room(from_room, to_room) {
                None
            } else {
                let (to_slot, _) = self.rooms[to_room][0..self.slot_size].iter()
                    .enumerate()
                    .rev()
                    .find(|(_, v)| v.is_none())
                    .unwrap();
                let mut next = self.clone();
                next.rooms[to_room][to_slot] = next.rooms[from_room][from_slot];
                next.rooms[from_room][from_slot] = None;
                Some((next, State::dist_room_to_room(from_room, from_slot, to_room, to_slot) * amphipod.cost()))
            }
        }
    }

    fn successors_from_room(&self, room: usize) -> Option<Vec<(Self, usize)>> {
        if self.rooms[room][0..self.slot_size].iter().all(|s| match s {
            None => true,
            Some(amphipod) => amphipod.own_room() == room
        }) {
            None
        } else {
            let (slot, amphipod) = self.rooms[room][0..self.slot_size].iter()
                .enumerate()
                .find_map(|(slot, value)| value.map(|amphipod| (slot, amphipod)))
                .unwrap();

            Some((0..State::HALL_LEN).into_iter()
                .filter_map(|hall| if self.blocked_room_to_hall(room, hall) {
                    None
                } else {
                    let mut next = self.clone();
                    State::move_amphipod(&mut next.rooms[room][slot], &mut next.hall[hall]);
                    Some((next, State::dist_room_to_hall(room, slot, hall) * amphipod.cost()))
                })
                .collect::<Vec<(Self, usize)>>())
        }
    }

    fn heuristic(&self) -> usize {
        let mut total = 0;

        for room in 0..State::ROOMS {
            for slot in 0..self.slot_size {
                if let Some(amphipod) = self.rooms[room][slot] {
                    let own_room = amphipod.own_room();
                    if room != own_room {
                        total += State::dist_room_to_room(room, slot, own_room, 0) * amphipod.cost();
                    }
                }
            }
        }

        for hall in 0..State::HALL_LEN {
            if let Some(amphipod) = self.hall[hall] {
                total += State::dist_hall_to_room(hall, amphipod.own_room(), 0) * amphipod.cost();
            }
        }

        total
    }

    fn success(&self) -> bool {
        (0..State::ROOMS).into_iter()
            .all(|room| self.rooms[room][0..self.slot_size].iter()
                .all(|s| match s {
                    None => false,
                    Some(amphipod) => amphipod.own_room() == room
                }))
    }

    fn move_amphipod(from: &mut Option<Amphipod>, to: &mut Option<Amphipod>) {
        *to = *from;
        *from = None;
    }

    fn dist_hall_to_room(hall: usize, room: usize, slot: usize) -> usize {
        let diff = hall as isize - room as isize;
        let mut dist = if diff > 2 {
            hall - room + 2
        } else if diff < 1 {
            room - hall + 1
        } else {
            0
        };
        dist *= 2;
        if hall == 0 || hall == 6 {
            dist -= 1
        }
        dist + 2 + slot
    }

    fn dist_room_to_hall(room: usize, slot: usize, hall: usize) -> usize {
        State::dist_hall_to_room(hall, room, slot)
    }

    fn dist_room_to_room(from_room: usize, from_slot: usize, to_room: usize, to_slot: usize) -> usize {
        (if from_room > to_room {
            from_room - to_room
        } else {
            to_room - from_room
        }) * 2 + 2 + from_slot + to_slot
    }

    fn blocked_hall_to_room(&self, hall: usize, room: usize) -> bool {
        let diff = hall as isize - room as isize;
        if diff > 2 {
            self.hall[room + 2..hall].iter().any(Option::is_some)
        } else if diff < 1 {
            self.hall[hall + 1..=(room + 1)].iter().any(Option::is_some)
        } else {
            false
        }
    }

    fn blocked_room_to_hall(&self, room: usize, hall: usize) -> bool {
        let diff = hall as isize - room as isize;
        if diff > 2 {
            self.hall[room + 2..=hall].iter().any(Option::is_some)
        } else if diff < 1 {
            self.hall[hall..=(room + 1)].iter().any(Option::is_some)
        } else {
            false
        }
    }

    fn blocked_room_to_room(&self, from_room: usize, to_room: usize) -> bool {
        if from_room > to_room {
            self.blocked_room_to_room(to_room, from_room)
        } else {
            (from_room + 2..to_room + 2).into_iter().any(|hall| self.hall[hall].is_some())
        }
    }
}

impl<R: BufRead> From<R> for State {
    fn from(reader: R) -> Self {
        let mut lines = reader.lines();
        lines.next();
        lines.next();
        let mut rooms = [[None; State::MAX_SLOTS]; State::ROOMS];
        for slot in 0..2 {
            let chars = lines.next().unwrap().unwrap().chars().collect::<Vec<char>>();
            for room in 0..State::ROOMS {
                rooms[room][slot] = Some(Amphipod::from(chars[room * 2 + 3]));
            }
        }
        State {
            hall: [None; State::HALL_LEN],
            rooms,
            slot_size: 2,
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
        assert_eq!(part1(BufReader::new(BASIC)), "12521")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(BASIC)), "44169")
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

// 71379 too high
