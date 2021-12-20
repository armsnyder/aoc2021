#![feature(test)]

#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::ops::Mul;

use crate::transform::{ALL_ORIENTATIONS, Coord, Matrix4, Vector3};

mod transform;

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    let regions = read_regions(reader);
    let region_transforms = get_region_transforms(&regions);
    count_beacons(regions, region_transforms).to_string()
}

fn part2<R: BufRead>(reader: R) -> String {
    let regions = read_regions(reader);
    let region_transforms = get_region_transforms(&regions);
    get_largest_manhattan_distance_between_scanners(region_transforms).to_string()
}

fn get_region_transforms(regions: &Vec<Region>) -> Vec<Matrix4> {
    let mut region_transforms = vec![Matrix4::IDENTITY; regions.len()];
    let mut visited = vec![false; regions.len()];
    let mut queue = vec![(0, Matrix4::IDENTITY)];

    while let Some((base_index, base_transform)) = queue.pop() {
        visited[base_index] = true;
        region_transforms[base_index] = base_transform;

        for i in 0..regions.len() {
            if i != base_index && !visited[i] {
                if let Some(transform) = regions[i].get_transform_relative_to(&regions[base_index]) {
                    queue.push((i, base_transform * transform));
                }
            }
        }
    }

    region_transforms
}

fn count_beacons(regions: Vec<Region>, region_transforms: Vec<Matrix4>) -> usize {
    regions.into_iter()
        .enumerate()
        .map(|(i, region)| region * region_transforms[i])
        .flat_map(|r| r.beacons)
        .collect::<HashSet<Vector3>>()
        .len()
}

fn get_largest_manhattan_distance_between_scanners(region_transforms: Vec<Matrix4>) -> Coord {
    let translations = region_transforms.into_iter()
        .map(Matrix4::translation)
        .collect::<Vec<Vector3>>();

    translations
        .iter()
        .enumerate()
        .flat_map(|(i, &a)|
            translations[i + 1..]
                .iter()
                .map(move |&b| b - a)
                .map(Vector3::manhattan_distance)
        )
        .max()
        .unwrap()
}

fn read_regions<R: BufRead>(reader: R) -> Vec<Region> {
    let mut regions = Vec::new();
    let lines = &mut reader.lines();
    while let Some(region) = next_region(lines) {
        regions.push(region);
    }
    regions
}

fn next_region<R: BufRead>(lines: &mut Lines<R>) -> Option<Region> {
    match lines.next() {
        None => None,
        Some(_header) => Some(
            lines
                .map(Result::unwrap)
                .take_while(|line| !line.is_empty())
                .map(Vector3::from)
                .collect::<Region>()
        )
    }
}

#[derive(Default)]
struct Region {
    // The beacon positions:
    beacons: Vec<Vector3>,
    // Markers for finding overlapping regions (distances between every combination of 2 beacons):
    distance_markers: HashSet<Coord>,
    // Markers for orienting regions (vectors for every combination of 2 beacons):
    orientation_markers: HashMap<Vector3, Vector3>,
}

impl Region {
    const REQUIRED_MARKERS_FOR_OVERLAP: usize = 66; // nCr(12, 2)

    fn insert(&mut self, beacon: Vector3) {
        for cur_beacon in &self.beacons {
            let orientation_marker = *cur_beacon - beacon;
            self.orientation_markers.insert(orientation_marker, *cur_beacon);
            self.orientation_markers.insert(-orientation_marker, beacon);
            self.distance_markers.insert(orientation_marker.manhattan_distance());
        }
        self.beacons.push(beacon);
    }

    fn get_transform_relative_to(&self, other: &Self) -> Option<Matrix4> {
        if self.distance_markers.intersection(&other.distance_markers).count() < Self::REQUIRED_MARKERS_FOR_OVERLAP {
            None
        } else {
            for &try_rotation in ALL_ORIENTATIONS.iter() {
                let overlapping_orientation_markers = self.orientation_markers.iter()
                    .map(|(&k, &v)| (try_rotation * k, try_rotation * v))
                    .filter(|(k, _)| other.orientation_markers.contains_key(k))
                    .collect::<HashMap<Vector3, Vector3>>();

                if overlapping_orientation_markers.len() >= Self::REQUIRED_MARKERS_FOR_OVERLAP {
                    let translation = overlapping_orientation_markers
                        .into_iter()
                        .next()
                        .map(|(k, v)| other.orientation_markers[&k] - v)
                        .unwrap();

                    return Some(try_rotation + translation);
                }
            }

            unreachable!()
        }
    }
}

impl Mul<Matrix4> for Region {
    type Output = Self;

    fn mul(self, rhs: Matrix4) -> Self {
        Region {
            beacons: self.beacons.iter().map(|&b| rhs * b).collect(),
            distance_markers: self.distance_markers,
            orientation_markers: self.orientation_markers.iter().map(|(&k, &v)| (rhs * k, rhs * v)).collect(),
        }
    }
}

impl FromIterator<Vector3> for Region {
    fn from_iter<T: IntoIterator<Item=Vector3>>(iter: T) -> Self {
        let mut region = Region::default();
        iter.into_iter().for_each(|v| region.insert(v));
        region
    }
}

fn read_input() -> BufReader<File> {
    BufReader::new(File::open("input.txt").unwrap())
}

#[cfg(test)]
mod tests;
