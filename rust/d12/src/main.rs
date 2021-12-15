#![feature(test)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

fn part1<R: BufRead>(reader: R) -> String {
    let graph = Graph::from(reader);
    count_paths(&graph, graph.start, &vec![false; graph.adjacency_list.len()]).to_string()
}

fn part2<R: BufRead>(reader: R) -> String {
    reader.lines()
        .map(Result::unwrap);

    return String::new();
}

fn count_paths(graph: &Graph, cur: usize, visited_small: &[bool]) -> u32 {
    let mut total = 0u32;

    for node in &graph.adjacency_list[cur] {
        let node = *node;

        if graph.smalls[node] {
            continue;
        }

        total += if graph.smalls[node] {
            let mut visited_small: &mut [bool] = visited_small.copy_from_slice();
            (*visited_small)[node] = true;
            count_paths(graph, node, &visited_small)
        } else {
            count_paths(graph, node, visited_small)
        };

        if node == graph.end {
            total += 1;
        }
    }

    total
}

struct Graph {
    adjacency_list: Vec<Vec<usize>>,
    smalls: Vec<bool>,
    start: usize,
    end: usize,
}

impl<R: BufRead> From<R> for Graph {
    fn from(reader: R) -> Self {
        let mut index_lookup: HashMap<String, usize> = HashMap::new();
        let mut adjacency_list = Vec::new();
        let mut smalls = Vec::new();

        let mut name_to_index = |name: &str, adjacency_list: &mut Vec<Vec<usize>>| -> usize {
            match index_lookup.get(name) {
                None => {
                    let index = adjacency_list.len();
                    index_lookup.insert(String::from(name), index);
                    adjacency_list.push(Vec::new());
                    smalls.push((name.chars().next().unwrap() as u8) >= ('a' as u8));
                    index
                }
                Some(v) => {
                    *v
                }
            }
        };

        let mut add = |a: &str, b: &str, adjacency_list: &mut Vec<Vec<usize>>| {
            let a_index = name_to_index(a, adjacency_list);
            let b_index = name_to_index(b, adjacency_list);
            adjacency_list[a_index].push(b_index);
            adjacency_list[b_index].push(a_index);
        };

        for line in reader.lines().map(Result::unwrap) {
            let mut split = line.split("-");
            add(split.next().unwrap(), split.next().unwrap(), &mut adjacency_list);
        }

        let start = name_to_index("start", &mut adjacency_list);
        let end = name_to_index("end", &mut adjacency_list);

        Graph { adjacency_list, smalls, start, end }
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

    const SMALL: &[u8] = include_str!("testdata/small.txt").as_bytes();
    const MEDIUM: &[u8] = include_str!("testdata/medium.txt").as_bytes();
    const LARGE: &[u8] = include_str!("testdata/large.txt").as_bytes();

    #[test]
    fn test_part1_small() {
        assert_eq!(part1(BufReader::new(SMALL)), "10")
    }

    #[test]
    fn test_part1_medium() {
        assert_eq!(part1(BufReader::new(MEDIUM)), "19")
    }

    #[test]
    fn test_part1_large() {
        assert_eq!(part1(BufReader::new(LARGE)), "226")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(BufReader::new(SMALL)), "")
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
