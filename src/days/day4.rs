use crate::problem::Problem;
use itertools::Itertools;

pub struct DayFour {}

fn day4_parser() -> Vec<usize> {
    include_str!("../data/day4.input")
        .split(['\n', ',', '-'])
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<usize>>()
}

impl Problem for DayFour {
    fn part_one(&self) -> String {
        let res: usize = day4_parser()
            .iter()
            .tuples()
            .map(|(s0, e0, s1, e1)| {
                let r0_in_r1 = s0 >= s1 && e0 <= e1;
                let r1_in_r0 = s1 >= s0 && e1 <= e0;
                usize::from(r0_in_r1 || r1_in_r0)
            })
            .sum();
        format!("Day 3 part 1: {}", res)
    }

    fn part_two(&self) -> String {
        let res: usize = day4_parser()
            .iter()
            .tuples()
            .map(|(s0, e0, s1, e1)| usize::from(e0 >= s1 && e1 >= s0))
            .sum();
        format!("Day 3 part 2: {}", res)
    }
}
