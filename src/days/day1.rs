use crate::problem::Problem;
use itertools::Itertools;

pub struct DayOne {}

impl Problem for DayOne {
    fn part_one(&self) -> String {
        let res: Vec<usize> = include_str!("../data/day1.input")
            .split("\n\n")
            .map(|elf| {
                elf.split('\n')
                    .map(|food| food.parse::<usize>().unwrap_or(0))
                    .sum()
            })
            .sorted()
            .rev()
            .collect();
        format!("Day 1 part 1: {}", res[0])
    }

    fn part_two(&self) -> String {
        let res: Vec<usize> = include_str!("../data/day1.input")
            .split("\n\n")
            .map(|elf| {
                elf.split('\n')
                    .map(|food| food.parse::<usize>().unwrap_or(0))
                    .sum()
            })
            .sorted()
            .rev()
            .collect();
        format!("Day 1 part 2: {}", res.iter().take(3).sum::<usize>())
    }
}
