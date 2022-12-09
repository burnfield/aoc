use crate::problem::Problem;
use itertools::Itertools;

pub struct DayTwo {}

fn day02_part1((a, x): (usize, usize)) -> usize {
    if a == x {
        x + 3
    } else if x == (1 + a % 3) {
        x + 6
    } else {
        x
    }
}

fn day02_part2((a, x): (usize, usize)) -> usize {
    let loose_value = if a - 1 == 0 { 3 } else { a - 1 };
    let win_value = if a + 1 == 4 { 1 } else { a + 1 };
    match x {
        1 => loose_value,
        2 => 3 + a,
        3 => 6 + win_value,
        _ => panic!(),
    }
}

impl Problem for DayTwo {
    fn part_one(&self) -> String {
        let res: usize = include_str!("../data/day2.input")
            .split('\n')
            .map(|gestures| -> usize {
                gestures
                    .split(' ')
                    .filter_map(|play| "ABCXYZ".find(play).map(|play| 1 + play % 3))
                    .tuples()
                    .map(day02_part1)
                    .sum::<usize>()
            })
            .sum();

        format!("Day 2 part 1: {}", res)
    }

    fn part_two(&self) -> String {
        let res: usize = include_str!("../data/day2.input")
            .split('\n')
            .map(|gestures| -> usize {
                gestures
                    .split(' ')
                    .filter_map(|play| "ABCXYZ".find(play).map(|play| 1 + play % 3))
                    .tuples()
                    .map(day02_part2)
                    .sum::<usize>()
            })
            .sum();

        format!("Day 2 part 2: {}", res)
    }
}
