use crate::problem::Problem;
use itertools::Itertools;
use std::collections::HashSet;

pub struct DaySix {}

fn daysix(num_unique: usize) -> usize {
    let (res, _) = include_bytes!("../data/day6.input")
        .windows(num_unique)
        .find_position(|w| HashSet::<&u8>::from_iter(w.iter()).len() == num_unique)
        .unwrap();
    res + num_unique
}

impl Problem for DaySix {
    fn part_one(&self) -> String {
        format!("Day 3 part 1: {}", daysix(4))
    }

    fn part_two(&self) -> String {
        format!("Day 3 part 2: {}", daysix(14))
    }
}
