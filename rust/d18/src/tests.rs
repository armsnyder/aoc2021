extern crate test;

use std::fs;
use test::Bencher;

use super::*;

const BASIC: &[u8] = include_str!("testdata/basic.txt").as_bytes();

#[test]
fn test_part1() {
    assert_eq!(part1(BufReader::new(BASIC)), "4140");
}

#[test]
fn test_part2() {
    assert_eq!(part2(BufReader::new(BASIC)), "3993");
}

#[test]
fn test_add_01() {
    assert_eq!(
        Number::from("[1,2]") + Number::from("[[3,4],5]"),
        Number::from("[[1,2],[[3,4],5]]"));
}

#[test]
fn test_explode_1() {
    let mut n = Number::from("[[[[[9,8],1],2],3],4]");
    assert_eq!(n.reduce_explode_once(), true);
    assert_eq!(n, Number::from("[[[[0,9],2],3],4]"));
}

#[test]
fn test_explode_2() {
    let mut n = Number::from("[7,[6,[5,[4,[3,2]]]]]");
    assert_eq!(n.reduce_explode_once(), true);
    assert_eq!(n, Number::from("[7,[6,[5,[7,0]]]]"));
}

#[test]
fn test_explode_3() {
    let mut n = Number::from("[[6,[5,[4,[3,2]]]],1]");
    assert_eq!(n.reduce_explode_once(), true);
    assert_eq!(n, Number::from("[[6,[5,[7,0]]],3]"));
}

#[test]
fn test_explode_4() {
    let mut n = Number::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
    assert_eq!(n.reduce_explode_once(), true);
    assert_eq!(n, Number::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
}

#[test]
fn test_explode_5() {
    let mut n = Number::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    assert_eq!(n.reduce_explode_once(), true);
    assert_eq!(n, Number::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
}

#[test]
fn test_split_1() {
    let mut n = Number::Single(10);
    assert_eq!(n.reduce_split_once(), true);
    assert_eq!(n, Number::from("[5,5]"));
}

#[test]
fn test_split_2() {
    let mut n = Number::Single(11);
    assert_eq!(n.reduce_split_once(), true);
    assert_eq!(n, Number::from("[5,6]"));
}

#[test]
fn test_split_3() {
    let mut n = Number::Single(12);
    assert_eq!(n.reduce_split_once(), true);
    assert_eq!(n, Number::from("[6,6]"));
}

#[test]
fn test_explode_6() {
    let mut n = Number::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
    assert_eq!(n.reduce_explode_once(), true);
    assert_eq!(n, Number::from("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"));
}

#[test]
fn test_explode_7() {
    let mut n = Number::from("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
    assert_eq!(n.reduce_explode_once(), true);
    assert_eq!(n, Number::from("[[[[0,7],4],[15,[0,13]]],[1,1]]"));
}

#[test]
fn test_split_4() {
    let mut n = Number::from("[[[[0,7],4],[15,[0,13]]],[1,1]]");
    assert_eq!(n.reduce_split_once(), true);
    assert_eq!(n, Number::from("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"));
}

#[test]
fn test_split_5() {
    let mut n = Number::from("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
    assert_eq!(n.reduce_split_once(), true);
    assert_eq!(n, Number::from("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"));
}

#[test]
fn test_explode_8() {
    let mut n = Number::from("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
    assert_eq!(n.reduce_explode_once(), true);
    assert_eq!(n, Number::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
}

#[test]
fn test_reduce() {
    let mut n = Number::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
    n.reduce();
    assert_eq!(n, Number::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
}

#[test]
fn test_add_02() {
    assert_eq!(
        Number::from("[[[[4,3],4],4],[7,[[8,4],9]]]") + Number::from("[1,1]"),
        Number::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
}

#[test]
fn test_sum_1() {
    let sum: Number = "[1,1]
[2,2]
[3,3]
[4,4]
".lines().map(Number::from).sum();
    assert_eq!(sum, Number::from("[[[[1,1],[2,2]],[3,3]],[4,4]]"));
}

#[test]
fn test_sum_2() {
    let sum: Number = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
".lines().map(Number::from).sum();
    assert_eq!(sum, Number::from("[[[[3,0],[5,3]],[4,4]],[5,5]]"));
}

#[test]
fn test_sum_3() {
    let sum: Number = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]
".lines().map(Number::from).sum();
    assert_eq!(sum, Number::from("[[[[5,0],[7,4]],[5,5]],[6,6]]"));
}

#[test]
fn test_sum_4() {
    let sum: Number = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
".lines().map(Number::from).sum();
    assert_eq!(sum, Number::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"));
}

#[test]
fn test_add_03() {
    assert_eq!(
        Number::from("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]") +
            Number::from("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"),
        Number::from("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"));
}

#[test]
fn test_add_04() {
    assert_eq!(
        Number::from("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]") +
            Number::from("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"),
        Number::from("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"));
}

#[test]
fn test_add_05() {
    assert_eq!(
        Number::from("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]") +
            Number::from("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"),
        Number::from("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"));
}

#[test]
fn test_add_06() {
    assert_eq!(
        Number::from("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]") +
            Number::from("[7,[5,[[3,8],[1,4]]]]"),
        Number::from("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]"));
}

#[test]
fn test_add_07() {
    assert_eq!(
        Number::from("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]") +
            Number::from("[[2,[2,2]],[8,[8,1]]]"),
        Number::from("[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]"));
}

#[test]
fn test_add_08() {
    assert_eq!(
        Number::from("[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]") +
            Number::from("[2,9]"),
        Number::from("[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]"));
}

#[test]
fn test_add_09() {
    assert_eq!(
        Number::from("[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]") +
            Number::from("[1,[[[9,3],9],[[9,0],[0,7]]]]"),
        Number::from("[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]"));
}

#[test]
fn test_add_10() {
    assert_eq!(
        Number::from("[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]") +
            Number::from("[[[5,[7,4]],7],1]"),
        Number::from("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]"));
}

#[test]
fn test_add_11() {
    assert_eq!(
        Number::from("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]") +
            Number::from("[[[[4,2],2],6],[8,7]]"),
        Number::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"));
}

#[test]
fn test_magnitude_1() {
    assert_eq!(Number::from("[[1,2],[[3,4],5]]").magnitude(), 143);
}

#[test]
fn test_magnitude_2() {
    assert_eq!(Number::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(), 1384);
}

#[test]
fn test_magnitude_3() {
    assert_eq!(Number::from("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(), 445);
}

#[test]
fn test_magnitude_4() {
    assert_eq!(Number::from("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(), 791);
}

#[test]
fn test_magnitude_5() {
    assert_eq!(Number::from("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(), 1137);
}

#[test]
fn test_magnitude_6() {
    assert_eq!(
        Number::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(),
        3488);
}

#[test]
fn test_sum_5() {
    let sum: Number = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
".lines().map(Number::from).sum();
    assert_eq!(
        sum,
        Number::from("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"));
}

#[test]
fn test_magnitude_7() {
    assert_eq!(
        Number::from("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]").magnitude(),
        4140);
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
