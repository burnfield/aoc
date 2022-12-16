use itertools::Itertools;
use std::collections::HashSet;

use crate::problem::Problem;

pub struct DayNine {}

type Point = (i32, i32);

fn step_head(h: &mut Point, dir: &str) {
    match dir {
        "R" => h.0 += 1,
        "L" => h.0 -= 1,
        "U" => h.1 += 1,
        "D" => h.1 -= 1,
        _ => unreachable!(),
    }
}

fn solution<const ROPE_SIZE: usize>() -> usize {
    let lines = parse();

    let rope: &mut [Point] = &mut [(0, 0); ROPE_SIZE];

    HashSet::<Point>::from_iter(lines.iter().map(|dir| step_rope::<ROPE_SIZE>(dir, rope))).len()
}

fn step_rope<const ROPE_SIZE: usize>(dir: &str, rope: &mut [Point]) -> Point {
    step_head(&mut rope[0], dir);
    // Todo: mutable sliding window??
    let mut p = rope[0];
    for c in rope.iter_mut().skip(1) {
        let xdiff = p.0 - c.0;
        let ydiff = p.1 - c.1;
        if xdiff.abs() != 2 && ydiff.abs() != 2 {
            break;
        }
        c.0 += xdiff.signum();
        c.1 += ydiff.signum();
        p = *c;
    }
    rope[ROPE_SIZE - 1]
}

fn parse() -> Vec<&'static str> {
    include_str!("../data/day9.input")
        .lines()
        .map(str::split_whitespace)
        .filter_map(Itertools::collect_tuple)
        .flat_map(|(dir, x)| vec![dir; x.parse::<usize>().unwrap()])
        .collect()
}

impl Problem for DayNine {
    fn part_one(&self) -> String {
        let res = solution::<2>();

        assert!(res == 6190);

        format!("Day 7 part 1: {}", res)
    }

    fn part_two(&self) -> String {
        let res = solution::<10>();

        assert!(res == 2516);

        format!("Day 7 part 2: {}", res)
    }
}
