#![feature(test)]

use std::fs::File;
use std::io::{BufRead, BufReader};

use pathfinding::directed::astar::astar;

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    let start = State::from(reader);
    let (_, cost) = astar(&start, State::successors, State::heuristic, State::success).unwrap();
    cost.to_string()
}

fn part2<R: BufRead>(reader: R) -> String {
    let mut start = State::from(reader);
    start.unfold();
    let (_, cost) = astar(&start, State::successors, State::heuristic, State::success).unwrap();
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

impl Amphipod {
    fn cost(&self) -> u32 {
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

    fn successors(&self) -> Vec<(Self, u32)> {
        // If an amphipod can immediately move into a room, that is always the best move.

        if let Some(next) = (0..State::HALL_LEN).into_iter()
            .find_map(|hall| self.successor_from_hall(hall)) {
            return vec![next];
        }

        if let Some(next) = (0..State::ROOMS).into_iter()
            .find_map(|room| self.successor_from_room_to_room(room)) {
            return vec![next];
        }

        // Otherwise, explore moving amphipods into the hallway.

        (0..State::ROOMS).into_iter()
            .filter_map(|room| self.successors_from_room_to_hall(room))
            .flatten()
            .collect()
    }

    fn successor_from_hall(&self, hall: usize) -> Option<(Self, u32)> {
        match self.hall[hall] {
            None => None,
            Some(amphipod) => {
                let room = amphipod.own_room();

                if !self.is_room_available(room, amphipod) {
                    None
                } else if self.blocked_hall_to_room(hall, room) {
                    None
                } else {
                    let slot = self.get_empty_slot(room);

                    let mut next = self.clone();
                    next.rooms[room][slot] = next.hall[hall];
                    next.hall[hall] = None;

                    Some((next, State::dist_hall_to_room(hall, room, slot) * amphipod.cost()))
                }
            }
        }
    }

    fn successor_from_room_to_room(&self, from_room: usize) -> Option<(Self, u32)> {
        if self.rooms[from_room][0..self.slot_size].iter().all(|s| match s {
            None => true,
            Some(amphipod) => amphipod.own_room() == from_room
        }) {
            None
        } else {
            let (from_slot, amphipod) = self.get_movable_amphipod_in_room(from_room);

            let to_room = amphipod.own_room();

            if !self.is_room_available(to_room, amphipod) {
                return None;
            }

            if self.blocked_room_to_room(from_room, to_room) {
                None
            } else {
                let to_slot = self.get_empty_slot(to_room);

                let mut next = self.clone();
                next.rooms[to_room][to_slot] = next.rooms[from_room][from_slot];
                next.rooms[from_room][from_slot] = None;

                Some((next, State::dist_room_to_room(from_room, from_slot, to_room, to_slot) * amphipod.cost()))
            }
        }
    }

    fn successors_from_room_to_hall(&self, room: usize) -> Option<Vec<(Self, u32)>> {
        if self.rooms[room][0..self.slot_size].iter().all(|s| match s {
            None => true,
            Some(amphipod) => amphipod.own_room() == room
        }) {
            None
        } else {
            let (slot, amphipod) = self.get_movable_amphipod_in_room(room);

            Some((0..State::HALL_LEN).into_iter()
                .filter_map(|hall| if self.blocked_room_to_hall(room, hall) {
                    None
                } else {
                    let mut next = self.clone();
                    next.hall[hall] = next.rooms[room][slot];
                    next.rooms[room][slot] = None;

                    Some((next, State::dist_room_to_hall(room, slot, hall) * amphipod.cost()))
                })
                .collect::<Vec<(Self, u32)>>())
        }
    }

    fn get_empty_slot(&self, room: usize) -> usize {
        let (slot, _) = self.rooms[room][0..self.slot_size].iter()
            .enumerate()
            .rev()
            .find(|(_, v)| v.is_none())
            .unwrap();
        slot
    }

    fn is_room_available(&self, room: usize, amphipod: Amphipod) -> bool {
        self.rooms[room][0..self.slot_size].iter().all(|room| match room {
            None => true,
            Some(other_amphipod) => *other_amphipod == amphipod,
        })
    }

    fn get_movable_amphipod_in_room(&self, room: usize) -> (usize, Amphipod) {
        self.rooms[room][0..self.slot_size].iter()
            .enumerate()
            .find_map(|(slot, value)| value.map(|amphipod| (slot, amphipod)))
            .unwrap()
    }

    fn heuristic(&self) -> u32 {
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

    fn dist_hall_to_room(hall: usize, room: usize, slot: usize) -> u32 {
        let diff = hall as isize - room as isize;
        let mut dist = if diff > 2 {
            hall - room - 2
        } else if diff < 1 {
            room - hall + 1
        } else {
            0
        } as u32;
        dist *= 2;
        if hall == 0 || hall == 6 {
            dist -= 1
        }
        dist + 2 + slot as u32
    }

    fn dist_room_to_hall(room: usize, slot: usize, hall: usize) -> u32 {
        State::dist_hall_to_room(hall, room, slot)
    }

    fn dist_room_to_room(from_room: usize, from_slot: usize, to_room: usize, to_slot: usize) -> u32 {
        (if from_room > to_room {
            from_room - to_room
        } else {
            to_room - from_room
        }) as u32 * 2 + 2 + from_slot as u32 + to_slot as u32
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

    // #[bench]
    // fn bench_part2(b: &mut Bencher) {
    //     let input = fs::read_to_string("input.txt").unwrap();
    //     let input = input.as_bytes();
    //     b.iter(|| part2(BufReader::new(input)))
    // }
}
